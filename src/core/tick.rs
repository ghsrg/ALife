use crate::core::accounting::{IntegratedAccountingSnapshot, MatterAccountingDelta};
use crate::core::cell_store::{CellIndex, EnergyBuffer, LifecycleState, RuntimeFlags};
use crate::core::config::RuntimeConfig;
use crate::core::deltas::CommitSummary;
use crate::core::events::EventKind;
use crate::core::materials::MaterialSlot;
use crate::core::process::{ActionCandidate, FeasibilityResult, ProcessId};
use crate::core::resources::ResourceLayerIndex;
use crate::core::summary::{
    CollapseReason, MetricsSummary, ProcessDiagnostics, RunSummary, SurvivalResult,
};
use crate::core::units::{
    EnergyAmount, HeatAmount, MaterialAmount, Position, ResourceAmount, WasteAmount, WorldSize,
};
use crate::core::world::{WorldInitError, WorldState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TickError {
    WorldInit(WorldInitError),
}

impl From<WorldInitError> for TickError {
    fn from(value: WorldInitError) -> Self {
        Self::WorldInit(value)
    }
}

pub struct TickExecutor {
    world: WorldState,
}

#[derive(Clone, Copy, Debug, Default)]
struct Phase2GMetricsDelta {
    reaction_matched_count: u32,
    reaction_executed_count: u32,
    reaction_rejected_count: u32,
    reaction_input_amount: f32,
    reaction_output_amount: f32,
    reaction_heat_generated: f32,
    reaction_energy_output: f32,
    reaction_accounting_error: f32,
    resource_diffused_amount: f32,
    resource_decay_amount: f32,
    fragment_created_amount: f32,
    fragment_converted_amount: f32,
    material_degradation_amount: f32,
    boundary_leakage_amount: f32,
}

#[derive(Clone, Copy, Debug, Default)]
struct Phase2HMetricsDelta {
    joint_created_count: u32,
    joint_creation_rejected_count: u32,
    joint_broken_count: u32,
    joint_resource_transfer_amount: f32,
    joint_signal_generated_total: f32,
    joint_signal_readable_total: f32,
    joint_heat_transfer_amount: f32,
    joint_degradation_amount: f32,
    joint_mechanical_correction_amount: f32,
}

impl Phase2GMetricsDelta {
    fn add(&mut self, other: Self) {
        self.reaction_matched_count += other.reaction_matched_count;
        self.reaction_executed_count += other.reaction_executed_count;
        self.reaction_rejected_count += other.reaction_rejected_count;
        self.reaction_input_amount += other.reaction_input_amount;
        self.reaction_output_amount += other.reaction_output_amount;
        self.reaction_heat_generated += other.reaction_heat_generated;
        self.reaction_energy_output += other.reaction_energy_output;
        self.reaction_accounting_error += other.reaction_accounting_error;
        self.resource_diffused_amount += other.resource_diffused_amount;
        self.resource_decay_amount += other.resource_decay_amount;
        self.fragment_created_amount += other.fragment_created_amount;
        self.fragment_converted_amount += other.fragment_converted_amount;
        self.material_degradation_amount += other.material_degradation_amount;
        self.boundary_leakage_amount += other.boundary_leakage_amount;
    }
}

impl TickExecutor {
    pub fn new(config: RuntimeConfig) -> Result<Self, TickError> {
        Ok(Self {
            world: WorldState::from_config(config)?,
        })
    }

    pub fn world(&self) -> &WorldState {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut WorldState {
        &mut self.world
    }

    pub fn step(&mut self) -> Result<RunSummary, TickError> {
        let config = self.world.config().clone();
        let len = self.world.cells().len();
        let accounting_before = IntegratedAccountingSnapshot::from_world(&self.world);

        // Rebuild Spatial Index at the start of tick
        self.world.rebuild_spatial_index();
        self.world.rebuild_contact_cache();
        if config.joints.enabled {
            self.world
                .joints_mut_for_commit()
                .begin_tick_signal_rollover(config.joints.signal_decay);
            let dead_endpoints = (0..self.world.cells().len())
                .map(CellIndex::from_raw)
                .filter(|idx| self.world.cells().lifecycle_state(*idx) == LifecycleState::Dead)
                .collect::<Vec<_>>();
            for endpoint in dead_endpoints {
                self.world
                    .joints_mut_for_commit()
                    .make_inert_for_endpoint(endpoint);
            }
        }

        let mut phase2g_metrics = self.commit_passive_chemistry_reactions(&config);
        phase2g_metrics.add(self.commit_controlled_chemistry_reactions(&config));
        phase2g_metrics.material_degradation_amount +=
            self.commit_local_heat_material_degradation(&config);
        phase2g_metrics.material_degradation_amount += self.commit_material_type_decay(&config);

        let contact_pairs_count = self.world.contact_cache().pairs().len() as u32;
        let contact_pressure_pre_total = self.world.contact_cache().total_overlap();
        let mut contact_pressure_max_over_tick = self.world.contact_cache().max_overlap();
        let contact_stimulus_readable_total_for_summary = (0..self.world.cells().len())
            .map(|i| self.world.cells().contact_stimulus(CellIndex::from_raw(i)))
            .sum::<f32>();
        let mut contact_exchange_amount = 0.0_f32;
        let mut contact_exchange_pairs_count = 0_u32;
        let mut contact_exchange_rejections_no_capability = 0_u32;
        let mut phase2h_metrics = Phase2HMetricsDelta::default();
        if config.joints.enabled {
            let tick = self.world.tick();
            for joint_id in self.world.joints().active_ids().collect::<Vec<_>>() {
                phase2h_metrics.joint_signal_readable_total += self
                    .world
                    .joints()
                    .readable_signal(joint_id, tick)
                    .unwrap_or(0.0);
            }
        }

        if config.joints.enabled {
            let pairs = self.world.contact_cache().pairs().to_vec();
            if pairs.is_empty() && self.world.cells().len() >= 2 {
                phase2h_metrics.joint_creation_rejected_count += 1;
            }
            for pair in pairs {
                let Some(endpoints) = crate::core::joints::JointEndpoints::new(pair.a, pair.b)
                else {
                    phase2h_metrics.joint_creation_rejected_count += 1;
                    continue;
                };
                if self.world.joints().has_active_between(endpoints) {
                    phase2h_metrics.joint_creation_rejected_count += 1;
                    continue;
                }
                let cost_each = config.joints.creation_material_cost.raw() * 0.5;
                let resource_cost_each = config.joints.creation_resource_cost.raw() * 0.5;
                let energy_cost_each = config.joints.creation_energy_cost.raw() * 0.5;
                let a_structural = self.world.cells().structural_material(pair.a).raw();
                let b_structural = self.world.cells().structural_material(pair.b).raw();
                let a_resource = self.world.cells().resource_amount(pair.a).raw();
                let b_resource = self.world.cells().resource_amount(pair.b).raw();
                let a_energy = self.world.cells().energy(pair.a);
                let b_energy = self.world.cells().energy(pair.b);
                if a_structural < cost_each
                    || b_structural < cost_each
                    || a_resource < resource_cost_each
                    || b_resource < resource_cost_each
                    || a_energy.current().raw() < energy_cost_each
                    || b_energy.current().raw() < energy_cost_each
                {
                    phase2h_metrics.joint_creation_rejected_count += 1;
                    continue;
                }
                {
                    let cells = self.world.cells_mut_for_commit();
                    cells.set_structural_material(
                        pair.a,
                        MaterialAmount::new_unchecked((a_structural - cost_each).max(0.0)),
                    );
                    cells.set_structural_material(
                        pair.b,
                        MaterialAmount::new_unchecked((b_structural - cost_each).max(0.0)),
                    );
                    cells.set_resources(
                        pair.a,
                        ResourceAmount::new_unchecked((a_resource - resource_cost_each).max(0.0)),
                    );
                    cells.set_resources(
                        pair.b,
                        ResourceAmount::new_unchecked((b_resource - resource_cost_each).max(0.0)),
                    );
                    cells.set_energy(
                        pair.a,
                        EnergyBuffer::new(
                            EnergyAmount::new_unchecked(
                                (a_energy.current().raw() - energy_cost_each).max(0.0),
                            ),
                            a_energy.capacity(),
                        ),
                    );
                    cells.set_energy(
                        pair.b,
                        EnergyBuffer::new(
                            EnergyAmount::new_unchecked(
                                (b_energy.current().raw() - energy_cost_each).max(0.0),
                            ),
                            b_energy.capacity(),
                        ),
                    );
                }
                let tick = self.world.tick();
                self.world.joints_mut_for_commit().create(
                    endpoints,
                    config.joints.creation_material_cost,
                    crate::core::joints::JointChannelConfig {
                        mechanical_strength: config.joints.mechanical_strength,
                        resource_transfer_rate: config.joints.resource_transfer_rate,
                        max_resource_transfer_per_tick: config
                            .joints
                            .max_resource_transfer_per_tick
                            .raw(),
                        signal_conductivity: config.joints.signal_conductivity,
                        signal_decay: config.joints.signal_decay,
                        heat_conductivity: config.joints.heat_conductivity,
                    },
                    tick,
                );
                phase2h_metrics.joint_created_count += 1;
            }

            let joint_ids = self.world.joints().active_ids().collect::<Vec<_>>();
            for joint_id in joint_ids {
                let endpoints = self
                    .world
                    .joints()
                    .endpoints(joint_id)
                    .expect("active joint id has endpoints");
                let channel = self
                    .world
                    .joints()
                    .config(joint_id)
                    .expect("active joint id has config");
                if channel.resource_transfer_rate <= 0.0
                    || channel.max_resource_transfer_per_tick <= 0.0
                {
                    continue;
                }
                let a = self.world.cells().resource_amount(endpoints.a).raw();
                let b = self.world.cells().resource_amount(endpoints.b).raw();
                if (a - b).abs() <= f32::EPSILON {
                    continue;
                }
                let (source, target, gradient) = if a > b {
                    (endpoints.a, endpoints.b, a - b)
                } else {
                    (endpoints.b, endpoints.a, b - a)
                };
                let free_target = self
                    .world
                    .cells()
                    .effective_free_capacity(
                        target,
                        config.material_effects.storage_capacity_per_unit,
                    )
                    .raw();
                let requested = (gradient * channel.resource_transfer_rate)
                    .min(channel.max_resource_transfer_per_tick)
                    .min(free_target);
                if requested <= 0.0 {
                    continue;
                }
                let moved = self
                    .world
                    .cells_mut_for_commit()
                    .transfer_resources_limited_by_effective_capacity(
                        source,
                        target,
                        ResourceAmount::new(requested).expect("joint resource transfer is clamped"),
                        config.material_effects.storage_capacity_per_unit,
                    );
                phase2h_metrics.joint_resource_transfer_amount += moved.raw();
            }

            let joint_ids = self.world.joints().active_ids().collect::<Vec<_>>();
            for joint_id in joint_ids {
                let endpoints = self
                    .world
                    .joints()
                    .endpoints(joint_id)
                    .expect("active joint id has endpoints");
                let channel = self
                    .world
                    .joints()
                    .config(joint_id)
                    .expect("active joint id has config");
                if channel.signal_conductivity <= 0.0 {
                    continue;
                }
                let overlap = self
                    .world
                    .contact_cache()
                    .pairs()
                    .iter()
                    .find(|pair| {
                        crate::core::joints::JointEndpoints::new(pair.a, pair.b) == Some(endpoints)
                    })
                    .map(|pair| pair.overlap)
                    .unwrap_or(0.0);
                let signal = (overlap * channel.signal_conductivity).clamp(0.0, 1.0);
                if signal <= 0.0 {
                    continue;
                }
                let readable_from = self.world.tick().next();
                self.world
                    .joints_mut_for_commit()
                    .add_next_signal(joint_id, signal, readable_from);
                phase2h_metrics.joint_signal_generated_total += signal;
            }

            let joint_ids = self.world.joints().active_ids().collect::<Vec<_>>();
            for joint_id in joint_ids {
                let endpoints = self
                    .world
                    .joints()
                    .endpoints(joint_id)
                    .expect("active joint id has endpoints");
                let channel = self
                    .world
                    .joints()
                    .config(joint_id)
                    .expect("active joint id has config");
                if channel.heat_conductivity <= 0.0 {
                    continue;
                }
                let temp_a = self.world.cells().temperature(endpoints.a).raw();
                let temp_b = self.world.cells().temperature(endpoints.b).raw();
                let delta = (temp_a - temp_b) * channel.heat_conductivity.clamp(0.0, 1.0) * 0.5;
                if delta.abs() <= f32::EPSILON {
                    continue;
                }
                let cells = self.world.cells_mut_for_commit();
                cells.set_temperature(
                    endpoints.a,
                    crate::core::units::Temperature::new(temp_a - delta),
                );
                cells.set_temperature(
                    endpoints.b,
                    crate::core::units::Temperature::new(temp_b + delta),
                );
                phase2h_metrics.joint_heat_transfer_amount += delta.abs();
            }
        }

        if config.local_interaction.enabled {
            let pairs = self.world.contact_cache().pairs().to_vec();
            for pair in pairs {
                let a = pair.a;
                let b = pair.b;
                let a_res = self.world.cells().resource_amount(a).raw();
                let b_res = self.world.cells().resource_amount(b).raw();
                if (a_res - b_res).abs() <= f32::EPSILON {
                    continue;
                }

                let (source, target, gradient) = if a_res > b_res {
                    (a, b, a_res - b_res)
                } else {
                    (b, a, b_res - a_res)
                };

                if !has_contact_exchange_capability(&self.world, source, &config.local_interaction)
                    || !has_contact_exchange_capability(
                        &self.world,
                        target,
                        &config.local_interaction,
                    )
                {
                    contact_exchange_rejections_no_capability += 1;
                    continue;
                }

                let free_target = self
                    .world
                    .cells()
                    .effective_free_capacity(
                        target,
                        config.material_effects.storage_capacity_per_unit,
                    )
                    .raw();
                let requested = (gradient * config.local_interaction.contact_exchange_rate)
                    .min(config.local_interaction.max_exchange_per_pair.raw())
                    .min(free_target);
                if requested <= 0.0 {
                    continue;
                }

                let moved = {
                    let cells = self.world.cells_mut_for_commit();
                    cells.transfer_resources_limited_by_effective_capacity(
                        source,
                        target,
                        ResourceAmount::new(requested).expect("requested exchange is clamped"),
                        config.material_effects.storage_capacity_per_unit,
                    )
                };

                if moved.raw() > 0.0 {
                    contact_exchange_amount += moved.raw();
                    contact_exchange_pairs_count += 1;
                    phase2g_metrics.boundary_leakage_amount += moved.raw();
                }
            }
        }
        let mut contact_stimulus_generated_total = 0.0_f32;
        if config.local_interaction.enabled
            && config.local_interaction.contact_stimulus_per_overlap > 0.0
        {
            let pairs = self.world.contact_cache().pairs().to_vec();
            for pair in pairs {
                for target in [pair.a, pair.b] {
                    let sensory = self.world.cells().capability_level(
                        target,
                        crate::core::process::MaterialCapability::ResourceSensing,
                    );
                    if sensory <= 0.0 {
                        continue;
                    }
                    let stimulus = (pair.overlap
                        * config.local_interaction.contact_stimulus_per_overlap
                        * sensory)
                        .clamp(0.0, 1.0);
                    self.world
                        .cells_mut_for_commit()
                        .add_next_contact_stimulus(target, stimulus);
                    contact_stimulus_generated_total += stimulus;
                }
            }
        }

        let mut metabolism_heat_total = 0.0_f32;
        let mut metabolism_waste_total = 0.0_f32;
        let mut process_attempts = 0_u32;
        let mut process_rejections = 0_u32;
        let mut repair_success_count = 0_u32;
        let mut repair_rejection_count = 0_u32;
        let mut diagnostics = ProcessDiagnostics::default();

        // Phase A: Uptake, Metabolism, Synthesis, Growth, and Displacement Reflex Loop
        for i in 0..len {
            let index = CellIndex::from_raw(i);
            if self.world.cells().lifecycle_state(index) == LifecycleState::Dead {
                continue;
            }

            // 1. Uptake
            if config.resource_interaction.enabled {
                let uptake_level = self.world.cells().capability_level(
                    index,
                    crate::core::process::MaterialCapability::ResourceUptake,
                );
                let max_uptake = config.resource_interaction.max_uptake_per_tick.raw()
                    * baseline_process_level(uptake_level)
                    * config.material_effects.transport_uptake_per_unit;
                let (feasible, feasibility) = run_process(
                    &self.world,
                    index,
                    ProcessId::LocalResourceUptake,
                    max_uptake,
                    &mut diagnostics,
                    &mut process_attempts,
                    &mut process_rejections,
                );
                if feasible {
                    let accepted_amount = match feasibility {
                        FeasibilityResult::Allowed {
                            accepted_amount, ..
                        } => accepted_amount,
                        FeasibilityResult::Rejected(_) => 0.0,
                    };
                    let layer = ResourceLayerIndex::from_raw(
                        config.resource_interaction.uptake_layer_index,
                    );
                    let coord = self
                        .world
                        .resources()
                        .coord_for_position(self.world.cells().position(index));
                    let external_available = self
                        .world
                        .resources()
                        .amount_at(layer, coord)
                        .expect("resource interaction layer is config-validated");
                    let requested =
                        ResourceAmount::new(external_available.raw().min(accepted_amount))
                            .expect("requested uptake is clamped");

                    let accepted = {
                        let cells = self.world.cells_mut_for_commit();
                        cells.add_resources_limited_by_effective_capacity(
                            index,
                            requested,
                            config.material_effects.storage_capacity_per_unit,
                        )
                    };
                    let remaining_external = external_available.saturating_sub(accepted);
                    self.world
                        .resources_mut_for_commit()
                        .set_amount_at(layer, coord, remaining_external)
                        .expect("resource interaction coord is derived from grid bounds");
                }
            }

            // 2. Metabolism
            let mut metabolism_heat = 0.0_f32;
            let mut metabolism_waste = 0.0_f32;
            let mut metabolism_energy = EnergyAmount::zero();

            if config.resource_interaction.enabled {
                let metabolism_level = self
                    .world
                    .cells()
                    .capability_level(index, crate::core::process::MaterialCapability::Metabolism);
                let req_amount = config
                    .resource_interaction
                    .metabolism_resource_per_tick
                    .raw()
                    * baseline_process_level(metabolism_level)
                    * config.material_effects.metabolic_conversion_per_unit;
                let (feasible, feasibility) = run_process(
                    &self.world,
                    index,
                    ProcessId::MetabolismEnergyConversion,
                    req_amount,
                    &mut diagnostics,
                    &mut process_attempts,
                    &mut process_rejections,
                );
                if feasible {
                    let accepted_amount = match feasibility {
                        FeasibilityResult::Allowed {
                            accepted_amount, ..
                        } => accepted_amount,
                        FeasibilityResult::Rejected(_) => 0.0,
                    };
                    let consumed = {
                        let cells = self.world.cells_mut_for_commit();
                        cells.consume_resources(
                            index,
                            ResourceAmount::new(accepted_amount)
                                .expect("accepted metabolism is clamped"),
                        )
                    };

                    metabolism_energy = EnergyAmount::new(
                        consumed.raw() * config.resource_interaction.energy_per_resource,
                    )
                    .expect("metabolism energy is config-validated");
                    metabolism_heat =
                        consumed.raw() * config.resource_interaction.heat_per_resource;
                    metabolism_waste =
                        consumed.raw() * config.resource_interaction.waste_per_resource;
                }
            }

            metabolism_heat_total += metabolism_heat;
            metabolism_waste_total += metabolism_waste;

            if metabolism_energy.raw() > 0.0 {
                let cells = self.world.cells_mut_for_commit();
                let current = cells.energy(index);
                let new_current = current
                    .current()
                    .saturating_add(metabolism_energy)
                    .clamp_max(current.capacity());
                cells.set_energy(index, EnergyBuffer::new(new_current, current.capacity()));
            }

            // 3. Material Synthesis
            let (feasible, _) = run_process(
                &self.world,
                index,
                ProcessId::MaterialSynthesis,
                1.0,
                &mut diagnostics,
                &mut process_attempts,
                &mut process_rejections,
            );
            if feasible {
                let _ = self.world.execute_synthesis(index);
            }

            // 4. Structural Growth
            if config.growth_enabled && config.resource_interaction.enabled {
                let (feasible, _) = run_process(
                    &self.world,
                    index,
                    ProcessId::GrowthResourceAllocation,
                    1.0,
                    &mut diagnostics,
                    &mut process_attempts,
                    &mut process_rejections,
                );
                if feasible {
                    let candidate_growth = ActionCandidate {
                        process_id: ProcessId::GrowthResourceAllocation,
                        requested_amount: 1.0,
                    };
                    let _ = self.world.execute_growth(index, &candidate_growth);
                }
            }

            // 5. Contractile Displacement
            let (feasible, _) = run_process(
                &self.world,
                index,
                ProcessId::ContractileDisplacement,
                1.0,
                &mut diagnostics,
                &mut process_attempts,
                &mut process_rejections,
            );
            if feasible {
                let _ = self.world.execute_displacement(index);
            }
        }

        if config.chemistry.repair.enabled {
            for i in 0..len {
                let index = CellIndex::from_raw(i);
                if self.world.cells().lifecycle_state(index) == LifecycleState::Dead {
                    continue;
                }
                let (feasible, feasibility) = run_process(
                    &self.world,
                    index,
                    ProcessId::RepairBoundary,
                    config.chemistry.repair.max_amount_per_tick,
                    &mut diagnostics,
                    &mut process_attempts,
                    &mut process_rejections,
                );
                match feasibility {
                    FeasibilityResult::Allowed {
                        accepted_amount,
                        energy_cost,
                        resource_cost,
                    } if feasible => {
                        let cells = self.world.cells_mut_for_commit();
                        let accepted_material =
                            MaterialAmount::new_unchecked(accepted_amount.max(0.0));
                        let requested_resource = ResourceAmount::new(resource_cost)
                            .expect("repair resource cost is clamped");
                        if let Some(resource_type) =
                            crate::core::world::repair_resource_type_id(&config)
                        {
                            let available = cells
                                .typed_resource_amount(index, resource_type)
                                .expect("repair resource type is derived from validated config");
                            if available.raw() + f32::EPSILON < resource_cost {
                                repair_rejection_count += 1;
                                continue;
                            }
                            let consumed = cells
                                .consume_typed_resource(index, resource_type, requested_resource)
                                .expect("repair resource type is derived from validated config");
                            if consumed.raw() + f32::EPSILON < resource_cost {
                                cells
                                    .set_typed_resource_amount(index, resource_type, available)
                                    .expect(
                                        "repair resource type is derived from validated config",
                                    );
                                repair_rejection_count += 1;
                                continue;
                            }
                        } else {
                            let available = cells.resource_amount(index);
                            if available.raw() + f32::EPSILON < resource_cost {
                                repair_rejection_count += 1;
                                continue;
                            }
                            let consumed = cells.consume_resources(index, requested_resource);
                            if consumed.raw() + f32::EPSILON < resource_cost {
                                cells.set_resources(index, available);
                                repair_rejection_count += 1;
                                continue;
                            }
                        }
                        let repair_remaining = MaterialAmount::new_unchecked(
                            (cells.repair_material(index).raw() - accepted_amount).max(0.0),
                        );
                        cells.set_repair_material(index, repair_remaining);

                        let boundary_next = cells
                            .boundary_material(index)
                            .saturating_add(accepted_material);
                        cells.set_boundary_material(index, boundary_next);
                        let damage_next = (cells.material_damage(index, MaterialSlot::Boundary)
                            - accepted_amount)
                            .max(0.0);
                        cells.set_material_damage(index, MaterialSlot::Boundary, damage_next);

                        let energy = cells.energy(index);
                        let next_energy = energy
                            .current()
                            .saturating_sub(EnergyAmount::new(energy_cost).unwrap());
                        cells.set_energy(index, EnergyBuffer::new(next_energy, energy.capacity()));
                        repair_success_count += 1;
                    }
                    FeasibilityResult::Rejected(_) => {
                        repair_rejection_count += 1;
                    }
                    _ => {}
                }
            }
        }

        // Positional Overlap Solver Loop
        let mut overlap_resolved = 0.0;
        {
            // Reset contact pressures at the start of physics solver
            {
                let cells = self.world.cells_mut_for_commit();
                for i in 0..cells.len() {
                    cells.set_contact_pressure(CellIndex::from_raw(i), 0.0);
                }
            }

            let iterations = config.space.physics_solver_iterations;
            let world_size = config.world.size;

            for _ in 0..iterations {
                self.world.rebuild_spatial_index();
                self.world.rebuild_contact_cache();
                contact_pressure_max_over_tick =
                    contact_pressure_max_over_tick.max(self.world.contact_cache().max_overlap());
                let pairs = self.world.contact_cache().pairs().to_vec();

                // 1. Resolve cell-cell overlaps
                for pair in &pairs {
                    let idx_i = pair.a;
                    let idx_j = pair.b;
                    let (pos_i, r_i) = {
                        let cells = self.world.cells();
                        if cells.runtime_flags(idx_i).inert || cells.runtime_flags(idx_j).inert {
                            continue;
                        }
                        (cells.position(idx_i), cells.radius(idx_i))
                    };
                    let (pos_j, r_j) = {
                        let cells = self.world.cells();
                        (cells.position(idx_j), cells.radius(idx_j))
                    };

                    let dx = pos_i.x() - pos_j.x();
                    let dy = pos_i.y() - pos_j.y();
                    let dist_sq = dx * dx + dy * dy;
                    let target_dist = r_i.raw() + r_j.raw();

                    if dist_sq < target_dist * target_dist {
                        let dist = dist_sq.sqrt();
                        let overlap = target_dist - dist;
                        overlap_resolved += overlap;

                        let (ux, uy) = if dist > 0.0 {
                            (dx / dist, dy / dist)
                        } else {
                            // If exactly overlapping, push along X axis deterministically based on ID order
                            let sign = if idx_i.raw() < idx_j.raw() { 1.0 } else { -1.0 };
                            (sign, 0.0)
                        };

                        // Push each cell by half of the overlap distance
                        let push_dist = overlap * 0.5;
                        let new_pos_i =
                            Position::new(pos_i.x() + ux * push_dist, pos_i.y() + uy * push_dist);
                        let new_pos_j =
                            Position::new(pos_j.x() - ux * push_dist, pos_j.y() - uy * push_dist);

                        let cells = self.world.cells_mut_for_commit();
                        cells.set_position(idx_i, new_pos_i);
                        cells.set_position(idx_j, new_pos_j);
                        let p_i = cells.contact_pressure(idx_i) + overlap;
                        let p_j = cells.contact_pressure(idx_j) + overlap;
                        cells.set_contact_pressure(idx_i, p_i);
                        cells.set_contact_pressure(idx_j, p_j);
                    }
                }

                // 2. Resolve wall boundaries (solid_wall)
                for i in 0..len {
                    let idx = CellIndex::from_raw(i);
                    let (pos, r) = {
                        let cells = self.world.cells();
                        if cells.runtime_flags(idx).inert {
                            continue;
                        }
                        (cells.position(idx), cells.radius(idx))
                    };

                    let radius = r.raw();
                    let mut px = pos.x();
                    let mut py = pos.y();
                    let mut clamped = false;

                    if px - radius < 0.0 {
                        px = radius;
                        clamped = true;
                    } else if px + radius > world_size.width() {
                        px = world_size.width() - radius;
                        clamped = true;
                    }

                    if py - radius < 0.0 {
                        py = radius;
                        clamped = true;
                    } else if py + radius > world_size.height() {
                        py = world_size.height() - radius;
                        clamped = true;
                    }

                    if clamped {
                        let cells = self.world.cells_mut_for_commit();
                        cells.set_position(idx, Position::new(px, py));
                    }
                }
            }
        }

        self.world.rebuild_spatial_index();
        self.world.rebuild_contact_cache();
        let contact_pressure_post_total = self.world.contact_cache().total_overlap();
        contact_pressure_max_over_tick =
            contact_pressure_max_over_tick.max(self.world.contact_cache().max_overlap());

        if config.joints.enabled {
            let joint_ids = self.world.joints().active_ids().collect::<Vec<_>>();
            for joint_id in joint_ids {
                let endpoints = self
                    .world
                    .joints()
                    .endpoints(joint_id)
                    .expect("active joint id has endpoints");
                let channel = self
                    .world
                    .joints()
                    .config(joint_id)
                    .expect("active joint id has config");
                if channel.mechanical_strength <= 0.0 {
                    continue;
                }
                let pos_a = self.world.cells().position(endpoints.a);
                let pos_b = self.world.cells().position(endpoints.b);
                let dx = pos_b.x() - pos_a.x();
                let dy = pos_b.y() - pos_a.y();
                let distance = (dx * dx + dy * dy).sqrt();
                let rest = self.world.cells().radius(endpoints.a).raw()
                    + self.world.cells().radius(endpoints.b).raw();
                if distance <= rest || distance <= 0.001 {
                    continue;
                }
                let correction =
                    (distance - rest) * channel.mechanical_strength.clamp(0.0, 1.0) * 0.5;
                let nx = dx / distance;
                let ny = dy / distance;
                let radius_a = self.world.cells().radius(endpoints.a).raw();
                let radius_b = self.world.cells().radius(endpoints.b).raw();
                let next_a = clamp_position_to_world(
                    Position::new(pos_a.x() + nx * correction, pos_a.y() + ny * correction),
                    radius_a,
                    config.world.size,
                );
                let next_b = clamp_position_to_world(
                    Position::new(pos_b.x() - nx * correction, pos_b.y() - ny * correction),
                    radius_b,
                    config.world.size,
                );
                let cells = self.world.cells_mut_for_commit();
                cells.set_position(endpoints.a, next_a);
                cells.set_position(endpoints.b, next_b);
                phase2h_metrics.joint_mechanical_correction_amount += correction * 2.0;
            }
            self.world.rebuild_spatial_index();
            self.world.rebuild_contact_cache();
        }

        // Phase B: Compute environment updates
        let heat_next = HeatAmount::new(
            (self.world.environment().heat().raw()
                + config.environment.heat_generated_per_tick.raw()
                + metabolism_heat_total
                - config.environment.heat_dissipation_rate.raw())
            .max(0.0),
        )
        .expect("heat accounting is clamped");

        let waste_next = WasteAmount::new(
            (self.world.environment().waste().raw()
                + config.environment.waste_generated_per_tick.raw()
                + metabolism_waste_total
                - config.environment.waste_sink_rate.raw())
            .max(0.0),
        )
        .expect("waste accounting is clamped");

        let heat_warning = heat_next.raw() > config.environment.heat_warning_threshold.raw();
        let heat_death = heat_next.raw() > config.environment.heat_death_threshold.raw();
        let waste_warning = waste_next.raw() > config.environment.waste_warning_threshold.raw();
        let waste_death = waste_next.raw() > config.environment.waste_death_threshold.raw();

        let mut final_energy = 0.0;
        let overall_lifecycle;
        let mut collapse_reason = CollapseReason::None;

        // Phase C: Pay cost and check lifecycle
        for i in 0..len {
            let index = CellIndex::from_raw(i);
            let cell_state_before = self.world.cells().lifecycle_state(index);
            if cell_state_before == LifecycleState::Dead {
                continue;
            }
            let current = self.world.cells().energy(index);
            let available = current
                .current()
                .saturating_add(config.cell.passive_energy_income);

            let used_capacity = self.world.cells().used_capacity(index);
            let effective_capacity = self
                .world
                .cells()
                .effective_capacity_limit(index, config.material_effects.storage_capacity_per_unit);
            let over_capacity = used_capacity.raw() > effective_capacity.raw();
            let critical_capacity_exceeded = used_capacity.raw()
                > effective_capacity.raw() + config.lifecycle.critical_capacity_overrun.raw();

            let full_cost = config.cell.mandatory_cost_per_tick;
            let dormant_cost = EnergyAmount::new(
                config.cell.mandatory_cost_per_tick.raw()
                    * config.lifecycle.dormant_mandatory_cost_modifier,
            )
            .expect("dormant cost modifier is validated");

            // 1. Wake up check
            let mut current_state = cell_state_before;
            if current_state == LifecycleState::Dormant {
                let can_pay_full = available.raw() >= full_cost.raw();
                let energy_after_full = available.saturating_sub(full_cost);
                let is_stressed_after_full = energy_after_full.raw()
                    < config.lifecycle.stress_energy_threshold.raw()
                    || heat_warning
                    || waste_warning
                    || over_capacity;
                if can_pay_full && !is_stressed_after_full {
                    current_state = LifecycleState::Alive;
                }
            }

            // 2. Determine upkeep cost to pay
            let upkeep_cost = if current_state == LifecycleState::Dormant {
                dormant_cost
            } else {
                full_cost
            };

            let mandatory_paid = available.raw() >= upkeep_cost.raw();
            let mut energy_after = if mandatory_paid {
                available.saturating_sub(upkeep_cost)
            } else {
                available
            };

            // 3. Evaluate death and dormancy transitions
            let mut is_dead = false;
            let mut cell_collapse_reason = CollapseReason::None;
            let mut next_state;

            if energy_after.raw() <= 0.0 {
                is_dead = true;
                cell_collapse_reason = CollapseReason::EnergyDepleted;
                next_state = LifecycleState::Dead;
            } else if critical_capacity_exceeded {
                is_dead = true;
                cell_collapse_reason = CollapseReason::CapacityExceeded;
                next_state = LifecycleState::Dead;
            } else if heat_death {
                is_dead = true;
                cell_collapse_reason = CollapseReason::HeatLimitExceeded;
                next_state = LifecycleState::Dead;
            } else if waste_death {
                is_dead = true;
                cell_collapse_reason = CollapseReason::WasteLimitExceeded;
                next_state = LifecycleState::Dead;
            } else if !mandatory_paid {
                let can_go_dormant = current_state != LifecycleState::Dormant
                    && config.lifecycle.dormancy_allowed
                    && available.raw() >= dormant_cost.raw();
                if can_go_dormant {
                    next_state = LifecycleState::Dormant;
                    energy_after = available.saturating_sub(dormant_cost);
                    if energy_after.raw() <= 0.0 {
                        is_dead = true;
                        cell_collapse_reason = CollapseReason::EnergyDepleted;
                        next_state = LifecycleState::Dead;
                    }
                } else {
                    is_dead = true;
                    cell_collapse_reason = CollapseReason::MandatoryCostUnpaid;
                    next_state = LifecycleState::Dead;
                }
            } else {
                // Determine state for alive cell
                if current_state == LifecycleState::Dormant {
                    next_state = LifecycleState::Dormant;
                } else {
                    let is_stressed = energy_after.raw()
                        < config.lifecycle.stress_energy_threshold.raw()
                        || heat_warning
                        || waste_warning
                        || over_capacity;
                    if is_stressed {
                        next_state = LifecycleState::Stressed;
                    } else {
                        next_state = LifecycleState::Alive;
                    }
                }
            }

            // Commit cell state changes
            let energy_after_clamped = energy_after.clamp_max(current.capacity());
            let radius_val = self.world.cells().radius(index).raw();
            let pressure_val = self.world.cells().contact_pressure(index);
            let division_ready = radius_val >= config.growth.growth_target_radius.raw()
                && pressure_val <= config.growth.max_division_pressure;

            {
                let cells = self.world.cells_mut_for_commit();
                cells.set_energy(
                    index,
                    EnergyBuffer::new(energy_after_clamped, current.capacity()),
                );
                cells.set_lifecycle_state(index, next_state);
                cells.set_runtime_flags(
                    index,
                    RuntimeFlags {
                        mandatory_paid,
                        stalled: !mandatory_paid,
                        over_capacity,
                        inert: cells.runtime_flags(index).inert
                            || (is_dead && !config.decomposition.enabled),
                        division_ready,
                    },
                );
            }

            let current_tick = self.world.tick();
            let cell_id = self.world.cells().id_at(index);
            if !mandatory_paid {
                self.world.events_mut_for_commit().push(
                    current_tick,
                    EventKind::MandatoryCostFailed,
                    Some(cell_id),
                );
            }
            if next_state == LifecycleState::Dead {
                self.world.events_mut_for_commit().push(
                    current_tick,
                    EventKind::CellDead,
                    Some(cell_id),
                );
            }

            final_energy += energy_after_clamped.raw();

            if collapse_reason == CollapseReason::None
                && cell_collapse_reason != CollapseReason::None
            {
                collapse_reason = cell_collapse_reason;
            }
        }

        let mut any_alive = false;
        let mut any_stressed = false;
        let mut any_dormant = false;

        for i in 0..len {
            let idx = CellIndex::from_raw(i);
            match self.world.cells().lifecycle_state(idx) {
                LifecycleState::Alive => any_alive = true,
                LifecycleState::Stressed => any_stressed = true,
                LifecycleState::Dormant => any_dormant = true,
                LifecycleState::Dead => {}
            }
        }

        if any_stressed {
            overall_lifecycle = LifecycleState::Stressed;
        } else if any_dormant {
            overall_lifecycle = LifecycleState::Dormant;
        } else if any_alive {
            overall_lifecycle = LifecycleState::Alive;
        } else {
            overall_lifecycle = LifecycleState::Dead;
        }

        let mut divisions_count = 0_u32;
        let mut births_count = 0_u32;

        if self.world.config().division.enabled {
            let mut division_candidates = Vec::new();
            let len = self.world.cells().len();
            for i in 0..len {
                let idx = CellIndex::from_raw(i);
                if self.world.cells().lifecycle_state(idx) == LifecycleState::Alive
                    && self.world.cells().runtime_flags(idx).division_ready
                {
                    division_candidates.push(idx);
                }
            }

            let current_tick = self.world.tick();
            for idx in division_candidates {
                let candidate = crate::core::process::ActionCandidate {
                    process_id: crate::core::process::ProcessId::Division,
                    requested_amount: 1.0,
                };
                *diagnostics
                    .attempts_by_process
                    .entry(ProcessId::Division)
                    .or_insert(0) += 1;
                process_attempts += 1;
                match self.world.validate_feasibility(idx, &candidate) {
                    FeasibilityResult::Allowed { .. } => {
                        if let Ok(outcome) = self.world.execute_division(idx, &candidate) {
                            divisions_count += 1;
                            births_count += 1;
                            self.world.events_mut_for_commit().push(
                                current_tick,
                                EventKind::CellDivided,
                                Some(outcome.parent_id),
                            );
                            self.world.events_mut_for_commit().push(
                                current_tick,
                                EventKind::CellBorn,
                                Some(outcome.daughter_b_id),
                            );
                        }
                    }
                    FeasibilityResult::Rejected(reason) => {
                        *diagnostics
                            .rejections_by_process
                            .entry(ProcessId::Division)
                            .or_insert(0) += 1;
                        *diagnostics.rejections_by_reason.entry(reason).or_insert(0) += 1;
                        process_rejections += 1;
                    }
                }
            }
        }

        if config.decomposition.enabled && config.chemistry.resources.is_empty() == false {
            let tick = self.world.tick();
            let converted = self
                .world
                .fragments_mut_for_commit()
                .drain_convertible_before(tick, config.decomposition.materials_per_tick);
            for fragment in converted {
                let layer = ResourceLayerIndex::from_raw(config.decomposition.resource_layer_index);
                let coord = self
                    .world
                    .resources()
                    .coord_for_position(fragment.position());
                if let Ok(current) = self.world.resources().amount_at(layer, coord) {
                    let next = current
                        .saturating_add(ResourceAmount::new_unchecked(fragment.amount().raw()));
                    if self
                        .world
                        .resources_mut_for_commit()
                        .set_amount_at(layer, coord, next)
                        .is_ok()
                    {
                        phase2g_metrics.fragment_converted_amount += fragment.amount().raw();
                    }
                }
            }
        }
        let fragment_amount_before = self.world.fragments().total_amount().raw();
        let decomposed_cells_count = self.world.execute_decomposition_for_dead_cells();
        let fragment_amount_after = self.world.fragments().total_amount().raw();
        phase2g_metrics.fragment_created_amount +=
            (fragment_amount_after - fragment_amount_before).max(0.0);

        let repair_placeholder_available = (0..self.world.cells().len()).any(|i| {
            let idx = CellIndex::from_raw(i);
            self.world.cells().lifecycle_state(idx) != LifecycleState::Dead
                && self.world.cells().repair_material(idx).raw() > 0.0
        });
        if repair_placeholder_available
            && !diagnostics
                .tool_limited_mechanisms
                .iter()
                .any(|name| name == "repair")
        {
            diagnostics
                .tool_limited_mechanisms
                .push("repair".to_string());
        }

        if config.joints.enabled {
            let tick = self.world.tick();
            let (degraded, broken) = self.world.joints_mut_for_commit().degrade_active(
                config.joints.upkeep_material_decay_per_tick,
                config.joints.break_damage_threshold,
                tick,
            );
            phase2h_metrics.joint_degradation_amount += degraded;
            phase2h_metrics.joint_broken_count += broken;
        }

        self.world.environment_mut_for_commit().set_heat(heat_next);
        self.world
            .environment_mut_for_commit()
            .set_waste(waste_next);
        for (layer, resource) in config.chemistry.resources.iter().enumerate() {
            if resource.diffusion_rate <= 0.0 {
                continue;
            }
            if let Ok(diffused) = self
                .world
                .resources_mut_for_commit()
                .diffuse_layer(ResourceLayerIndex::from_raw(layer), resource.diffusion_rate)
            {
                phase2g_metrics.resource_diffused_amount += diffused.raw();
            }
        }
        let resource_amount_before_decay = self.aggregate_external_resources();
        self.world
            .resources_mut_for_commit()
            .decay_or_passive_update();
        let resource_amount_after_decay = self.aggregate_external_resources();
        phase2g_metrics.resource_decay_amount +=
            (resource_amount_before_decay - resource_amount_after_decay).max(0.0);
        self.world
            .cells_mut_for_commit()
            .commit_contact_stimulus(config.local_interaction.stimulus_decay_per_tick);
        self.world.advance_tick();

        let current_tick = self.world.tick();
        self.world
            .events_mut_for_commit()
            .push(current_tick, EventKind::TickCommitted, None);

        let survival_result = if overall_lifecycle == LifecycleState::Dead {
            SurvivalResult::Collapse
        } else if overall_lifecycle == LifecycleState::Stressed
            || overall_lifecycle == LifecycleState::Dormant
        {
            SurvivalResult::Fragile
        } else {
            SurvivalResult::Stable
        };

        let min_energy = final_energy;
        let max_energy = final_energy;
        let mut accounting_after = IntegratedAccountingSnapshot::from_world(&self.world);
        accounting_after.explicit_sinks = phase2g_metrics.resource_decay_amount
            + phase2g_metrics.material_degradation_amount
            + phase2g_metrics.boundary_leakage_amount
            + phase2h_metrics.joint_degradation_amount;
        let accounting_delta = MatterAccountingDelta::between(accounting_before, accounting_after);

        Ok(RunSummary {
            tick: self.world.tick(),
            config_hash: config.config_hash(),
            survival_result,
            collapse_reason,
            metrics: self.build_metrics_summary(
                final_energy,
                heat_next.raw(),
                waste_next.raw(),
                min_energy,
                max_energy,
                overlap_resolved,
                contact_pairs_count,
                contact_pressure_pre_total,
                contact_pressure_post_total,
                contact_pressure_max_over_tick,
                contact_exchange_amount,
                contact_exchange_pairs_count,
                contact_exchange_rejections_no_capability,
                contact_stimulus_generated_total,
                contact_stimulus_readable_total_for_summary,
                process_attempts,
                process_rejections,
                divisions_count,
                births_count,
                decomposed_cells_count,
                phase2g_metrics,
                repair_success_count,
                repair_rejection_count,
                phase2h_metrics,
                accounting_before,
                accounting_after,
                accounting_delta,
            ),
            diagnostics,
        })
    }

    pub fn run_until_configured_tick(&mut self) -> Result<RunSummary, TickError> {
        let target = self.world.config().world.tick_count.raw();
        let mut latest = self.step()?;
        let mut min_energy = latest.metrics.final_energy;
        let mut max_energy = latest.metrics.final_energy;

        if latest.survival_result == SurvivalResult::Collapse {
            latest.metrics.min_energy = min_energy;
            latest.metrics.max_energy = max_energy;
            return Ok(latest);
        }

        while self.world.tick().raw() < target {
            latest = self.step()?;
            min_energy = min_energy.min(latest.metrics.final_energy);
            max_energy = max_energy.max(latest.metrics.final_energy);
            if latest.survival_result == SurvivalResult::Collapse {
                break;
            }
        }

        latest.metrics.min_energy = min_energy;
        latest.metrics.max_energy = max_energy;
        Ok(latest)
    }

    pub fn last_commit_summary(&self) -> CommitSummary {
        CommitSummary {
            ticks_committed: self.world.tick().raw(),
            events_emitted: 0,
        }
    }

    fn commit_passive_chemistry_reactions(
        &mut self,
        config: &RuntimeConfig,
    ) -> Phase2GMetricsDelta {
        let mut metrics = Phase2GMetricsDelta::default();
        if config.chemistry.reactions.is_empty() {
            return metrics;
        }

        let resource_layer = |id: &str| {
            config
                .chemistry
                .resources
                .iter()
                .position(|resource| resource.id == id)
                .map(ResourceLayerIndex::from_raw)
        };
        let width = self.world.resources().width();
        let height = self.world.resources().height();
        for (reaction_index, reaction) in config.chemistry.reactions.iter().enumerate() {
            if reaction.mode != "passive" {
                continue;
            }
            if reaction.rate <= 0.0 {
                continue;
            }
            for y in 0..height {
                for x in 0..width {
                    let coord = crate::core::units::GridCoord::new(x, y);
                    if !reaction_occurs(
                        config.world.seed.raw(),
                        self.world.tick().raw(),
                        x,
                        y,
                        reaction_index as u32,
                        reaction.probability,
                    ) {
                        continue;
                    }
                    metrics.reaction_matched_count += 1;

                    let inputs = reaction
                        .inputs
                        .iter()
                        .map(|(id, amount)| {
                            resource_layer(id).map(|layer| (layer, amount * reaction.rate))
                        })
                        .collect::<Option<Vec<_>>>();
                    let outputs = reaction
                        .outputs
                        .iter()
                        .map(|(id, amount)| {
                            resource_layer(id).map(|layer| (layer, amount * reaction.rate))
                        })
                        .collect::<Option<Vec<_>>>();
                    let (Some(inputs), Some(outputs)) = (inputs, outputs) else {
                        metrics.reaction_rejected_count += 1;
                        continue;
                    };
                    if inputs.iter().any(|(layer, required)| {
                        self.world
                            .resources()
                            .amount_at(*layer, coord)
                            .map(|available| available.raw() + 1e-6 < *required)
                            .unwrap_or(true)
                    }) {
                        metrics.reaction_rejected_count += 1;
                        continue;
                    }

                    metrics.reaction_executed_count += 1;
                    metrics.reaction_input_amount +=
                        inputs.iter().map(|(_, amount)| *amount).sum::<f32>();
                    metrics.reaction_output_amount +=
                        outputs.iter().map(|(_, amount)| *amount).sum::<f32>();
                    metrics.reaction_heat_generated += reaction.heat_output * reaction.rate;
                    metrics.reaction_energy_output += reaction.energy_output * reaction.rate;
                    let heat_delta = if config.chemistry.heat.capacity > 0.0 {
                        reaction.heat_output * reaction.rate / config.chemistry.heat.capacity
                    } else {
                        0.0
                    };
                    if heat_delta > 0.0 {
                        let heated_cells = (0..self.world.cells().len())
                            .map(CellIndex::from_raw)
                            .filter(|cell| {
                                self.world
                                    .resources()
                                    .coord_for_position(self.world.cells().position(*cell))
                                    == coord
                            })
                            .collect::<Vec<_>>();
                        for cell in heated_cells {
                            let temperature = self.world.cells().temperature(cell);
                            self.world.cells_mut_for_commit().set_temperature(
                                cell,
                                crate::core::units::Temperature::new(
                                    temperature.raw() + heat_delta,
                                ),
                            );
                        }
                    }
                    let mut changes = vec![0.0_f32; self.world.resources().layer_count()];
                    for (layer, amount) in inputs {
                        changes[layer.raw()] -= amount;
                    }
                    for (layer, amount) in outputs {
                        changes[layer.raw()] += amount;
                    }
                    for (layer, change) in changes.into_iter().enumerate() {
                        if change == 0.0 {
                            continue;
                        }
                        let layer = ResourceLayerIndex::from_raw(layer);
                        let current = self
                            .world
                            .resources()
                            .amount_at(layer, coord)
                            .expect("chemistry layer is derived from validated config");
                        let next = ResourceAmount::new((current.raw() + change).max(0.0))
                            .expect("reaction amount is clamped");
                        self.world
                            .resources_mut_for_commit()
                            .set_amount_at(layer, coord, next)
                            .expect("chemistry coordinate is in grid bounds");
                    }
                }
            }
        }
        metrics
    }

    fn commit_controlled_chemistry_reactions(
        &mut self,
        config: &RuntimeConfig,
    ) -> Phase2GMetricsDelta {
        let mut metrics = Phase2GMetricsDelta::default();
        let resource_type = |id: &str| {
            config
                .chemistry
                .resources
                .iter()
                .position(|resource| resource.id == id)
                .map(|index| crate::core::ids::ResourceTypeId::from_raw(index as u32))
        };
        for cell_raw in 0..self.world.cells().len() {
            let cell = CellIndex::from_raw(cell_raw);
            for reaction in config.chemistry.reactions.iter().filter(|reaction| {
                reaction.mode == "controlled"
                    && reaction.process_id.as_deref() == Some("energy_conversion")
                    && reaction.rate > 0.0
            }) {
                metrics.reaction_matched_count += 1;
                if !self
                    .world
                    .cells()
                    .has_capability(cell, crate::core::process::MaterialCapability::Metabolism)
                {
                    metrics.reaction_rejected_count += 1;
                    continue;
                }
                if !reaction.required_materials.iter().all(|(id, amount)| {
                    material_slot_for_chemistry_id(id).is_some_and(|slot| {
                        self.world
                            .cells()
                            .material_amount_for_slot(cell, slot)
                            .raw()
                            >= *amount
                    })
                }) {
                    metrics.reaction_rejected_count += 1;
                    continue;
                }
                let inputs = reaction
                    .inputs
                    .iter()
                    .map(|(id, amount)| resource_type(id).map(|id| (id, amount * reaction.rate)))
                    .collect::<Option<Vec<_>>>();
                let Some(inputs) = inputs else {
                    metrics.reaction_rejected_count += 1;
                    continue;
                };
                if inputs.iter().any(|(id, amount)| {
                    self.world
                        .cells()
                        .typed_resource_amount(cell, *id)
                        .map(|available| available.raw() + 1e-6 < *amount)
                        .unwrap_or(true)
                }) {
                    metrics.reaction_rejected_count += 1;
                    continue;
                }
                metrics.reaction_executed_count += 1;
                metrics.reaction_input_amount +=
                    inputs.iter().map(|(_, amount)| *amount).sum::<f32>();
                metrics.reaction_output_amount += reaction
                    .outputs
                    .iter()
                    .map(|(_, amount)| amount * reaction.rate)
                    .sum::<f32>();
                metrics.reaction_heat_generated += reaction.heat_output * reaction.rate;
                metrics.reaction_energy_output += reaction.energy_output * reaction.rate;
                for (id, amount) in inputs {
                    let _ = self.world.cells_mut_for_commit().consume_typed_resource(
                        cell,
                        id,
                        ResourceAmount::new(amount).expect("reaction amount is validated"),
                    );
                }
                if reaction.energy_output > 0.0 {
                    let cells = self.world.cells_mut_for_commit();
                    let energy = cells.energy(cell);
                    let next = energy.current().saturating_add(
                        EnergyAmount::new(reaction.energy_output * reaction.rate)
                            .expect("reaction energy is validated"),
                    );
                    cells.set_energy(cell, EnergyBuffer::new(next, energy.capacity()));
                }
                if reaction.heat_output > 0.0 && config.chemistry.heat.capacity > 0.0 {
                    let cells = self.world.cells_mut_for_commit();
                    let temperature = cells.temperature(cell);
                    cells.set_temperature(
                        cell,
                        crate::core::units::Temperature::new(
                            temperature.raw()
                                + reaction.heat_output * reaction.rate
                                    / config.chemistry.heat.capacity,
                        ),
                    );
                }
            }
        }
        metrics
    }

    fn commit_local_heat_material_degradation(&mut self, config: &RuntimeConfig) -> f32 {
        let warning = config.chemistry.heat.warning_threshold;
        let death = config.chemistry.heat.death_threshold;
        if config.chemistry.heat.capacity <= 0.0 || warning <= 0.0 || death <= warning {
            return 0.0;
        }

        let mut degraded_total = 0.0_f32;
        for cell_raw in 0..self.world.cells().len() {
            let cell = CellIndex::from_raw(cell_raw);
            let temperature = self.world.cells().temperature(cell);
            if temperature.raw() <= warning {
                continue;
            }

            let damage_rate = ((temperature.raw() - warning) / (death - warning)).clamp(0.0, 1.0);
            let cells = self.world.cells_mut_for_commit();
            for slot in MaterialSlot::ALL {
                degraded_total += cells
                    .apply_thermal_damage(cell, slot, temperature, warning, damage_rate)
                    .raw();
            }
        }
        degraded_total
    }

    fn commit_material_type_decay(&mut self, config: &RuntimeConfig) -> f32 {
        if config.chemistry.materials.is_empty() {
            return 0.0;
        }

        let mut degraded_total = 0.0_f32;
        for material in &config.chemistry.materials {
            let Some(slot) = material_slot_for_chemistry_id(&material.id) else {
                continue;
            };
            let decay_rate = material.decay_rate.clamp(0.0, 1.0);
            if decay_rate <= 0.0 {
                continue;
            }

            for cell_raw in 0..self.world.cells().len() {
                let cell = CellIndex::from_raw(cell_raw);
                let current = self.world.cells().material_amount_for_slot(cell, slot);
                let decayed = (current.raw() * decay_rate).min(current.raw());
                if decayed <= 0.0 {
                    continue;
                }
                let remaining = MaterialAmount::new_unchecked((current.raw() - decayed).max(0.0));
                self.world
                    .cells_mut_for_commit()
                    .set_material_amount_for_slot(cell, slot, remaining);
                degraded_total += decayed;
            }
        }
        degraded_total
    }

    fn aggregate_external_resources(&self) -> f32 {
        (0..self.world.resources().layer_count())
            .map(|layer| {
                self.world
                    .resources()
                    .total_amount_for_layer(ResourceLayerIndex::from_raw(layer))
                    .expect("layer range is derived from layer_count")
                    .raw()
            })
            .sum()
    }

    #[allow(clippy::too_many_arguments)]
    fn build_metrics_summary(
        &self,
        final_energy: f32,
        heat: f32,
        waste: f32,
        min_energy: f32,
        max_energy: f32,
        overlap_resolved: f32,
        contact_pairs_count: u32,
        contact_pressure_pre_total: f32,
        contact_pressure_post_total: f32,
        contact_pressure_max_over_tick: f32,
        contact_exchange_amount: f32,
        contact_exchange_pairs_count: u32,
        contact_exchange_rejections_no_capability: u32,
        contact_stimulus_generated_total: f32,
        contact_stimulus_readable_total: f32,
        process_attempts: u32,
        process_rejections: u32,
        divisions_count: u32,
        births_count: u32,
        decomposed_cells_count: u32,
        phase2g_metrics: Phase2GMetricsDelta,
        repair_success_count: u32,
        repair_rejection_count: u32,
        phase2h_metrics: Phase2HMetricsDelta,
        accounting_before: IntegratedAccountingSnapshot,
        accounting_after: IntegratedAccountingSnapshot,
        accounting_delta: MatterAccountingDelta,
    ) -> MetricsSummary {
        let cells = self.world.cells();
        let mut final_internal_resources = 0.0_f32;
        let mut final_used_capacity = 0.0_f32;
        let mut final_free_capacity = 0.0_f32;
        let mut growth_readiness = false;
        let mut alive_cells_count = 0_u32;
        let mut dead_cells_count = 0_u32;
        let mut sensory_input_accumulated = 0.0_f32;
        let mut repair_placeholder_available = false;

        for i in 0..cells.len() {
            let idx = CellIndex::from_raw(i);
            match cells.lifecycle_state(idx) {
                LifecycleState::Dead => dead_cells_count += 1,
                _ => {
                    alive_cells_count += 1;
                    final_internal_resources += cells.resource_amount(idx).raw();
                    final_used_capacity += cells.used_capacity(idx).raw();
                    final_free_capacity += cells.free_capacity(idx).raw();
                    if cells.runtime_flags(idx).division_ready {
                        growth_readiness = true;
                    }
                    let sensory_level = cells.sensory_material(idx).raw();
                    if sensory_level > 0.0 {
                        let coord = self
                            .world
                            .resources()
                            .coord_for_position(cells.position(idx));
                        let local_resource = self
                            .world
                            .resources()
                            .amount_at(ResourceLayerIndex::from_raw(0), coord)
                            .map(|amount| amount.raw())
                            .unwrap_or(0.0);
                        sensory_input_accumulated += sensory_level
                            * self.world.config().material_effects.sensory_input_per_unit
                            * (local_resource + cells.contact_pressure(idx));
                    }
                    if cells.repair_material(idx).raw() > 0.0 {
                        repair_placeholder_available = true;
                    }
                }
            }
        }

        MetricsSummary {
            final_energy,
            heat,
            waste,
            min_energy,
            max_energy,
            final_internal_resources,
            final_external_resources: self.aggregate_external_resources(),
            final_used_capacity,
            final_free_capacity,
            growth_readiness,
            contact_pairs_count,
            contact_pressure_pre_total,
            contact_pressure_post_total,
            contact_pressure_max_over_tick,
            contact_exchange_amount,
            contact_exchange_pairs_count,
            contact_exchange_rejections_no_capability,
            contact_stimulus_generated_total,
            contact_stimulus_readable_total,
            overlap_resolved,
            process_attempts,
            process_rejections,
            alive_cells_count,
            dead_cells_count,
            divisions_count,
            births_count,
            decomposed_cells_count,
            sensory_input_accumulated,
            repair_placeholder_available,
            reaction_matched_count: phase2g_metrics.reaction_matched_count,
            reaction_executed_count: phase2g_metrics.reaction_executed_count,
            reaction_rejected_count: phase2g_metrics.reaction_rejected_count,
            reaction_input_amount: phase2g_metrics.reaction_input_amount,
            reaction_output_amount: phase2g_metrics.reaction_output_amount,
            reaction_heat_generated: phase2g_metrics.reaction_heat_generated,
            reaction_energy_output: phase2g_metrics.reaction_energy_output,
            reaction_accounting_error: phase2g_metrics.reaction_accounting_error,
            resource_diffused_amount: phase2g_metrics.resource_diffused_amount,
            resource_decay_amount: phase2g_metrics.resource_decay_amount,
            fragment_created_amount: phase2g_metrics.fragment_created_amount,
            fragment_converted_amount: phase2g_metrics.fragment_converted_amount,
            material_degradation_amount: phase2g_metrics.material_degradation_amount,
            boundary_leakage_amount: phase2g_metrics.boundary_leakage_amount,
            repair_success_count,
            repair_rejection_count,
            joint_count: self.world.joints().active_ids().count() as u32,
            joint_created_count: phase2h_metrics.joint_created_count,
            joint_creation_rejected_count: phase2h_metrics.joint_creation_rejected_count,
            joint_broken_count: phase2h_metrics.joint_broken_count,
            joint_resource_transfer_amount: phase2h_metrics.joint_resource_transfer_amount,
            joint_signal_generated_total: phase2h_metrics.joint_signal_generated_total,
            joint_signal_readable_total: phase2h_metrics.joint_signal_readable_total,
            joint_heat_transfer_amount: phase2h_metrics.joint_heat_transfer_amount,
            joint_degradation_amount: phase2h_metrics.joint_degradation_amount,
            joint_mechanical_correction_amount: phase2h_metrics.joint_mechanical_correction_amount,
            integrated_matter_before: accounting_before.total_matter(),
            integrated_matter_after: accounting_after.total_matter(),
            integrated_matter_unclassified_loss: accounting_delta.unclassified_loss,
            integrated_matter_unclassified_gain: accounting_delta.unclassified_gain,
        }
    }
}

fn baseline_process_level(raw_level: f32) -> f32 {
    if raw_level > 0.0 {
        raw_level.max(1.0)
    } else {
        0.0
    }
}

fn clamp_position_to_world(position: Position, radius: f32, world_size: WorldSize) -> Position {
    Position::new(
        position.x().clamp(radius, world_size.width() - radius),
        position.y().clamp(radius, world_size.height() - radius),
    )
}

fn reaction_occurs(
    seed: u64,
    tick: u64,
    x: usize,
    y: usize,
    reaction_index: u32,
    probability: f32,
) -> bool {
    if probability >= 1.0 {
        return true;
    }
    if probability <= 0.0 {
        return false;
    }
    let mut value = seed
        ^ tick.wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (x as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9)
        ^ (y as u64).wrapping_mul(0x94D0_49BB_1331_11EB)
        ^ reaction_index as u64;
    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;
    let sample = (value >> 40) as f32 / (1_u32 << 24) as f32;
    sample < probability
}

fn material_slot_for_chemistry_id(id: &str) -> Option<MaterialSlot> {
    let normalized = id.to_ascii_lowercase();
    if normalized.contains("boundary") {
        Some(MaterialSlot::Boundary)
    } else if normalized.contains("transport") {
        Some(MaterialSlot::Transport)
    } else if normalized.contains("metabolic") {
        Some(MaterialSlot::Metabolic)
    } else if normalized.contains("storage") {
        Some(MaterialSlot::Storage)
    } else if normalized.contains("synthesis") {
        Some(MaterialSlot::Synthesis)
    } else if normalized.contains("structural") {
        Some(MaterialSlot::Structural)
    } else if normalized.contains("repair") {
        Some(MaterialSlot::Repair)
    } else if normalized.contains("contractile") {
        Some(MaterialSlot::Contractile)
    } else if normalized.contains("sensory") {
        Some(MaterialSlot::Sensory)
    } else {
        None
    }
}

fn has_contact_exchange_capability(
    world: &WorldState,
    index: CellIndex,
    config: &crate::core::config::LocalInteractionConfig,
) -> bool {
    let cells = world.cells();
    cells.capability_level(
        index,
        crate::core::process::MaterialCapability::BoundaryPermeability,
    ) >= config.min_boundary_capability
        && cells.capability_level(
            index,
            crate::core::process::MaterialCapability::ResourceUptake,
        ) >= config.min_transport_capability
}

fn run_process(
    world: &WorldState,
    index: CellIndex,
    process_id: ProcessId,
    requested_amount: f32,
    diagnostics: &mut ProcessDiagnostics,
    process_attempts: &mut u32,
    process_rejections: &mut u32,
) -> (bool, FeasibilityResult) {
    let candidate = ActionCandidate {
        process_id,
        requested_amount,
    };
    *process_attempts += 1;
    *diagnostics
        .attempts_by_process
        .entry(process_id)
        .or_insert(0) += 1;

    let feasibility = world.validate_feasibility(index, &candidate);
    match feasibility {
        FeasibilityResult::Allowed { .. } => (true, feasibility),
        FeasibilityResult::Rejected(reason) => {
            *process_rejections += 1;
            *diagnostics
                .rejections_by_process
                .entry(process_id)
                .or_insert(0) += 1;
            *diagnostics.rejections_by_reason.entry(reason).or_insert(0) += 1;
            (false, feasibility)
        }
    }
}
