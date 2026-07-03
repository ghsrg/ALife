#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaterialCapability {
    BoundaryPermeability,
    ResourceUptake,
    Metabolism,
    StorageCapacity,
    MaterialSynthesis,
    StructuralGrowth,
    Repair,
    Contractility,
    ResourceSensing,
    PressureSensing,
    DamageSensing,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MaterialCapabilityFlags {
    pub boundary_permeability: bool,
    pub resource_uptake: bool,
    pub metabolism: bool,
    pub storage_capacity: bool,
    pub material_synthesis: bool,
    pub structural_growth: bool,
    pub repair: bool,
    pub contractility: bool,
    pub resource_sensing: bool,
    pub pressure_sensing: bool,
    pub damage_sensing: bool,
}

impl MaterialCapabilityFlags {
    pub const fn has(&self, capability: MaterialCapability) -> bool {
        match capability {
            MaterialCapability::BoundaryPermeability => self.boundary_permeability,
            MaterialCapability::ResourceUptake => self.resource_uptake,
            MaterialCapability::Metabolism => self.metabolism,
            MaterialCapability::StorageCapacity => self.storage_capacity,
            MaterialCapability::MaterialSynthesis => self.material_synthesis,
            MaterialCapability::StructuralGrowth => self.structural_growth,
            MaterialCapability::Repair => self.repair,
            MaterialCapability::Contractility => self.contractility,
            MaterialCapability::ResourceSensing => self.resource_sensing,
            MaterialCapability::PressureSensing => self.pressure_sensing,
            MaterialCapability::DamageSensing => self.damage_sensing,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProcessId {
    MandatoryUpkeep,
    LocalResourceUptake,
    MetabolismEnergyConversion,
    MaterialSynthesis,
    GrowthResourceAllocation,
    Division,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActionCandidate {
    pub process_id: ProcessId,
    pub requested_amount: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RejectionReason {
    MissingCapability(MaterialCapability),
    InsufficientResources,
    InsufficientEnergy,
    InsufficientCapacity,
    LifecycleStateDead,
    RadiusBelowTarget,
    PressureTooHigh,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeasibilityResult {
    Feasible,
    Rejected(RejectionReason),
}

pub struct FeasibilityInput<'a> {
    pub cell_idx: crate::core::cell_store::CellIndex,
    pub cells: &'a crate::core::cell_store::CellStore,
    pub resource_interaction: &'a crate::core::config::ResourceInteractionConfig,
}

impl FeasibilityResult {
    pub fn is_feasible(&self) -> bool {
        matches!(self, Self::Feasible)
    }
}
