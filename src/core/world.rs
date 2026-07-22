use crate::core::action_plan::ActionPlan;
use crate::core::cell_store::{
    CellIndex, CellStore, EnergyBuffer, InitialCellState, LifecycleState,
};
use crate::core::config::{RuntimeConfig, deterministic_genome_decision_offset};
use crate::core::contact::ContactCache;
use crate::core::environment::EnvironmentState;
use crate::core::events::EventBuffer;
use crate::core::fragments::FragmentStore;
use crate::core::genome::{GenomeId, GenomeOutputValue, GenomeState};
use crate::core::genome_bootstrap::instantiate_initial_genome;
use crate::core::ids::ResourceTypeId;
use crate::core::joints::JointStore;
use crate::core::lineage::{
    DivisionLineage, GenomeCopyLineage, GenomeMutationDelta, LineageEventLog,
};
use crate::core::resources::ResourceGrid;
use crate::core::spatial::SpatialIndex;
use crate::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Tick,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInitError {
    InvalidInitialState,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DivisionOutcome {
    pub parent_id: crate::core::ids::CellId,
    pub daughter_a_id: crate::core::ids::CellId,
    pub daughter_b_id: crate::core::ids::CellId,
    pub daughter_a_index: CellIndex,
    pub daughter_b_index: CellIndex,
}

#[derive(Clone, Debug)]
pub struct WorldState {
    tick: Tick,
    config: RuntimeConfig,
    cells: CellStore,
    resources: ResourceGrid,
    environment: EnvironmentState,
    spatial_index: SpatialIndex,
    contact_cache: ContactCache,
    events: EventBuffer,
    lineage_events: LineageEventLog,
    fragments: FragmentStore,
    joints: JointStore,
    genomes: Vec<GenomeState>,
}

impl WorldState {
    pub fn from_config(config: RuntimeConfig) -> Result<Self, WorldInitError> {
        let mut cells = CellStore::with_capacity(config.initial_cells.len());
        if !config.chemistry.resources.is_empty() {
            cells
                .configure_typed_resource_types(
                    (0..config.chemistry.resources.len())
                        .map(|index| ResourceTypeId::from_raw(index as u32))
                        .collect(),
                )
                .map_err(|_| WorldInitError::InvalidInitialState)?;
        }
        if config.initial_cells.len() == 1 {
            cells.insert_initial(InitialCellState {
                position: config.cell.position,
                radius: config.cell.radius,
                energy: EnergyBuffer::new(config.cell.initial_energy, config.cell.energy_capacity),
                resources: if config
                    .initial_typed_resources
                    .first()
                    .is_some_and(|inventory| !inventory.is_empty())
                {
                    ResourceAmount::zero()
                } else {
                    config.cell.initial_resource_amount
                },
                boundary_material: config.cell.initial_boundary_material,
                transport_material: config.cell.initial_transport_material,
                metabolic_material: config.cell.initial_metabolic_material,
                storage_material: config.cell.initial_storage_material,
                synthesis_material: config.cell.initial_synthesis_material,
                structural_material: config.cell.initial_structural_material,
                repair_material: config.cell.initial_repair_material,
                contractile_material: config.cell.initial_contractile_material,
                sensory_material: config.cell.initial_sensory_material,
                capacity_limit: config.cell.capacity_limit,
                temperature: crate::core::units::Temperature::new(25.0),
            });
            for (resource_type, amount) in
                config.initial_typed_resources.first().into_iter().flatten()
            {
                cells
                    .set_typed_resource_amount(CellIndex::from_raw(0), *resource_type, *amount)
                    .map_err(|_| WorldInitError::InvalidInitialState)?;
            }
        } else {
            for (cell_index, cell_config) in config.initial_cells.iter().enumerate() {
                cells.insert_initial(InitialCellState {
                    position: cell_config.position,
                    radius: cell_config.radius,
                    energy: EnergyBuffer::new(
                        cell_config.initial_energy,
                        cell_config.energy_capacity,
                    ),
                    resources: if config
                        .initial_typed_resources
                        .get(cell_index)
                        .is_some_and(|inventory| !inventory.is_empty())
                    {
                        ResourceAmount::zero()
                    } else {
                        cell_config.initial_resource_amount
                    },
                    boundary_material: cell_config.initial_boundary_material,
                    transport_material: cell_config.initial_transport_material,
                    metabolic_material: cell_config.initial_metabolic_material,
                    storage_material: cell_config.initial_storage_material,
                    synthesis_material: cell_config.initial_synthesis_material,
                    structural_material: cell_config.initial_structural_material,
                    repair_material: cell_config.initial_repair_material,
                    contractile_material: cell_config.initial_contractile_material,
                    sensory_material: cell_config.initial_sensory_material,
                    capacity_limit: cell_config.capacity_limit,
                    temperature: crate::core::units::Temperature::new(25.0),
                });
                for (resource_type, amount) in config
                    .initial_typed_resources
                    .get(cell_index)
                    .into_iter()
                    .flatten()
                {
                    cells
                        .set_typed_resource_amount(
                            CellIndex::from_raw(cell_index),
                            *resource_type,
                            *amount,
                        )
                        .map_err(|_| WorldInitError::InvalidInitialState)?;
                }
            }
        }

        let mut genomes = Vec::new();
        let world_seed = config.world.seed.raw();
        for cell_raw in 0..cells.len() {
            let Some(template_id) = config
                .initial_cell_genome_templates
                .get(cell_raw)
                .and_then(|assignment| assignment.as_ref())
            else {
                continue;
            };
            let template = config
                .genome_templates
                .iter()
                .find(|candidate| candidate.id().as_str() == template_id.as_str())
                .ok_or(WorldInitError::InvalidInitialState)?;
            let genome = instantiate_initial_genome(world_seed, cell_raw, template);
            let genome_id = genome.id;
            let cell = CellIndex::from_raw(cell_raw);
            cells.set_genome_id(cell, Some(genome_id));
            cells.set_genome_carrier_amount(cell, genome.carrier.amount);
            genomes.push(genome);
        }

        for cell_raw in 0..cells.len() {
            let cell = CellIndex::from_raw(cell_raw);
            let genome = cells
                .genome_id(cell)
                .and_then(|genome_id| genomes.iter().find(|genome| genome.id == genome_id));
            let plan = ActionPlan::from_genome(genome);
            let cadence = config
                .effective_genome_runtime_cadence_ticks_for_genome(genome)
                .max(1);
            let offset =
                deterministic_genome_decision_offset(world_seed, cells.id_at(cell).raw(), cadence);
            cells.set_action_plan(cell, plan);
            cells.set_genome_decision_offset(cell, offset);
            cells.set_next_genome_decision_due_tick(cell, cadence + offset);
        }

        let mut lineage_events = LineageEventLog::with_capacity(cells.len() + 16);
        for cell in cells.iter_indices() {
            let genome_id = cells.genome_id(cell);
            let genome_template_id = genome_id.and_then(|id| {
                genomes
                    .iter()
                    .find(|genome| genome.id == id)
                    .map(|genome| genome.template_id.clone())
            });
            lineage_events.push_founder_cell(
                Tick::from_raw(0),
                cells.id_at(cell),
                genome_id,
                genome_template_id,
            );
        }

        let mut spatial_index = SpatialIndex::new();
        spatial_index.rebuild(&cells, config.world.size, config.space.spatial_grid_size);
        let mut contact_cache = ContactCache::default();
        contact_cache.rebuild(&cells, &spatial_index);

        let resources = if let Some(layers) = config.prepared_resource_layers.clone() {
            ResourceGrid::new_from_layers(
                config.world.size,
                config.space.spatial_grid_size,
                layers,
                config.resources.optional_decay_rate,
            )
        } else {
            ResourceGrid::new(
                config.world.size,
                config.space.spatial_grid_size,
                config.resources.initial_distribution.clone(),
                config.resources.optional_decay_rate,
            )
        }
        .map_err(|_| WorldInitError::InvalidInitialState)?;

        let environment = EnvironmentState::from_config(&config.environment);

        Ok(Self {
            tick: Tick::from_raw(0),
            config,
            cells,
            resources,
            environment,
            spatial_index,
            contact_cache,
            events: EventBuffer::with_capacity(128),
            lineage_events,
            fragments: FragmentStore::default(),
            joints: JointStore::with_capacity(4),
            genomes,
        })
    }

    pub const fn tick(&self) -> Tick {
        self.tick
    }

    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }

    pub fn cells(&self) -> &CellStore {
        &self.cells
    }

    pub fn cells_mut_for_commit(&mut self) -> &mut CellStore {
        &mut self.cells
    }

    pub fn resources(&self) -> &ResourceGrid {
        &self.resources
    }

    pub fn resources_mut_for_commit(&mut self) -> &mut ResourceGrid {
        &mut self.resources
    }

    pub fn environment(&self) -> EnvironmentState {
        self.environment
    }

    pub fn environment_mut_for_commit(&mut self) -> &mut EnvironmentState {
        &mut self.environment
    }

    pub fn spatial_index(&self) -> &SpatialIndex {
        &self.spatial_index
    }

    pub fn spatial_index_mut_for_commit(&mut self) -> &mut SpatialIndex {
        &mut self.spatial_index
    }

    pub fn rebuild_spatial_index(&mut self) {
        self.spatial_index.rebuild(
            &self.cells,
            self.config.world.size,
            self.config.space.spatial_grid_size,
        );
    }

    pub fn contact_cache(&self) -> &ContactCache {
        &self.contact_cache
    }

    pub fn rebuild_contact_cache(&mut self) {
        self.contact_cache.rebuild(&self.cells, &self.spatial_index);
    }

    pub fn events(&self) -> &EventBuffer {
        &self.events
    }

    pub fn events_mut_for_commit(&mut self) -> &mut EventBuffer {
        &mut self.events
    }

    pub fn lineage_events(&self) -> &LineageEventLog {
        &self.lineage_events
    }

    pub fn lineage_events_mut_for_commit(&mut self) -> &mut LineageEventLog {
        &mut self.lineage_events
    }

    pub fn fragments(&self) -> &FragmentStore {
        &self.fragments
    }

    pub fn fragments_mut_for_commit(&mut self) -> &mut FragmentStore {
        &mut self.fragments
    }

    pub fn joints(&self) -> &JointStore {
        &self.joints
    }

    pub fn joints_mut_for_commit(&mut self) -> &mut JointStore {
        &mut self.joints
    }

    pub fn genome(&self, id: GenomeId) -> Option<&GenomeState> {
        self.genomes.iter().find(|genome| genome.id == id)
    }

    pub(crate) fn advance_tick(&mut self) {
        self.tick = self.tick.next();
        self.spatial_index.rebuild(
            &self.cells,
            self.config.world.size,
            self.config.space.spatial_grid_size,
        );
        self.contact_cache.rebuild(&self.cells, &self.spatial_index);
    }

    pub fn validate_feasibility(
        &self,
        cell_idx: CellIndex,
        action: &crate::core::process::ActionCandidate,
    ) -> crate::core::process::FeasibilityResult {
        use crate::core::process::{
            FeasibilityResult, MaterialCapability, ProcessId, RejectionReason,
        };

        if self.cells.lifecycle_state(cell_idx) == LifecycleState::Dead {
            return FeasibilityResult::Rejected(RejectionReason::LifecycleStateDead);
        }

        match action.process_id {
            ProcessId::MandatoryUpkeep => {
                let current_energy = self.cells.energy(cell_idx).current();
                if current_energy.raw() <= 0.0 {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
                } else {
                    FeasibilityResult::Allowed {
                        accepted_amount: 0.0,
                        energy_cost: 0.0,
                        resource_cost: 0.0,
                    }
                }
            }
            ProcessId::LocalResourceUptake => {
                if !self
                    .cells
                    .has_capability(cell_idx, MaterialCapability::ResourceUptake)
                {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(
                        MaterialCapability::ResourceUptake,
                    ));
                }
                let free_cap = self
                    .cells
                    .effective_free_capacity(
                        cell_idx,
                        self.config.material_effects.storage_capacity_per_unit,
                    )
                    .raw();
                if free_cap <= 0.0 {
                    return FeasibilityResult::Rejected(RejectionReason::InsufficientCapacity);
                }
                let grid_coord = self
                    .resources
                    .coord_for_position(self.cells.position(cell_idx));
                let layer = crate::core::resources::ResourceLayerIndex::from_raw(
                    self.config.resource_interaction.uptake_layer_index,
                );
                let external = self
                    .resources
                    .amount_at(layer, grid_coord)
                    .map(|a| a.raw())
                    .unwrap_or(0.0);
                let accepted = action.requested_amount.min(free_cap).min(external);
                FeasibilityResult::Allowed {
                    accepted_amount: accepted,
                    energy_cost: 0.0,
                    resource_cost: 0.0,
                }
            }
            ProcessId::MetabolismEnergyConversion => {
                if !self
                    .cells
                    .has_capability(cell_idx, MaterialCapability::Metabolism)
                {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(
                        MaterialCapability::Metabolism,
                    ));
                }
                let internal_res = self.cells.generic_resource_amount(cell_idx);
                if internal_res.raw() < action.requested_amount {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientResources)
                } else {
                    let accepted = internal_res.raw().min(action.requested_amount);
                    FeasibilityResult::Allowed {
                        accepted_amount: accepted,
                        energy_cost: 0.0,
                        resource_cost: 0.0,
                    }
                }
            }
            ProcessId::MaterialSynthesis => {
                if !self
                    .cells
                    .has_capability(cell_idx, MaterialCapability::MaterialSynthesis)
                {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(
                        MaterialCapability::MaterialSynthesis,
                    ));
                }
                let cost_res = self.config.synthesis.cost_resource.raw();
                let cost_eng = self.config.synthesis.cost_energy.raw();
                let current_res = self.cells.generic_resource_amount(cell_idx).raw();
                let current_eng = self.cells.energy(cell_idx).current().raw();

                if current_res < cost_res {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientResources)
                } else if current_eng < cost_eng {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
                } else {
                    FeasibilityResult::Allowed {
                        accepted_amount: 1.0,
                        energy_cost: cost_eng,
                        resource_cost: cost_res,
                    }
                }
            }
            ProcessId::ContractileDisplacement => {
                if !self
                    .cells
                    .has_capability(cell_idx, MaterialCapability::Contractility)
                {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(
                        MaterialCapability::Contractility,
                    ));
                }
                let pressure = self.cells.contact_pressure(cell_idx);
                if pressure <= 0.0 {
                    return FeasibilityResult::Rejected(RejectionReason::NoPressure);
                }
                let cost_eng = self.config.contractility.energy_cost.raw();
                let current_eng = self.cells.energy(cell_idx).current().raw();

                if current_eng < cost_eng {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
                } else {
                    FeasibilityResult::Allowed {
                        accepted_amount: 1.0,
                        energy_cost: cost_eng,
                        resource_cost: 0.0,
                    }
                }
            }
            ProcessId::GrowthResourceAllocation => {
                if self.cells.radius(cell_idx).raw()
                    >= self.config.growth.growth_target_radius.raw()
                {
                    return FeasibilityResult::Rejected(RejectionReason::GrowthTargetReached);
                }
                if !self
                    .cells
                    .has_capability(cell_idx, MaterialCapability::StructuralGrowth)
                {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(
                        MaterialCapability::StructuralGrowth,
                    ));
                }
                let cost_res = self.config.growth.growth_cost_resource.raw();
                let cost_eng = self.config.growth.growth_cost_energy.raw();

                let current_res = self.cells.generic_resource_amount(cell_idx).raw();
                let current_eng = self.cells.energy(cell_idx).current().raw();

                if current_res < cost_res {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientResources)
                } else if current_eng < cost_eng {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
                } else {
                    FeasibilityResult::Allowed {
                        accepted_amount: 1.0,
                        energy_cost: cost_eng,
                        resource_cost: cost_res,
                    }
                }
            }
            ProcessId::Division => {
                if !self.config.division.enabled {
                    return FeasibilityResult::Rejected(RejectionReason::ProcessDisabled);
                }
                if self.cells.genome_id(cell_idx).is_some()
                    && self.cells.copied_genome_id(cell_idx).is_none()
                {
                    return FeasibilityResult::Rejected(RejectionReason::MissingGenomeCopy);
                }

                let radius = self.cells.radius(cell_idx).raw();
                let target = self.config.growth.growth_target_radius.raw();
                if radius < target {
                    return FeasibilityResult::Rejected(RejectionReason::RadiusBelowTarget);
                }

                let pressure = self.cells.contact_pressure(cell_idx);
                let max_pressure = self.config.growth.max_division_pressure;
                if pressure > max_pressure {
                    return FeasibilityResult::Rejected(RejectionReason::PressureTooHigh);
                }

                let current_eng = self.cells.energy(cell_idx).current().raw();
                let cost_eng = self.config.division.energy_cost.raw();
                if current_eng < cost_eng {
                    return FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy);
                }

                FeasibilityResult::Allowed {
                    accepted_amount: 1.0,
                    energy_cost: cost_eng,
                    resource_cost: 0.0,
                }
            }
            ProcessId::PassiveContactExchange => FeasibilityResult::Allowed {
                accepted_amount: action.requested_amount,
                energy_cost: 0.0,
                resource_cost: 0.0,
            },
            ProcessId::JointCreate => {
                if !self.config.joints.enabled {
                    return FeasibilityResult::Rejected(RejectionReason::ProcessDisabled);
                }
                if !self
                    .cells
                    .has_capability(cell_idx, MaterialCapability::BoundaryPermeability)
                {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(
                        MaterialCapability::BoundaryPermeability,
                    ));
                }
                if !self
                    .cells
                    .has_capability(cell_idx, MaterialCapability::StructuralGrowth)
                {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(
                        MaterialCapability::StructuralGrowth,
                    ));
                }
                if self.cells.structural_material(cell_idx).raw()
                    < self.config.joints.creation_material_cost.raw() * 0.5
                {
                    return FeasibilityResult::Rejected(RejectionReason::InsufficientMaterial);
                }
                FeasibilityResult::Allowed {
                    accepted_amount: 1.0,
                    energy_cost: self.config.joints.creation_energy_cost.raw() * 0.5,
                    resource_cost: self.config.joints.creation_resource_cost.raw() * 0.5,
                }
            }
            ProcessId::JointRepair => FeasibilityResult::Rejected(RejectionReason::ProcessDisabled),
            ProcessId::GenomeCopying => {
                if !self.config.genome_copying.enabled {
                    return FeasibilityResult::Rejected(RejectionReason::ProcessDisabled);
                }
                if self.cells.genome_id(cell_idx).is_none() {
                    return FeasibilityResult::Rejected(RejectionReason::MissingGenomeCopy);
                }
                if self.cells.copied_genome_id(cell_idx).is_some() {
                    return FeasibilityResult::Rejected(RejectionReason::GrowthTargetReached);
                }
                if !self
                    .cells
                    .has_capability(cell_idx, MaterialCapability::GenomeCopying)
                {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(
                        MaterialCapability::GenomeCopying,
                    ));
                }
                let remaining = (1.0 - self.cells.genome_copy_progress(cell_idx)).max(0.0);
                if remaining <= 0.0 {
                    return FeasibilityResult::Rejected(RejectionReason::GrowthTargetReached);
                }
                let progress = self
                    .config
                    .genome_copying
                    .progress_per_step
                    .min(action.requested_amount)
                    .min(remaining);
                if progress <= 0.0 {
                    return FeasibilityResult::Rejected(RejectionReason::ProcessDisabled);
                }
                let energy_cost = self.config.genome_copying.energy_cost_per_step.raw();
                let resource_cost = self
                    .config
                    .genome_copying
                    .carrier_resource_cost_per_step
                    .raw();
                if self.cells.energy(cell_idx).current().raw() < energy_cost {
                    return FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy);
                }
                if self.cells.generic_resource_amount(cell_idx).raw() < resource_cost {
                    return FeasibilityResult::Rejected(RejectionReason::InsufficientResources);
                }
                if self
                    .cells
                    .effective_free_capacity(
                        cell_idx,
                        self.config.material_effects.storage_capacity_per_unit,
                    )
                    .raw()
                    < resource_cost
                {
                    return FeasibilityResult::Rejected(RejectionReason::InsufficientCapacity);
                }
                FeasibilityResult::Allowed {
                    accepted_amount: progress,
                    energy_cost,
                    resource_cost,
                }
            }
            ProcessId::RepairBoundary => {
                if !self.config.chemistry.repair.enabled {
                    return FeasibilityResult::Rejected(RejectionReason::ProcessDisabled);
                }
                if !self
                    .cells
                    .has_capability(cell_idx, MaterialCapability::Repair)
                {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(
                        MaterialCapability::Repair,
                    ));
                }

                let boundary_damage = self
                    .cells
                    .material_damage(cell_idx, crate::core::materials::MaterialSlot::Boundary);
                if boundary_damage <= 0.0 {
                    return FeasibilityResult::Rejected(RejectionReason::MissingTargetDamage);
                }

                let current_energy = self.cells.energy(cell_idx).current().raw();
                let energy_cost = self.config.chemistry.repair.energy_cost;
                if current_energy < energy_cost {
                    return FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy);
                }

                let available_repair_resource = repair_resource_type_id(&self.config)
                    .and_then(|resource_type| {
                        self.cells
                            .typed_resource_amount(cell_idx, resource_type)
                            .ok()
                            .map(|amount| amount.raw())
                    })
                    .unwrap_or_else(|| self.cells.generic_resource_amount(cell_idx).raw());

                let requested = action
                    .requested_amount
                    .min(self.config.chemistry.repair.max_amount_per_tick)
                    .min(boundary_damage)
                    .min(self.cells.repair_material(cell_idx).raw())
                    .min(available_repair_resource);
                if requested <= 0.0 {
                    return FeasibilityResult::Rejected(RejectionReason::InsufficientResources);
                }

                FeasibilityResult::Allowed {
                    accepted_amount: requested,
                    energy_cost,
                    resource_cost: requested,
                }
            }
        }
    }

    pub fn execute_genome_copying(
        &mut self,
        cell_idx: CellIndex,
        action: &crate::core::process::ActionCandidate,
    ) -> Result<(), String> {
        let feasibility = self.validate_feasibility(cell_idx, action);
        let (accepted_amount, energy_cost, resource_cost) = match feasibility {
            crate::core::process::FeasibilityResult::Allowed {
                accepted_amount,
                energy_cost,
                resource_cost,
            } => (accepted_amount, energy_cost, resource_cost),
            crate::core::process::FeasibilityResult::Rejected(reason) => {
                return Err(format!("{:?}", reason));
            }
        };

        let energy = self.cells.energy(cell_idx);
        let next_energy = EnergyAmount::new((energy.current().raw() - energy_cost).max(0.0))
            .expect("genome copying energy cost is feasibility-checked");
        self.cells
            .set_energy(cell_idx, EnergyBuffer::new(next_energy, energy.capacity()));
        let consumed = self
            .cells
            .consume_resources(cell_idx, ResourceAmount::new(resource_cost).unwrap());
        let next_carrier_amount =
            self.cells.copied_genome_carrier_amount(cell_idx) + consumed.raw();
        self.cells
            .set_copied_genome_carrier_amount(cell_idx, next_carrier_amount);

        let next_progress = (self.cells.genome_copy_progress(cell_idx) + accepted_amount).min(1.0);
        self.cells.set_genome_copy_progress(cell_idx, next_progress);
        if next_progress < 1.0 || self.cells.copied_genome_id(cell_idx).is_some() {
            return Ok(());
        }

        let parent_id = self
            .cells
            .genome_id(cell_idx)
            .ok_or_else(|| "MissingGenomeCopy".to_string())?;
        let parent = self
            .genomes
            .iter()
            .find(|genome| genome.id == parent_id)
            .cloned()
            .ok_or_else(|| "MissingGenomeCopy".to_string())?;
        let copied_id = self.next_genome_id();
        let mut copied = parent.clone();
        copied.id = copied_id;
        copied.outputs = copied
            .outputs
            .into_iter()
            .enumerate()
            .map(|(offset, (output_id, value))| {
                let mutated = mutate_genome_output_value(
                    self.config.world.seed.raw(),
                    self.tick.raw(),
                    self.cells.id_at(cell_idx).raw(),
                    parent_id.raw(),
                    offset as u32,
                    value.raw(),
                    self.config.genome_copying.mutation_rate,
                    self.config.genome_copying.mutation_step,
                );
                (output_id, GenomeOutputValue::new(mutated))
            })
            .collect();
        let mutation_deltas = parent
            .outputs
            .iter()
            .zip(copied.outputs.iter())
            .filter_map(|((parent_output_id, before), (copied_output_id, after))| {
                debug_assert_eq!(parent_output_id, copied_output_id);
                (before.raw() != after.raw()).then_some(GenomeMutationDelta {
                    output_id: *copied_output_id,
                    before: before.raw(),
                    after: after.raw(),
                })
            })
            .collect();
        let copy_lineage = GenomeCopyLineage {
            cell_id: self.cells.id_at(cell_idx),
            parent_genome_id: parent_id,
            copied_genome_id: copied_id,
            genome_template_id: copied.template_id.clone(),
            carrier_material_id: copied.carrier.material_id.clone(),
            carrier_amount: copied.carrier.amount,
            carrier_integrity: copied.carrier.integrity,
            mutation_deltas,
        };
        self.genomes.push(copied);
        self.cells.set_copied_genome_id(cell_idx, Some(copied_id));
        self.lineage_events
            .push_genome_copied(self.tick, copy_lineage);
        Ok(())
    }

    pub fn execute_growth(
        &mut self,
        cell_idx: CellIndex,
        _action: &crate::core::process::ActionCandidate,
    ) -> Result<(), String> {
        let config = self.config.clone();

        if self.cells.radius(cell_idx).raw() >= config.growth.growth_target_radius.raw() {
            return Ok(());
        }

        let cost_res = config.growth.growth_cost_resource.raw();
        let cost_eng = config.growth.growth_cost_energy.raw();

        let current_res = self.cells.generic_resource_amount(cell_idx).raw();
        let current_eng = self.cells.energy(cell_idx).current().raw();

        if current_res < cost_res || current_eng < cost_eng {
            return Err("Insufficient resources or energy".to_string());
        }

        // Deduct cost
        self.cells.set_resources(
            cell_idx,
            ResourceAmount::new(current_res - cost_res).unwrap(),
        );
        let next_energy = EnergyAmount::new(current_eng - cost_eng).unwrap();
        self.cells.set_energy(
            cell_idx,
            EnergyBuffer::new(next_energy, self.cells.energy(cell_idx).capacity()),
        );

        // Only structural material increases — capability derives from material amount only
        let old_structural = self.cells.structural_material(cell_idx).raw();
        let growth_output = (baseline_process_level(old_structural)
            * config.material_effects.structural_growth_per_unit)
            .max(0.0);
        let new_structural = old_structural + growth_output;
        self.cells
            .set_structural_material(cell_idx, MaterialAmount::new(new_structural).unwrap());

        // Radius follows absolute accepted structural output so higher structural capacity
        // produces a measurable directional effect instead of only preserving ratios.
        let old_radius = self.cells.radius(cell_idx).raw();
        let computed_radius_val = if old_structural > 0.0 {
            old_radius * (1.0 + growth_output).sqrt()
        } else {
            old_radius
        };
        let target_radius = config.growth.growth_target_radius.raw();
        let new_radius_val = computed_radius_val.min(target_radius).max(old_radius);
        let new_radius = Radius::new(new_radius_val).unwrap();
        self.cells.set_radius(cell_idx, new_radius);

        // Update capacity limit scaling with radius area increase
        let old_cap = self.cells.capacity_limit(cell_idx).raw();
        let new_cap_val = if old_radius > 0.0 {
            old_cap * (new_radius_val / old_radius).powi(2)
        } else {
            old_cap
        };
        self.cells
            .set_capacity_limit(cell_idx, CapacityAmount::new(new_cap_val).unwrap());

        Ok(())
    }

    pub fn execute_division(
        &mut self,
        cell_idx: CellIndex,
        action: &crate::core::process::ActionCandidate,
    ) -> Result<DivisionOutcome, String> {
        let feasibility = self.validate_feasibility(cell_idx, action);
        let (cost_eng, _cost_res) = match feasibility {
            crate::core::process::FeasibilityResult::Allowed {
                energy_cost,
                resource_cost,
                ..
            } => (energy_cost, resource_cost),
            crate::core::process::FeasibilityResult::Rejected(reason) => {
                return Err(format!("{:?}", reason));
            }
        };

        let ratio = self.config.division.split_ratio;
        let inv_ratio = 1.0 - ratio;
        let loss_keep = 1.0 - self.config.division.partition_loss_fraction;
        let current_tick = self.tick;
        self.joints.break_for_endpoint(cell_idx, current_tick);

        let parent_id = self.cells.id_at(cell_idx);
        let parent_genome_id = self.cells.genome_id(cell_idx);
        let copied_genome_id = self.cells.copied_genome_id(cell_idx);
        let parent_pos = self.cells.position(cell_idx);
        let parent_radius = self.cells.radius(cell_idx).raw();
        let parent_energy = self.cells.energy(cell_idx);
        let energy_after_cost = (parent_energy.current().raw() - cost_eng).max(0.0);

        let a_energy = EnergyAmount::new(energy_after_cost * ratio).unwrap();
        let b_energy = EnergyAmount::new(energy_after_cost * inv_ratio).unwrap();
        let a_capacity = EnergyAmount::new(parent_energy.capacity().raw() * ratio).unwrap();
        let b_capacity = EnergyAmount::new(parent_energy.capacity().raw() * inv_ratio).unwrap();

        let split_resource = |amount: ResourceAmount, r: f32| {
            ResourceAmount::new(amount.raw() * r * loss_keep).unwrap()
        };
        let split_material = |amount: MaterialAmount, r: f32| {
            MaterialAmount::new(amount.raw() * r * loss_keep).unwrap()
        };

        let parent_resources = self.cells.generic_resource_amount(cell_idx);
        let parent_boundary = self.cells.boundary_material(cell_idx);
        let parent_transport = self.cells.transport_material(cell_idx);
        let parent_metabolic = self.cells.metabolic_material(cell_idx);
        let parent_storage = self.cells.storage_material(cell_idx);
        let parent_synthesis = self.cells.synthesis_material(cell_idx);
        let parent_structural = self.cells.structural_material(cell_idx);
        let parent_repair = self.cells.repair_material(cell_idx);
        let parent_contractile = self.cells.contractile_material(cell_idx);
        let parent_sensory = self.cells.sensory_material(cell_idx);
        let parent_capacity_limit = self.cells.capacity_limit(cell_idx);
        let parent_temperature = self.cells.temperature(cell_idx);

        // Daughter radii: start at max(parent_radius * sqrt(ratio), min_daughter_radius)
        let min_daughter_radius = self.config.division.min_daughter_radius.raw();
        let a_radius_val = (parent_radius * ratio.sqrt()).max(min_daughter_radius);
        let b_radius_val = (parent_radius * inv_ratio.sqrt()).max(min_daughter_radius);
        let a_radius = Radius::new(a_radius_val).unwrap();
        let b_radius = Radius::new(b_radius_val).unwrap();

        // Spacing offset left/right along X
        let a_x = parent_pos.x() - (a_radius_val + self.config.division.daughter_spacing);
        let b_x = parent_pos.x() + (b_radius_val + self.config.division.daughter_spacing);

        let width = self.config.world.size.width();
        let height = self.config.world.size.height();

        let clamp_pos = |x: f32, y: f32, r: f32| {
            let clamped_x = x.clamp(r, width - r);
            let clamped_y = y.clamp(r, height - r);
            Position::new(clamped_x, clamped_y)
        };

        let a_position = clamp_pos(a_x, parent_pos.y(), a_radius_val);
        let b_position = clamp_pos(b_x, parent_pos.y(), b_radius_val);

        // Modify parent state to become daughter A
        self.cells
            .set_energy(cell_idx, EnergyBuffer::new(a_energy, a_capacity));
        self.cells
            .set_resources(cell_idx, split_resource(parent_resources, ratio));
        self.cells
            .set_boundary_material(cell_idx, split_material(parent_boundary, ratio));
        self.cells
            .set_transport_material(cell_idx, split_material(parent_transport, ratio));
        self.cells
            .set_metabolic_material(cell_idx, split_material(parent_metabolic, ratio));
        self.cells
            .set_storage_material(cell_idx, split_material(parent_storage, ratio));
        self.cells
            .set_synthesis_material(cell_idx, split_material(parent_synthesis, ratio));
        self.cells
            .set_structural_material(cell_idx, split_material(parent_structural, ratio));
        self.cells
            .set_repair_material(cell_idx, split_material(parent_repair, ratio));
        self.cells
            .set_contractile_material(cell_idx, split_material(parent_contractile, ratio));
        self.cells
            .set_sensory_material(cell_idx, split_material(parent_sensory, ratio));
        self.cells.set_radius(cell_idx, a_radius);
        self.cells.set_capacity_limit(
            cell_idx,
            CapacityAmount::new(parent_capacity_limit.raw() * ratio).unwrap(),
        );
        self.cells.set_position(cell_idx, a_position);
        self.cells
            .set_lifecycle_state(cell_idx, LifecycleState::Alive);
        self.cells
            .set_runtime_flags(cell_idx, crate::core::cell_store::RuntimeFlags::default());

        // Insert daughter B
        let daughter_b_state = InitialCellState {
            position: b_position,
            radius: b_radius,
            energy: EnergyBuffer::new(b_energy, b_capacity),
            resources: split_resource(parent_resources, inv_ratio),
            boundary_material: split_material(parent_boundary, inv_ratio),
            transport_material: split_material(parent_transport, inv_ratio),
            metabolic_material: split_material(parent_metabolic, inv_ratio),
            storage_material: split_material(parent_storage, inv_ratio),
            synthesis_material: split_material(parent_synthesis, inv_ratio),
            structural_material: split_material(parent_structural, inv_ratio),
            repair_material: split_material(parent_repair, inv_ratio),
            contractile_material: split_material(parent_contractile, inv_ratio),
            sensory_material: split_material(parent_sensory, inv_ratio),
            capacity_limit: CapacityAmount::new(parent_capacity_limit.raw() * inv_ratio).unwrap(),
            temperature: parent_temperature,
        };

        let daughter_b_id = self.cells.insert_partitioned_daughter(daughter_b_state);
        let daughter_b_index = self.cells.resolve_id_cold(daughter_b_id).unwrap();
        if parent_genome_id.is_some() {
            self.cells.set_genome_id(daughter_b_index, copied_genome_id);
            let copied_carrier = copied_genome_id
                .and_then(|genome_id| self.genome(genome_id))
                .map(|genome| genome.carrier.amount)
                .unwrap_or(0.0);
            self.cells
                .set_genome_carrier_amount(daughter_b_index, copied_carrier);
            self.cells.reset_genome_copy_state(cell_idx);
            self.cells.reset_genome_copy_state(daughter_b_index);
        }
        self.cells
            .partition_typed_resources(cell_idx, daughter_b_index, ratio, loss_keep)
            .map_err(|err| format!("{:?}", err))?;

        for daughter in [cell_idx, daughter_b_index] {
            let genome = self
                .cells
                .genome_id(daughter)
                .and_then(|genome_id| self.genomes.iter().find(|genome| genome.id == genome_id));
            let plan = ActionPlan::from_genome(genome);
            let cadence = self
                .config
                .effective_genome_runtime_cadence_ticks_for_genome(genome)
                .max(1);
            let offset = deterministic_genome_decision_offset(
                self.config.world.seed.raw(),
                self.cells.id_at(daughter).raw(),
                cadence,
            );
            self.cells.set_action_plan(daughter, plan);
            self.cells.set_genome_decision_offset(daughter, offset);
            self.cells
                .set_next_genome_decision_due_tick(daughter, self.tick.raw() + 1);
        }

        self.lineage_events.push_cell_divided(
            self.tick,
            DivisionLineage {
                parent_cell_id: parent_id,
                daughter_a_cell_id: parent_id,
                daughter_b_cell_id: daughter_b_id,
                parent_genome_id,
                daughter_a_genome_id: self.cells.genome_id(cell_idx),
                daughter_b_genome_id: self.cells.genome_id(daughter_b_index),
                split_ratio: self.config.division.split_ratio,
                partition_loss_fraction: self.config.division.partition_loss_fraction,
            },
        );

        Ok(DivisionOutcome {
            parent_id,
            daughter_a_id: parent_id,
            daughter_b_id,
            daughter_a_index: cell_idx,
            daughter_b_index,
        })
    }

    fn next_genome_id(&self) -> GenomeId {
        let next = self
            .genomes
            .iter()
            .map(|genome| genome.id.raw())
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        GenomeId::from_raw(next)
    }

    pub fn execute_synthesis(&mut self, cell_idx: CellIndex) -> Result<(), String> {
        let cost_res = self.config.synthesis.cost_resource.raw();
        let cost_eng = self.config.synthesis.cost_energy.raw();
        let current_res = self.cells.generic_resource_amount(cell_idx).raw();
        let current_eng = self.cells.energy(cell_idx).current().raw();

        if current_res < cost_res || current_eng < cost_eng {
            return Err("Insufficient resources or energy".to_string());
        }

        self.cells.set_resources(
            cell_idx,
            ResourceAmount::new(current_res - cost_res).unwrap(),
        );
        let next_energy = EnergyAmount::new(current_eng - cost_eng).unwrap();
        self.cells.set_energy(
            cell_idx,
            EnergyBuffer::new(next_energy, self.cells.energy(cell_idx).capacity()),
        );

        // Synthesize structural material by default
        let old_structural = self.cells.structural_material(cell_idx).raw();
        self.cells
            .set_structural_material(cell_idx, MaterialAmount::new(old_structural + 1.0).unwrap());

        Ok(())
    }

    pub fn execute_displacement(&mut self, cell_idx: CellIndex) -> Result<(), String> {
        let cost_eng = self.config.contractility.energy_cost.raw();
        let current_eng = self.cells.energy(cell_idx).current().raw();
        if current_eng < cost_eng {
            return Err("Insufficient energy".to_string());
        }

        let pressure = self.cells.contact_pressure(cell_idx);
        if pressure <= 0.0 {
            return Err("No pressure present".to_string());
        }

        // Deduct energy
        let next_energy = EnergyAmount::new(current_eng - cost_eng).unwrap();
        self.cells.set_energy(
            cell_idx,
            EnergyBuffer::new(next_energy, self.cells.energy(cell_idx).capacity()),
        );

        // Calculate push vector away from colliding neighbors
        let cell_pos = self.cells.position(cell_idx);
        let cell_rad = self.cells.radius(cell_idx).raw();
        let mut push_x = 0.0;
        let mut push_y = 0.0;

        for i in 0..self.cells.len() {
            let other_idx = CellIndex::from_raw(i);
            if other_idx == cell_idx
                || self.cells.lifecycle_state(other_idx) == LifecycleState::Dead
            {
                continue;
            }
            let other_pos = self.cells.position(other_idx);
            let other_rad = self.cells.radius(other_idx).raw();
            let dx = cell_pos.x() - other_pos.x();
            let dy = cell_pos.y() - other_pos.y();
            let dist = (dx * dx + dy * dy).sqrt();
            let sum_rad = cell_rad + other_rad;
            if dist < sum_rad && dist > 0.001 {
                let overlap = sum_rad - dist;
                push_x += (dx / dist) * overlap;
                push_y += (dy / dist) * overlap;
            }
        }

        // Scale by contractile capability (mass) and force factor config
        let contractility_mass = self.cells.contractile_material(cell_idx).raw();
        let shift_factor = baseline_process_level(contractility_mass)
            * self.config.contractility.force_factor
            * self.config.material_effects.contractile_force_per_unit;
        let final_x = cell_pos.x() + push_x * shift_factor;
        let final_y = cell_pos.y() + push_y * shift_factor;

        // Clamp to world boundaries
        let max_w = self.config.world.size.width();
        let max_h = self.config.world.size.height();
        let clamped_x = final_x.clamp(cell_rad, max_w - cell_rad);
        let clamped_y = final_y.clamp(cell_rad, max_h - cell_rad);

        self.cells
            .set_position(cell_idx, Position::new(clamped_x, clamped_y));
        Ok(())
    }

    pub fn execute_decomposition_for_dead_cells(&mut self) -> u32 {
        if !self.config.decomposition.enabled {
            return 0;
        }

        let mut fully_decomposed_count = 0_u32;
        let resources_per_tick = self.config.decomposition.resources_per_tick.raw();
        let materials_per_tick = self.config.decomposition.materials_per_tick.raw();
        let layer = crate::core::resources::ResourceLayerIndex::from_raw(
            self.config.decomposition.resource_layer_index,
        );
        let len = self.cells.len();

        for i in 0..len {
            let idx = CellIndex::from_raw(i);
            if self.cells.lifecycle_state(idx) != LifecycleState::Dead {
                continue;
            }
            if self.cells.runtime_flags(idx).inert {
                continue;
            }

            let pos = self.cells.position(idx);
            let grid_coord = self.resources.coord_for_position(pos);

            // 1. Decompose internal resources
            let internal_res = self.cells.generic_resource_amount(idx).raw();
            let remaining_decompose_res = resources_per_tick;
            let actual_decompose_res = internal_res.min(remaining_decompose_res);

            if actual_decompose_res > 0.0 {
                self.cells.set_resources(
                    idx,
                    ResourceAmount::new(internal_res - actual_decompose_res).unwrap(),
                );
                let current_grid_res = self
                    .resources
                    .amount_at(layer, grid_coord)
                    .map(|a| a.raw())
                    .unwrap_or(0.0);
                let _ = self.resources.set_amount_at(
                    layer,
                    grid_coord,
                    ResourceAmount::new(current_grid_res + actual_decompose_res).unwrap(),
                );
            }

            // 2. Decompose materials
            let mut remaining_decompose_mat = materials_per_tick;
            let mut decomposed_mat_sum = 0.0;

            // boundary
            let val = self.cells.boundary_material(idx).raw();
            if val > 0.0 && remaining_decompose_mat > 0.0 {
                let to_decompose = val.min(remaining_decompose_mat);
                decomposed_mat_sum += to_decompose;
                remaining_decompose_mat -= to_decompose;
                self.cells
                    .set_boundary_material(idx, MaterialAmount::new(val - to_decompose).unwrap());
                self.fragments
                    .create(crate::core::fragments::MaterialFragment::new(
                        crate::core::materials::MaterialSlot::Boundary.material_type_id(),
                        MaterialAmount::new(to_decompose).unwrap(),
                        pos,
                        self.tick,
                    ));
            }

            // transport
            let val = self.cells.transport_material(idx).raw();
            if val > 0.0 && remaining_decompose_mat > 0.0 {
                let to_decompose = val.min(remaining_decompose_mat);
                decomposed_mat_sum += to_decompose;
                remaining_decompose_mat -= to_decompose;
                self.cells
                    .set_transport_material(idx, MaterialAmount::new(val - to_decompose).unwrap());
                self.fragments
                    .create(crate::core::fragments::MaterialFragment::new(
                        crate::core::materials::MaterialSlot::Transport.material_type_id(),
                        MaterialAmount::new(to_decompose).unwrap(),
                        pos,
                        self.tick,
                    ));
            }

            // metabolic
            let val = self.cells.metabolic_material(idx).raw();
            if val > 0.0 && remaining_decompose_mat > 0.0 {
                let to_decompose = val.min(remaining_decompose_mat);
                decomposed_mat_sum += to_decompose;
                remaining_decompose_mat -= to_decompose;
                self.cells
                    .set_metabolic_material(idx, MaterialAmount::new(val - to_decompose).unwrap());
                self.fragments
                    .create(crate::core::fragments::MaterialFragment::new(
                        crate::core::materials::MaterialSlot::Metabolic.material_type_id(),
                        MaterialAmount::new(to_decompose).unwrap(),
                        pos,
                        self.tick,
                    ));
            }

            // storage
            let val = self.cells.storage_material(idx).raw();
            if val > 0.0 && remaining_decompose_mat > 0.0 {
                let to_decompose = val.min(remaining_decompose_mat);
                decomposed_mat_sum += to_decompose;
                remaining_decompose_mat -= to_decompose;
                self.cells
                    .set_storage_material(idx, MaterialAmount::new(val - to_decompose).unwrap());
                self.fragments
                    .create(crate::core::fragments::MaterialFragment::new(
                        crate::core::materials::MaterialSlot::Storage.material_type_id(),
                        MaterialAmount::new(to_decompose).unwrap(),
                        pos,
                        self.tick,
                    ));
            }

            // synthesis
            let val = self.cells.synthesis_material(idx).raw();
            if val > 0.0 && remaining_decompose_mat > 0.0 {
                let to_decompose = val.min(remaining_decompose_mat);
                decomposed_mat_sum += to_decompose;
                remaining_decompose_mat -= to_decompose;
                self.cells
                    .set_synthesis_material(idx, MaterialAmount::new(val - to_decompose).unwrap());
                self.fragments
                    .create(crate::core::fragments::MaterialFragment::new(
                        crate::core::materials::MaterialSlot::Synthesis.material_type_id(),
                        MaterialAmount::new(to_decompose).unwrap(),
                        pos,
                        self.tick,
                    ));
            }

            // structural
            let val = self.cells.structural_material(idx).raw();
            if val > 0.0 && remaining_decompose_mat > 0.0 {
                let to_decompose = val.min(remaining_decompose_mat);
                decomposed_mat_sum += to_decompose;
                remaining_decompose_mat -= to_decompose;
                self.cells
                    .set_structural_material(idx, MaterialAmount::new(val - to_decompose).unwrap());
                self.fragments
                    .create(crate::core::fragments::MaterialFragment::new(
                        crate::core::materials::MaterialSlot::Structural.material_type_id(),
                        MaterialAmount::new(to_decompose).unwrap(),
                        pos,
                        self.tick,
                    ));
            }

            // repair
            let val = self.cells.repair_material(idx).raw();
            if val > 0.0 && remaining_decompose_mat > 0.0 {
                let to_decompose = val.min(remaining_decompose_mat);
                decomposed_mat_sum += to_decompose;
                remaining_decompose_mat -= to_decompose;
                self.cells
                    .set_repair_material(idx, MaterialAmount::new(val - to_decompose).unwrap());
                self.fragments
                    .create(crate::core::fragments::MaterialFragment::new(
                        crate::core::materials::MaterialSlot::Repair.material_type_id(),
                        MaterialAmount::new(to_decompose).unwrap(),
                        pos,
                        self.tick,
                    ));
            }

            // contractile
            let val = self.cells.contractile_material(idx).raw();
            if val > 0.0 && remaining_decompose_mat > 0.0 {
                let to_decompose = val.min(remaining_decompose_mat);
                decomposed_mat_sum += to_decompose;
                remaining_decompose_mat -= to_decompose;
                self.cells.set_contractile_material(
                    idx,
                    MaterialAmount::new(val - to_decompose).unwrap(),
                );
                self.fragments
                    .create(crate::core::fragments::MaterialFragment::new(
                        crate::core::materials::MaterialSlot::Contractile.material_type_id(),
                        MaterialAmount::new(to_decompose).unwrap(),
                        pos,
                        self.tick,
                    ));
            }

            // sensory
            let val = self.cells.sensory_material(idx).raw();
            if val > 0.0 && remaining_decompose_mat > 0.0 {
                let to_decompose = val.min(remaining_decompose_mat);
                decomposed_mat_sum += to_decompose;
                self.cells
                    .set_sensory_material(idx, MaterialAmount::new(val - to_decompose).unwrap());
                self.fragments
                    .create(crate::core::fragments::MaterialFragment::new(
                        crate::core::materials::MaterialSlot::Sensory.material_type_id(),
                        MaterialAmount::new(to_decompose).unwrap(),
                        pos,
                        self.tick,
                    ));
            }

            let _ = decomposed_mat_sum;

            // 3. Check if empty
            if self.cells.resource_amount(idx).raw() == 0.0
                && self.cells.total_materials(idx).raw() == 0.0
            {
                let mut flags = self.cells.runtime_flags(idx);
                flags.inert = true;
                self.cells.set_runtime_flags(idx, flags);
                fully_decomposed_count += 1;
                // Emit event
                let current_tick = self.tick;
                let cell_id = self.cells.id_at(idx);
                self.events.push(
                    current_tick,
                    crate::core::events::EventKind::CellDecomposed,
                    Some(cell_id),
                );
            }
        }

        fully_decomposed_count
    }
}

