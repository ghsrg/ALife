use crate::core::ids::ResourceTypeId;
use crate::core::units::{DecayRate, DiffusionRate, EnergyValue, Volume};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReactivityProfile {
    Stable,
    Reactive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PermeabilityConstraint {
    Blocked,
    Passive,
    ActiveRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ResourceTag {
    EnergySource = 0,
    Dissolved = 1,
    StructuralPrecursor = 2,
    Waste = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ResourceTags(u32);

impl ResourceTags {
    pub fn from<const N: usize>(tags: [ResourceTag; N]) -> Self {
        let mut result = Self::default();
        for tag in tags {
            result.insert(tag);
        }
        result
    }

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }

    pub fn insert(&mut self, tag: ResourceTag) {
        self.0 |= 1 << tag as u32;
    }

    pub const fn contains(self, tag: ResourceTag) -> bool {
        self.0 & (1 << tag as u32) != 0
    }
}

impl FromIterator<ResourceTag> for ResourceTags {
    fn from_iter<T: IntoIterator<Item = ResourceTag>>(iter: T) -> Self {
        let mut result = Self::empty();
        for tag in iter {
            result.insert(tag);
        }
        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResourceProperties {
    volume: Volume,
    diffusion_rate: DiffusionRate,
    energy_value: EnergyValue,
    decay_rate: DecayRate,
    reactivity_profile: ReactivityProfile,
    permeability: PermeabilityConstraint,
    tags: ResourceTags,
}

impl ResourceProperties {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        volume: Volume,
        diffusion_rate: DiffusionRate,
        energy_value: EnergyValue,
        decay_rate: DecayRate,
        reactivity_profile: ReactivityProfile,
        permeability: PermeabilityConstraint,
        tags: ResourceTags,
    ) -> Self {
        Self {
            volume,
            diffusion_rate,
            energy_value,
            decay_rate,
            reactivity_profile,
            permeability,
            tags,
        }
    }

    pub const fn volume(self) -> Volume {
        self.volume
    }
    pub const fn diffusion_rate(self) -> DiffusionRate {
        self.diffusion_rate
    }
    pub const fn energy_value(self) -> EnergyValue {
        self.energy_value
    }
    pub const fn decay_rate(self) -> DecayRate {
        self.decay_rate
    }
    pub const fn reactivity_profile(self) -> ReactivityProfile {
        self.reactivity_profile
    }
    pub const fn permeability(self) -> PermeabilityConstraint {
        self.permeability
    }
    pub const fn tags(self) -> ResourceTags {
        self.tags
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResourceType {
    id: ResourceTypeId,
    properties: ResourceProperties,
}

impl ResourceType {
    pub const fn new(id: ResourceTypeId, properties: ResourceProperties) -> Self {
        Self { id, properties }
    }

    pub const fn id(self) -> ResourceTypeId {
        self.id
    }
    pub const fn properties(self) -> ResourceProperties {
        self.properties
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceRegistryError {
    DuplicateId(ResourceTypeId),
    UnknownId(ResourceTypeId),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceRegistry {
    resources: Vec<ResourceType>,
}

impl ResourceRegistry {
    pub fn new(mut resources: Vec<ResourceType>) -> Result<Self, ResourceRegistryError> {
        resources.sort_by_key(|resource| resource.id());
        for pair in resources.windows(2) {
            if pair[0].id() == pair[1].id() {
                return Err(ResourceRegistryError::DuplicateId(pair[0].id()));
            }
        }
        Ok(Self { resources })
    }

    pub fn get(&self, id: ResourceTypeId) -> Option<&ResourceType> {
        self.resources
            .binary_search_by_key(&id, |resource| resource.id())
            .ok()
            .map(|index| &self.resources[index])
    }

    pub fn lookup(&self, id: ResourceTypeId) -> Result<&ResourceType, ResourceRegistryError> {
        self.get(id).ok_or(ResourceRegistryError::UnknownId(id))
    }

    pub fn iter(&self) -> impl Iterator<Item = &ResourceType> {
        self.resources.iter()
    }
}
