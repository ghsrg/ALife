use crate::core::cell_store::{
    CellIndex, CellStore, EnergyBuffer, InitialCellState, LifecycleState,
};
use crate::core::config::RuntimeConfig;
use crate::core::environment::EnvironmentState;
use crate::core::events::EventBuffer;
use crate::core::resources::ResourceGrid;
use crate::core::spatial::SpatialIndex;
use crate::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Radius, ResourceAmount, Tick,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInitError {
    InvalidInitialState,
}

#[derive(Clone, Debug)]
pub struct WorldState {
    tick: Tick,
    config: RuntimeConfig,
    cells: CellStore,
    resources: ResourceGrid,
    environment: EnvironmentState,
    spatial_index: SpatialIndex,
    events: EventBuffer,
}

impl WorldState {
    pub fn from_config(config: RuntimeConfig) -> Result<Self, WorldInitError> {
        let mut cells = CellStore::with_capacity(config.initial_cells.len());
        if config.initial_cells.len() == 1 {
            cells.insert_initial(InitialCellState {
                position: config.cell.position,
                radius: config.cell.radius,
                energy: EnergyBuffer::new(config.cell.initial_energy, config.cell.energy_capacity),
                resources: config.cell.initial_resource_amount,
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
        } else {
            for cell_config in &config.initial_cells {
                cells.insert_initial(InitialCellState {
                    position: cell_config.position,
                    radius: cell_config.radius,
                    energy: EnergyBuffer::new(
                        cell_config.initial_energy,
                        cell_config.energy_capacity,
                    ),
                    resources: cell_config.initial_resource_amount,
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
            }
        }

        let mut spatial_index = SpatialIndex::new();
        spatial_index.rebuild(&cells, config.world.size, config.space.spatial_grid_size);

        let resources = ResourceGrid::new(
            config.world.size,
            config.space.spatial_grid_size,
            config.resources.initial_distribution.clone(),
            config.resources.optional_decay_rate,
        )
        .map_err(|_| WorldInitError::InvalidInitialState)?;

        let environment = EnvironmentState::from_config(&config.environment);

        Ok(Self {
            tick: Tick::from_raw(0),
            config,
            cells,
            resources,
            environment,
            spatial_index,
            events: EventBuffer::with_capacity(128),
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

    pub fn events(&self) -> &EventBuffer {
        &self.events
    }

    pub fn events_mut_for_commit(&mut self) -> &mut EventBuffer {
        &mut self.events
    }

    pub(crate) fn advance_tick(&mut self) {
        self.tick = self.tick.next();
        self.spatial_index.rebuild(
            &self.cells,
            self.config.world.size,
            self.config.space.spatial_grid_size,
        );
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
                    FeasibilityResult::Feasible
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
                let free_capacity = self.cells.free_capacity(cell_idx);
                if free_capacity.raw() <= 0.0 {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientCapacity)
                } else {
                    FeasibilityResult::Feasible
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
                let internal_res = self.cells.resource_amount(cell_idx);
                if internal_res.raw() < action.requested_amount {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientResources)
                } else {
                    FeasibilityResult::Feasible
                }
            }
            ProcessId::MaterialSynthesis => FeasibilityResult::Feasible,
            ProcessId::GrowthResourceAllocation => {
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

                let current_res = self.cells.resource_amount(cell_idx).raw();
                let current_eng = self.cells.energy(cell_idx).current().raw();

                if current_res < cost_res {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientResources)
                } else if current_eng < cost_eng {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
                } else {
                    FeasibilityResult::Feasible
                }
            }
            ProcessId::Division => {
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

                FeasibilityResult::Feasible
            }
        }
    }

    pub fn execute_growth_for_test(
        &mut self,
        cell_idx: CellIndex,
        _action: &crate::core::process::ActionCandidate,
    ) -> Result<(), String> {
        let config = self.config.clone();

        let cost_res = config.growth.growth_cost_resource.raw();
        let cost_eng = config.growth.growth_cost_energy.raw();

        let current_res = self.cells.resource_amount(cell_idx).raw();
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

        // Synthesize materials (mass)
        let old_materials = self.cells.material_amount(cell_idx).raw();
        let new_materials = old_materials + 1.0;
        self.cells
            .set_materials(cell_idx, MaterialAmount::new(new_materials).unwrap());

        // Update radius based on mass scaling: radius = base_radius * sqrt(new_mass / old_mass)
        let old_radius = self.cells.radius(cell_idx).raw();
        let new_radius_val = if old_materials > 0.0 {
            old_radius * (new_materials / old_materials).sqrt()
        } else {
            old_radius
        };
        let new_radius = Radius::new(new_radius_val).unwrap();
        self.cells.set_radius(cell_idx, new_radius);

        // Update capacity limit scaling with radius area increase: capacity = old_cap * (new_radius / old_radius)^2
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
}
