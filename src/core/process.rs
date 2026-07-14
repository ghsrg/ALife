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
    ContractileDisplacement,
    PassiveContactExchange,
    RepairBoundary,
    JointCreate,
    JointRepair,
}

impl ProcessId {
    pub const fn phase3a_baseline_order(self) -> Option<usize> {
        match self {
            ProcessId::LocalResourceUptake => Some(0),
            ProcessId::MetabolismEnergyConversion => Some(1),
            ProcessId::MaterialSynthesis => Some(2),
            ProcessId::GrowthResourceAllocation => Some(3),
            ProcessId::ContractileDisplacement => Some(4),
            ProcessId::RepairBoundary => Some(5),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActionCandidate {
    pub process_id: ProcessId,
    pub requested_amount: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RejectionReason {
    MissingCapability(MaterialCapability),
    InsufficientResources,
    InsufficientEnergy,
    InsufficientCapacity,
    LifecycleStateDead,
    RadiusBelowTarget,
    GrowthTargetReached,
    PressureTooHigh,
    NoPressure,
    ProcessDisabled,
    MissingTargetDamage,
    JointNotLocal,
    JointEndpointLimitReached,
    JointAlreadyExists,
    InsufficientMaterial,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FeasibilityResult {
    /// Execution MUST use `accepted_amount`/`energy_cost`/`resource_cost` from this payload.
    /// Never re-read config constants independently after a feasibility check passes.
    Allowed {
        accepted_amount: f32,
        energy_cost: f32,
        resource_cost: f32,
    },
    Rejected(RejectionReason),
}

pub struct FeasibilityInput<'a> {
    pub cell_idx: crate::core::cell_store::CellIndex,
    pub cells: &'a crate::core::cell_store::CellStore,
    pub resource_interaction: &'a crate::core::config::ResourceInteractionConfig,
}

impl FeasibilityResult {
    pub fn is_feasible(&self) -> bool {
        matches!(self, Self::Allowed { .. })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessStatus {
    Now,
    Future,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProcessSpec {
    pub process_id: ProcessId,
    pub status: ProcessStatus,
    pub required_capabilities: &'static [MaterialCapability],
    pub description: &'static str,
}

impl ProcessSpec {
    pub fn for_id(id: ProcessId) -> &'static ProcessSpec {
        PROCESS_REGISTRY
            .iter()
            .find(|s| s.process_id == id)
            .unwrap_or_else(|| panic!("no registry entry for {:?}", id))
    }
}

static PROCESS_REGISTRY: &[ProcessSpec] = &[
    ProcessSpec {
        process_id: ProcessId::MandatoryUpkeep,
        status: ProcessStatus::Now,
        required_capabilities: &[],
        description: "Deducts mandatory energy cost every tick.",
    },
    ProcessSpec {
        process_id: ProcessId::LocalResourceUptake,
        status: ProcessStatus::Now,
        required_capabilities: &[MaterialCapability::ResourceUptake],
        description: "Absorbs external resources from local grid cell.",
    },
    ProcessSpec {
        process_id: ProcessId::MetabolismEnergyConversion,
        status: ProcessStatus::Now,
        required_capabilities: &[MaterialCapability::Metabolism],
        description: "Converts internal resources to energy.",
    },
    ProcessSpec {
        process_id: ProcessId::MaterialSynthesis,
        status: ProcessStatus::Now,
        required_capabilities: &[MaterialCapability::MaterialSynthesis],
        description: "Converts resource+energy into structural material.",
    },
    ProcessSpec {
        process_id: ProcessId::GrowthResourceAllocation,
        status: ProcessStatus::Now,
        required_capabilities: &[MaterialCapability::StructuralGrowth],
        description: "Grows cell radius using resource+energy budget.",
    },
    ProcessSpec {
        process_id: ProcessId::ContractileDisplacement,
        status: ProcessStatus::Now,
        required_capabilities: &[MaterialCapability::Contractility],
        description: "Displaces cell away from collision neighbors when contact_pressure > 0.",
    },
    ProcessSpec {
        process_id: ProcessId::PassiveContactExchange,
        status: ProcessStatus::Now,
        required_capabilities: &[
            MaterialCapability::BoundaryPermeability,
            MaterialCapability::ResourceUptake,
        ],
        description: "Passively moves internal resources between contacting cells down a resource gradient.",
    },
    ProcessSpec {
        process_id: ProcessId::RepairBoundary,
        status: ProcessStatus::Now,
        required_capabilities: &[MaterialCapability::Repair],
        description: "Consumes local inputs to restore damaged boundary material.",
    },
    ProcessSpec {
        process_id: ProcessId::JointCreate,
        status: ProcessStatus::Now,
        required_capabilities: &[
            MaterialCapability::BoundaryPermeability,
            MaterialCapability::StructuralGrowth,
        ],
        description: "Creates a local material-backed joint between contacting cells.",
    },
    ProcessSpec {
        process_id: ProcessId::JointRepair,
        status: ProcessStatus::Future,
        required_capabilities: &[MaterialCapability::Repair],
        description: "Repairs damaged joint material using local resources.",
    },
    ProcessSpec {
        process_id: ProcessId::Division,
        status: ProcessStatus::Now,
        required_capabilities: &[],
        description: "Splits one living cell into two accounted daughter cells.",
    },
];
