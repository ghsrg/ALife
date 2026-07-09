use crate::core::cell_store::{CellIndex, EnergyBuffer, LifecycleState, RuntimeFlags};
use crate::core::config::RuntimeConfig;
use crate::core::deltas::CommitSummary;
use crate::core::events::EventKind;
use crate::core::process::{ActionCandidate, FeasibilityResult, ProcessId};
use crate::core::resources::ResourceLayerIndex;
use crate::core::summary::{
    CollapseReason, MetricsSummary, ProcessDiagnostics, RunSummary, SurvivalResult,
};
use crate::core::units::{EnergyAmount, HeatAmount, Position, ResourceAmount, WasteAmount};
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

        // Rebuild Spatial Index at the start of tick
        self.world.rebuild_spatial_index();

        let mut metabolism_heat_total = 0.0_f32;
        let mut metabolism_waste_total = 0.0_f32;
        let mut process_attempts = 0_u32;
        let mut process_rejections = 0_u32;
        let mut diagnostics = ProcessDiagnostics::default();

        // Phase A: Uptake, Metabolism, Synthesis, Growth, and Displacement Reflex Loop
        for i in 0..len {
            let index = CellIndex::from_raw(i);
            if self.world.cells().lifecycle_state(index) == LifecycleState::Dead {
                continue;
            }

            // 1. Uptake
            if config.resource_interaction.enabled {
                let max_uptake = config.resource_interaction.max_uptake_per_tick.raw();
                let (feasible, _) = run_process(
                    &self.world,
                    index,
                    ProcessId::LocalResourceUptake,
                    max_uptake,
                    &mut diagnostics,
                    &mut process_attempts,
                    &mut process_rejections,
                );
                if feasible {
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
                    let free_cap = self.world.cells().free_capacity(index).raw();
                    let max_uptake = config
                        .resource_interaction
                        .max_uptake_per_tick
                        .raw()
                        .min(free_cap);
                    let requested = ResourceAmount::new(external_available.raw().min(max_uptake))
                        .expect("requested uptake is clamped");

                    let accepted = {
                        let cells = self.world.cells_mut_for_commit();
                        cells.add_resources_limited_by_capacity(index, requested)
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
                let req_amount = config
                    .resource_interaction
                    .metabolism_resource_per_tick
                    .raw();
                let (feasible, _) = run_process(
                    &self.world,
                    index,
                    ProcessId::MetabolismEnergyConversion,
                    req_amount,
                    &mut diagnostics,
                    &mut process_attempts,
                    &mut process_rejections,
                );
                if feasible {
                    let consumed = {
                        let cells = self.world.cells_mut_for_commit();
                        cells.consume_resources(
                            index,
                            config.resource_interaction.metabolism_resource_per_tick,
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

            let mut pairs = Vec::new();
            {
                let cells = self.world.cells();
                self.world
                    .spatial_index()
                    .generate_candidate_pairs(cells, &mut pairs);
            }

            let iterations = config.space.physics_solver_iterations;
            let world_size = config.world.size;

            for _ in 0..iterations {
                // 1. Resolve cell-cell overlaps
                for &(idx_i, idx_j) in &pairs {
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
            let over_capacity = used_capacity.raw() > config.cell.capacity_limit.raw();
            let critical_capacity_exceeded = used_capacity.raw()
                > config.cell.capacity_limit.raw()
                    + config.lifecycle.critical_capacity_overrun.raw();

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
        }

        let decomposed_cells_count = self.world.execute_decomposition_for_dead_cells();

        self.world.environment_mut_for_commit().set_heat(heat_next);
        self.world
            .environment_mut_for_commit()
            .set_waste(waste_next);
        self.world
            .resources_mut_for_commit()
            .decay_or_passive_update();
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
                process_attempts,
                process_rejections,
                divisions_count,
                births_count,
                decomposed_cells_count,
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
        process_attempts: u32,
        process_rejections: u32,
        divisions_count: u32,
        births_count: u32,
        decomposed_cells_count: u32,
    ) -> MetricsSummary {
        let cells = self.world.cells();
        let mut final_internal_resources = 0.0_f32;
        let mut final_used_capacity = 0.0_f32;
        let mut final_free_capacity = 0.0_f32;
        let mut growth_readiness = false;
        let mut alive_cells_count = 0_u32;
        let mut dead_cells_count = 0_u32;

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
            overlap_resolved,
            process_attempts,
            process_rejections,
            alive_cells_count,
            dead_cells_count,
            divisions_count,
            births_count,
            decomposed_cells_count,
        }
    }
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
