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
        let mut cells = CellStore::with_capacity(1);
        cells.insert_initial(InitialCellState {
            position: config.cell.position,
            radius: config.cell.radius,
            energy: EnergyBuffer::new(config.cell.initial_energy, config.cell.energy_capacity),
            resources: config.cell.initial_resource_amount,
            materials: config.cell.initial_material_amount,
            capacity_limit: config.cell.capacity_limit,
            temperature: crate::core::units::Temperature::new(25.0),
        });

        let mut spatial_index = SpatialIndex::new();
        spatial_index.rebuild();

        Ok(Self {
            tick: Tick::from_raw(0),
            config,
            cells,
            resources: ResourceGrid::phase1_placeholder(config.cell.initial_resource_amount),
            environment: EnvironmentState::from_config(&config.environment),
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

    pub fn environment(&self) -> EnvironmentState {
        self.environment
    }

    pub fn environment_mut_for_commit(&mut self) -> &mut EnvironmentState {
        &mut self.environment
    }

    pub fn spatial_index(&self) -> &SpatialIndex {
        &self.spatial_index
    }

    pub fn events(&self) -> &EventBuffer {
        &self.events
    }

    pub fn events_mut_for_commit(&mut self) -> &mut EventBuffer {
        &mut self.events
    }

    pub(crate) fn advance_tick(&mut self) {
        self.tick = self.tick.next();
        self.spatial_index.rebuild();
    }
}
