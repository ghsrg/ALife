use crate::core::ids::CellId;
use crate::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellIndex(usize);

impl CellIndex {
    pub const fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnergyBuffer {
    current: EnergyAmount,
    capacity: EnergyAmount,
}

impl EnergyBuffer {
    pub fn new(current: EnergyAmount, capacity: EnergyAmount) -> Self {
        Self {
            current: current.clamp_max(capacity),
            capacity,
        }
    }

    pub const fn current(self) -> EnergyAmount {
        self.current
    }

    pub const fn capacity(self) -> EnergyAmount {
        self.capacity
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleState {
    Alive,
    Stressed,
    Dormant,
    Dead,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RuntimeFlags {
    pub mandatory_paid: bool,
    pub stalled: bool,
    pub over_capacity: bool,
    pub inert: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitialCellState {
    pub position: Position,
    pub radius: Radius,
    pub energy: EnergyBuffer,
    pub resources: ResourceAmount,
    pub materials: MaterialAmount,
    pub capacity_limit: CapacityAmount,
    pub temperature: Temperature,
}

#[derive(Clone, Debug, Default)]
pub struct CellStore {
    ids: Vec<CellId>,
    positions: Vec<Position>,
    radii: Vec<Radius>,
    energy_buffers: Vec<EnergyBuffer>,
    resources: Vec<ResourceAmount>,
    materials: Vec<MaterialAmount>,
    capacity_limits: Vec<CapacityAmount>,
    temperatures: Vec<Temperature>,
    lifecycle_states: Vec<LifecycleState>,
    runtime_flags: Vec<RuntimeFlags>,
    next_cell_id: u32,
}

impl CellStore {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ids: Vec::with_capacity(capacity),
            positions: Vec::with_capacity(capacity),
            radii: Vec::with_capacity(capacity),
            energy_buffers: Vec::with_capacity(capacity),
            resources: Vec::with_capacity(capacity),
            materials: Vec::with_capacity(capacity),
            capacity_limits: Vec::with_capacity(capacity),
            temperatures: Vec::with_capacity(capacity),
            lifecycle_states: Vec::with_capacity(capacity),
            runtime_flags: Vec::with_capacity(capacity),
            next_cell_id: 1,
        }
    }

    pub fn insert_initial(&mut self, cell: InitialCellState) -> CellId {
        let id = CellId::from_raw(self.next_cell_id);
        self.next_cell_id += 1;
        self.ids.push(id);
        self.positions.push(cell.position);
        self.radii.push(cell.radius);
        self.energy_buffers.push(cell.energy);
        self.resources.push(cell.resources);
        self.materials.push(cell.materials);
        self.capacity_limits.push(cell.capacity_limit);
        self.temperatures.push(cell.temperature);
        self.lifecycle_states.push(LifecycleState::Alive);
        self.runtime_flags.push(RuntimeFlags::default());
        id
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn iter_indices(&self) -> impl Iterator<Item = CellIndex> {
        (0..self.len()).map(CellIndex::from_raw)
    }

    pub fn resolve_id_cold(&self, id: CellId) -> Option<CellIndex> {
        self.ids
            .iter()
            .position(|candidate| *candidate == id)
            .map(CellIndex)
    }

    pub fn id_at(&self, index: CellIndex) -> CellId {
        self.ids[index.raw()]
    }

    pub fn position(&self, index: CellIndex) -> Position {
        self.positions[index.raw()]
    }

    pub fn radius(&self, index: CellIndex) -> Radius {
        self.radii[index.raw()]
    }

    pub fn energy(&self, index: CellIndex) -> EnergyBuffer {
        self.energy_buffers[index.raw()]
    }

    pub fn lifecycle_state(&self, index: CellIndex) -> LifecycleState {
        self.lifecycle_states[index.raw()]
    }

    pub fn runtime_flags(&self, index: CellIndex) -> RuntimeFlags {
        self.runtime_flags[index.raw()]
    }

    pub fn used_capacity(&self, index: CellIndex) -> CapacityAmount {
        let genome_capacity_placeholder = 0.0;
        let internal_fragments_capacity_used = 0.0;
        let used = self.resources[index.raw()].raw()
            + self.materials[index.raw()].raw()
            + genome_capacity_placeholder
            + internal_fragments_capacity_used;
        CapacityAmount::new(used).expect("resource/material amounts are validated")
    }

    pub fn capacity_limit(&self, index: CellIndex) -> CapacityAmount {
        self.capacity_limits[index.raw()]
    }

    pub fn free_capacity(&self, index: CellIndex) -> CapacityAmount {
        let free =
            (self.capacity_limits[index.raw()].raw() - self.used_capacity(index).raw()).max(0.0);
        CapacityAmount::new(free).expect("free capacity is clamped")
    }

    pub fn resource_amount(&self, index: CellIndex) -> ResourceAmount {
        self.resources[index.raw()]
    }

    pub fn add_resources_limited_by_capacity(
        &mut self,
        index: CellIndex,
        requested: ResourceAmount,
    ) -> ResourceAmount {
        let accepted_raw = requested.raw().min(self.free_capacity(index).raw());
        let accepted = ResourceAmount::new(accepted_raw).expect("accepted uptake is clamped");
        self.resources[index.raw()] = self.resources[index.raw()].saturating_add(accepted);
        accepted
    }

    pub fn consume_resources(
        &mut self,
        index: CellIndex,
        requested: ResourceAmount,
    ) -> ResourceAmount {
        let available = self.resources[index.raw()];
        let consumed_raw = requested.raw().min(available.raw());
        let consumed = ResourceAmount::new(consumed_raw).expect("consumed resource is clamped");
        self.resources[index.raw()] = available.saturating_sub(consumed);
        consumed
    }

    pub(crate) fn set_energy(&mut self, index: CellIndex, energy: EnergyBuffer) {
        self.energy_buffers[index.raw()] = energy;
    }

    pub fn set_position(&mut self, index: CellIndex, position: Position) {
        self.positions[index.raw()] = position;
    }

    pub(crate) fn set_lifecycle_state(&mut self, index: CellIndex, state: LifecycleState) {
        self.lifecycle_states[index.raw()] = state;
    }

    pub(crate) fn set_runtime_flags(&mut self, index: CellIndex, flags: RuntimeFlags) {
        self.runtime_flags[index.raw()] = flags;
    }

    pub fn has_capability(
        &self,
        index: CellIndex,
        capability: crate::core::process::MaterialCapability,
    ) -> bool {
        use crate::core::process::MaterialCapabilityFlags;
        if self.lifecycle_state(index) == LifecycleState::Dead {
            return false;
        }
        let amount = self.materials[index.raw()];
        if amount.raw() > 0.0 {
            let default_flags = MaterialCapabilityFlags {
                boundary_permeability: true,
                resource_uptake: true,
                metabolism: true,
                structural_growth: true,
                storage_capacity: true,
                repair: true,
            };
            default_flags.has(capability)
        } else {
            false
        }
    }
}
