#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaterialCapability {
    BoundaryPermeability,
    ResourceUptake,
    Metabolism,
    StructuralGrowth,
    StorageCapacity,
    Repair,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MaterialCapabilityFlags {
    pub boundary_permeability: bool,
    pub resource_uptake: bool,
    pub metabolism: bool,
    pub structural_growth: bool,
    pub storage_capacity: bool,
    pub repair: bool,
}

impl MaterialCapabilityFlags {
    pub const fn has(&self, capability: MaterialCapability) -> bool {
        match capability {
            MaterialCapability::BoundaryPermeability => self.boundary_permeability,
            MaterialCapability::ResourceUptake => self.resource_uptake,
            MaterialCapability::Metabolism => self.metabolism,
            MaterialCapability::StructuralGrowth => self.structural_growth,
            MaterialCapability::StorageCapacity => self.storage_capacity,
            MaterialCapability::Repair => self.repair,
        }
    }
}