fn baseline_process_level(raw_level: f32) -> f32 {
    if raw_level > 0.0 {
        raw_level.max(1.0)
    } else {
        0.0
    }
}

fn mutate_genome_output_value(
    seed: u64,
    tick: u64,
    cell_id: u32,
    parent_genome_id: u32,
    output_offset: u32,
    value: f32,
    mutation_rate: f32,
    mutation_step: f32,
) -> f32 {
    if mutation_rate <= 0.0 || mutation_step <= 0.0 {
        return value;
    }
    let mut hash = seed
        ^ tick.wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ u64::from(cell_id).wrapping_mul(0xBF58_476D_1CE4_E5B9)
        ^ u64::from(parent_genome_id).wrapping_mul(0x94D0_49BB_1331_11EB)
        ^ u64::from(output_offset);
    hash ^= hash >> 30;
    hash = hash.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    hash ^= hash >> 27;
    hash = hash.wrapping_mul(0x94D0_49BB_1331_11EB);
    hash ^= hash >> 31;
    let sample = (hash >> 40) as f32 / (1_u32 << 24) as f32;
    if sample >= mutation_rate.clamp(0.0, 1.0) {
        return value;
    }
    let sign = if hash & 1 == 0 { -1.0 } else { 1.0 };
    (value + sign * mutation_step.clamp(0.0, 1.0)).clamp(-1.0, 1.0)
}

pub(crate) fn repair_resource_type_id(
    config: &crate::core::config::RuntimeConfig,
) -> Option<crate::core::ids::ResourceTypeId> {
    let repair_resource = config
        .chemistry
        .materials
        .iter()
        .find(|material| !material.repair_resource.is_empty())
        .map(|material| material.repair_resource.as_str())?;
    config
        .chemistry
        .resources
        .iter()
        .position(|resource| resource.id == repair_resource)
        .map(|index| crate::core::ids::ResourceTypeId::from_raw(index as u32))
}
