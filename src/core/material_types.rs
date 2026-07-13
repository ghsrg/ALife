use crate::core::ids::MaterialTypeId;
use crate::core::units::{DecayRate, EnergyCapacity, SignalAmount, Strength, Volume};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReactionProfile {
    Passive,
    Reactive,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RepairRequirements {
    volume: Volume,
}

impl RepairRequirements {
    pub const fn new(volume: Volume) -> Self {
        Self { volume }
    }

    pub const fn volume(self) -> Volume {
        self.volume
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SignalProperties {
    signal_sensitivity: Strength,
    signal_storage: SignalAmount,
    signal_conductivity: Strength,
}

impl SignalProperties {
    pub fn new(
        signal_sensitivity: Strength,
        signal_storage: SignalAmount,
        signal_conductivity: Strength,
    ) -> Self {
        Self {
            signal_sensitivity,
            signal_storage,
            signal_conductivity,
        }
    }

    pub const fn signal_sensitivity(self) -> Strength {
        self.signal_sensitivity
    }
    pub const fn signal_storage(self) -> SignalAmount {
        self.signal_storage
    }
    pub const fn signal_conductivity(self) -> Strength {
        self.signal_conductivity
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialProperties {
    volume: Volume,
    stability: Strength,
    strength: Strength,
    permeability: Strength,
    energy_capacity: EnergyCapacity,
    decay_rate: DecayRate,
    repair_requirements: RepairRequirements,
    reaction_profile: ReactionProfile,
    signal: SignalProperties,
}

impl MaterialProperties {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        volume: Volume,
        stability: Strength,
        strength: Strength,
        permeability: Strength,
        energy_capacity: EnergyCapacity,
        decay_rate: DecayRate,
        repair_requirements: RepairRequirements,
        reaction_profile: ReactionProfile,
        signal: SignalProperties,
    ) -> Self {
        Self {
            volume,
            stability,
            strength,
            permeability,
            energy_capacity,
            decay_rate,
            repair_requirements,
            reaction_profile,
            signal,
        }
    }

    pub const fn volume(self) -> Volume {
        self.volume
    }
    pub const fn stability(self) -> Strength {
        self.stability
    }
    pub const fn strength(self) -> Strength {
        self.strength
    }
    pub const fn permeability(self) -> Strength {
        self.permeability
    }
    pub const fn energy_capacity(self) -> EnergyCapacity {
        self.energy_capacity
    }
    pub const fn decay_rate(self) -> DecayRate {
        self.decay_rate
    }
    pub const fn repair_requirements(self) -> RepairRequirements {
        self.repair_requirements
    }
    pub const fn reaction_profile(self) -> ReactionProfile {
        self.reaction_profile
    }
    pub const fn signal(self) -> SignalProperties {
        self.signal
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialType {
    id: MaterialTypeId,
    properties: MaterialProperties,
}

impl MaterialType {
    pub const fn new(id: MaterialTypeId, properties: MaterialProperties) -> Self {
        Self { id, properties }
    }

    pub const fn id(self) -> MaterialTypeId {
        self.id
    }

    pub const fn properties(self) -> MaterialProperties {
        self.properties
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialState {
    damage: Strength,
    fatigue: Strength,
    stored_signal: SignalAmount,
    conductivity_modifier: Strength,
}

impl MaterialState {
    pub fn new(
        damage: Strength,
        fatigue: Strength,
        stored_signal: SignalAmount,
        conductivity_modifier: Strength,
    ) -> Self {
        Self {
            damage,
            fatigue,
            stored_signal,
            conductivity_modifier,
        }
    }

    pub const fn damage(self) -> Strength {
        self.damage
    }
    pub const fn fatigue(self) -> Strength {
        self.fatigue
    }
    pub const fn stored_signal(self) -> SignalAmount {
        self.stored_signal
    }
    pub const fn conductivity_modifier(self) -> Strength {
        self.conductivity_modifier
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaterialRegistryError {
    DuplicateId(MaterialTypeId),
    UnknownId(MaterialTypeId),
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialRegistry {
    materials: Vec<MaterialType>,
}

impl MaterialRegistry {
    pub fn new(mut materials: Vec<MaterialType>) -> Result<Self, MaterialRegistryError> {
        materials.sort_by_key(|material| material.id());
        for pair in materials.windows(2) {
            if pair[0].id() == pair[1].id() {
                return Err(MaterialRegistryError::DuplicateId(pair[0].id()));
            }
        }
        Ok(Self { materials })
    }

    pub fn get(&self, id: MaterialTypeId) -> Option<&MaterialType> {
        self.materials
            .binary_search_by_key(&id, |material| material.id())
            .ok()
            .map(|index| &self.materials[index])
    }

    pub fn lookup(&self, id: MaterialTypeId) -> Result<&MaterialType, MaterialRegistryError> {
        self.get(id).ok_or(MaterialRegistryError::UnknownId(id))
    }

    pub fn iter(&self) -> impl Iterator<Item = &MaterialType> {
        self.materials.iter()
    }
}
