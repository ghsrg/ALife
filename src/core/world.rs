use crate::core::cell_store::{CellStore, EnergyBuffer, InitialCellState};
use crate::core::config::RuntimeConfig;
use crate::core::environment::EnvironmentState;
use crate::core::events::EventBuffer;
use crate::core::resources::ResourceGrid;
use crate::core::spatial::SpatialIndex;
use crate::core::units::Tick;

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
                materials: config.cell.initial_material_amount,
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
                    materials: cell_config.initial_material_amount,
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
}
