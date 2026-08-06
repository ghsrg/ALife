use std::collections::BTreeMap;

use crate::core::process::MaterialCapability;
use crate::core::units::{
    EnergyAmount, HeatAmount, MaterialAmount, Position, ResourceAmount, Tick,
};

const CAPABILITIES: [MaterialCapability; 12] = [
    MaterialCapability::BoundaryPermeability,
    MaterialCapability::ResourceUptake,
    MaterialCapability::Metabolism,
    MaterialCapability::StorageCapacity,
    MaterialCapability::MaterialSynthesis,
    MaterialCapability::StructuralGrowth,
    MaterialCapability::Repair,
    MaterialCapability::GenomeCopying,
    MaterialCapability::Contractility,
    MaterialCapability::ResourceSensing,
    MaterialCapability::PressureSensing,
    MaterialCapability::DamageSensing,
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialProfile {
    volume: f32,
    stability: f32,
    strength: f32,
    energy_capacity: f32,
    permeability: f32,
    durability: f32,
}

impl MaterialProfile {
    pub fn new(
        volume: f32,
        stability: f32,
        strength: f32,
        energy_capacity: f32,
        permeability: f32,
        durability: f32,
    ) -> Result<Self, MaterialProfileError> {
        let profile = Self {
            volume,
            stability,
            strength,
            energy_capacity,
            permeability,
            durability,
        };
        if [
            profile.volume,
            profile.stability,
            profile.strength,
            profile.energy_capacity,
            profile.permeability,
            profile.durability,
        ]
        .iter()
        .all(|value| value.is_finite() && (0.0..=1.0).contains(value))
        {
            Ok(profile)
        } else {
            Err(MaterialProfileError::OutOfBounds)
        }
    }

    pub const fn volume(self) -> f32 {
        self.volume
    }
    pub const fn stability(self) -> f32 {
        self.stability
    }
    pub const fn strength(self) -> f32 {
        self.strength
    }
    pub const fn energy_capacity(self) -> f32 {
        self.energy_capacity
    }
    pub const fn permeability(self) -> f32 {
        self.permeability
    }
    pub const fn durability(self) -> f32 {
        self.durability
    }

    fn weighted(inputs: &[MaterialRecipeInput]) -> Result<Self, MaterialInstanceError> {
        let total = inputs.iter().map(|input| input.amount.raw()).sum::<f32>();
        if total <= 0.0 || !total.is_finite() {
            return Err(MaterialInstanceError::NoPrecursorVolume);
        }
        let weighted = |value: fn(MaterialProfile) -> f32| {
            inputs
                .iter()
                .map(|input| input.amount.raw() * value(input.profile))
                .sum::<f32>()
                / total
        };
        Self::new(
            weighted(Self::volume),
            weighted(Self::stability),
            weighted(Self::strength),
            weighted(Self::energy_capacity),
            weighted(Self::permeability),
            weighted(Self::durability),
        )
        .map_err(|_| MaterialInstanceError::InvalidDerivedProfile)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialProfileError {
    OutOfBounds,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialCapabilityProfile {
    values: [f32; 12],
}

impl MaterialCapabilityProfile {
    pub const fn empty() -> Self {
        Self { values: [0.0; 12] }
    }

    pub fn with(mut self, capability: MaterialCapability, value: f32) -> Self {
        self.values[capability_index(capability)] = value.clamp(0.0, 1.0);
        self
    }

    pub fn value(self, capability: MaterialCapability) -> f32 {
        self.values[capability_index(capability)]
    }

    pub(crate) fn values(self) -> [f32; 12] {
        self.values
    }

    fn weighted(inputs: &[MaterialRecipeInput]) -> Result<Self, MaterialInstanceError> {
        let total = inputs.iter().map(|input| input.amount.raw()).sum::<f32>();
        if total <= 0.0 || !total.is_finite() {
            return Err(MaterialInstanceError::NoPrecursorVolume);
        }
        let mut result = Self::empty();
        for capability in CAPABILITIES {
            result = result.with(
                capability,
                inputs
                    .iter()
                    .map(|input| input.amount.raw() * input.capabilities.value(capability))
                    .sum::<f32>()
                    / total,
            );
        }
        Ok(result)
    }
}

fn capability_index(capability: MaterialCapability) -> usize {
    CAPABILITIES
        .iter()
        .position(|candidate| *candidate == capability)
        .expect("all canonical capabilities are indexed")
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialRecipeInput {
    resource_id: String,
    amount: MaterialAmount,
    profile: MaterialProfile,
    capabilities: MaterialCapabilityProfile,
}

impl MaterialRecipeInput {
    pub fn new(
        resource_id: impl Into<String>,
        amount: MaterialAmount,
        profile: MaterialProfile,
        capabilities: MaterialCapabilityProfile,
    ) -> Self {
        Self {
            resource_id: resource_id.into(),
            amount,
            profile,
            capabilities,
        }
    }
    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialInstance {
    amount: MaterialAmount,
    profile: MaterialProfile,
    capabilities: MaterialCapabilityProfile,
    precursors: Vec<MaterialRecipeInput>,
}

impl MaterialInstance {
    pub fn from_precursors(
        amount: MaterialAmount,
        precursors: Vec<MaterialRecipeInput>,
    ) -> Result<Self, MaterialInstanceError> {
        if precursors.is_empty() {
            return Err(MaterialInstanceError::NoPrecursorVolume);
        }
        let profile = MaterialProfile::weighted(&precursors)?;
        let capabilities = MaterialCapabilityProfile::weighted(&precursors)?;
        Ok(Self {
            amount,
            profile,
            capabilities,
            precursors,
        })
    }
    pub const fn amount(&self) -> MaterialAmount {
        self.amount
    }
    pub const fn profile(&self) -> MaterialProfile {
        self.profile
    }
    pub const fn capabilities(&self) -> MaterialCapabilityProfile {
        self.capabilities
    }
    pub fn precursors(&self) -> &[MaterialRecipeInput] {
        &self.precursors
    }

    pub fn stable_fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        hash_u32(&mut hash, self.amount.raw().to_bits());
        hash_profile(&mut hash, self.profile);
        hash_capabilities(&mut hash, self.capabilities);

        let mut precursors = self.precursors.iter().collect::<Vec<_>>();
        precursors.sort_by(|left, right| {
            left.resource_id().cmp(right.resource_id()).then_with(|| {
                left.amount
                    .raw()
                    .to_bits()
                    .cmp(&right.amount.raw().to_bits())
            })
        });
        for precursor in precursors {
            hash_str(&mut hash, precursor.resource_id());
            hash_u32(&mut hash, precursor.amount.raw().to_bits());
            hash_profile(&mut hash, precursor.profile);
            hash_capabilities(&mut hash, precursor.capabilities);
        }
        hash
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialInstanceError {
    NoPrecursorVolume,
    InvalidDerivedProfile,
    InvalidFragmentAmount,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialSynthesisRecipe {
    id: String,
    output_amount: MaterialAmount,
    energy_cost: EnergyAmount,
    heat_output: HeatAmount,
    precursors: Vec<MaterialRecipeInput>,
    waste_outputs: Vec<(String, ResourceAmount)>,
}

impl MaterialSynthesisRecipe {
    pub fn new(
        id: impl Into<String>,
        output_amount: MaterialAmount,
        energy_cost: EnergyAmount,
        heat_output: HeatAmount,
        precursors: Vec<MaterialRecipeInput>,
        waste_outputs: Vec<(&str, ResourceAmount)>,
    ) -> Self {
        Self {
            id: id.into(),
            output_amount,
            energy_cost,
            heat_output,
            precursors,
            waste_outputs: waste_outputs
                .into_iter()
                .map(|(id, amount)| (id.to_string(), amount))
                .collect(),
        }
    }

    pub fn apply(
        &self,
        inventory: &mut MaterialSynthesisInventory,
    ) -> Result<MaterialSynthesisOutcome, MaterialSynthesisRejection> {
        if self.precursors.is_empty() || self.output_amount.raw() <= 0.0 {
            return Err(MaterialSynthesisRejection::InvalidRecipe);
        }
        if inventory.energy.raw() < self.energy_cost.raw() {
            return Err(MaterialSynthesisRejection::InsufficientEnergy);
        }
        if inventory.capacity_remaining.raw() < self.output_amount.raw() {
            return Err(MaterialSynthesisRejection::InsufficientCapacity);
        }
        for precursor in &self.precursors {
            if inventory.resource_amount(precursor.resource_id()).raw() < precursor.amount.raw() {
                return Err(MaterialSynthesisRejection::MissingPrecursor);
            }
        }

        let material =
            MaterialInstance::from_precursors(self.output_amount, self.precursors.clone())
                .map_err(|_| MaterialSynthesisRejection::InvalidRecipe)?;
        inventory.energy =
            EnergyAmount::new(inventory.energy.raw() - self.energy_cost.raw()).unwrap();
        inventory.capacity_remaining =
            MaterialAmount::new(inventory.capacity_remaining.raw() - self.output_amount.raw())
                .unwrap();
        for precursor in &self.precursors {
            let current = inventory.resource_amount(precursor.resource_id()).raw();
            inventory.set_resource(
                precursor.resource_id(),
                ResourceAmount::new(current - precursor.amount.raw()).unwrap(),
            );
        }
        inventory.materials.push(material.clone());

        Ok(MaterialSynthesisOutcome {
            material,
            heat_output: self.heat_output,
            waste_outputs: self.waste_outputs.clone(),
            energy_spent: self.energy_cost,
        })
    }

    pub fn derive_material_instance(&self) -> Result<MaterialInstance, MaterialInstanceError> {
        MaterialInstance::from_precursors(self.output_amount, self.precursors.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialSynthesisInventory {
    energy: EnergyAmount,
    capacity_remaining: MaterialAmount,
    resources: BTreeMap<String, ResourceAmount>,
    materials: Vec<MaterialInstance>,
}

impl MaterialSynthesisInventory {
    pub fn new(energy: EnergyAmount, capacity_remaining: MaterialAmount) -> Self {
        Self {
            energy,
            capacity_remaining,
            resources: BTreeMap::new(),
            materials: Vec::new(),
        }
    }

    pub fn set_resource(&mut self, id: impl Into<String>, amount: ResourceAmount) {
        self.resources.insert(id.into(), amount);
    }

    pub fn resource_amount(&self, id: &str) -> ResourceAmount {
        self.resources
            .get(id)
            .copied()
            .unwrap_or_else(ResourceAmount::zero)
    }

    pub const fn energy(&self) -> EnergyAmount {
        self.energy
    }

    pub fn materials(&self) -> &[MaterialInstance] {
        &self.materials
    }

    pub fn snapshot(&self) -> MaterialSynthesisInventorySnapshot {
        MaterialSynthesisInventorySnapshot {
            energy: self.energy,
            capacity_remaining: self.capacity_remaining,
            resources: self.resources.clone(),
            material_fingerprints: self
                .materials
                .iter()
                .map(MaterialInstance::stable_fingerprint)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialSynthesisInventorySnapshot {
    energy: EnergyAmount,
    capacity_remaining: MaterialAmount,
    resources: BTreeMap<String, ResourceAmount>,
    material_fingerprints: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialSynthesisOutcome {
    material: MaterialInstance,
    heat_output: HeatAmount,
    waste_outputs: Vec<(String, ResourceAmount)>,
    energy_spent: EnergyAmount,
}

impl MaterialSynthesisOutcome {
    pub const fn material(&self) -> &MaterialInstance {
        &self.material
    }
    pub const fn heat_output(&self) -> HeatAmount {
        self.heat_output
    }
    pub fn waste_outputs(&self) -> &[(String, ResourceAmount)] {
        &self.waste_outputs
    }
    pub const fn energy_spent(&self) -> EnergyAmount {
        self.energy_spent
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialSynthesisRejection {
    InvalidRecipe,
    MissingPrecursor,
    InsufficientEnergy,
    InsufficientCapacity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialInstanceFragment {
    amount: MaterialAmount,
    position: Position,
    created_tick: Tick,
    profile: MaterialProfile,
    capabilities: MaterialCapabilityProfile,
    source_fingerprint: u64,
}

impl MaterialInstanceFragment {
    pub const fn amount(&self) -> MaterialAmount {
        self.amount
    }
    pub const fn position(&self) -> Position {
        self.position
    }
    pub const fn created_tick(&self) -> Tick {
        self.created_tick
    }
    pub const fn profile(&self) -> MaterialProfile {
        self.profile
    }
    pub const fn source_fingerprint(&self) -> u64 {
        self.source_fingerprint
    }

    pub const fn active_cell_capability(&self, _capability: MaterialCapability) -> f32 {
        0.0
    }

    pub fn resource_outputs_without_conversion(&self) -> Vec<(String, ResourceAmount)> {
        Vec::new()
    }
}

impl MaterialInstance {
    pub fn degrade_to_fragment(
        &self,
        amount: MaterialAmount,
        position: Position,
        created_tick: Tick,
    ) -> Result<MaterialInstanceFragment, MaterialInstanceError> {
        if amount.raw() <= 0.0 || amount.raw() > self.amount.raw() {
            return Err(MaterialInstanceError::InvalidFragmentAmount);
        }
        Ok(MaterialInstanceFragment {
            amount,
            position,
            created_tick,
            profile: self.profile,
            capabilities: self.capabilities,
            source_fingerprint: self.stable_fingerprint(),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialFragmentConversionRecipe {
    id: String,
    outputs_per_material: Vec<(String, ResourceAmount)>,
}

impl MaterialFragmentConversionRecipe {
    pub fn new(id: impl Into<String>, outputs_per_material: Vec<(&str, ResourceAmount)>) -> Self {
        Self {
            id: id.into(),
            outputs_per_material: outputs_per_material
                .into_iter()
                .map(|(id, amount)| (id.to_string(), amount))
                .collect(),
        }
    }

    pub fn convert(&self, fragment: &MaterialInstanceFragment) -> Vec<(String, ResourceAmount)> {
        self.outputs_per_material
            .iter()
            .map(|(id, amount)| {
                (
                    id.clone(),
                    ResourceAmount::new(amount.raw() * fragment.amount.raw()).unwrap(),
                )
            })
            .collect()
    }
}

fn hash_profile(hash: &mut u64, profile: MaterialProfile) {
    hash_u32(hash, profile.volume().to_bits());
    hash_u32(hash, profile.stability().to_bits());
    hash_u32(hash, profile.strength().to_bits());
    hash_u32(hash, profile.energy_capacity().to_bits());
    hash_u32(hash, profile.permeability().to_bits());
    hash_u32(hash, profile.durability().to_bits());
}

fn hash_capabilities(hash: &mut u64, capabilities: MaterialCapabilityProfile) {
    for value in capabilities.values() {
        hash_u32(hash, value.to_bits());
    }
}

fn hash_str(hash: &mut u64, value: &str) {
    for byte in value.as_bytes() {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
}

fn hash_u32(hash: &mut u64, value: u32) {
    for byte in value.to_le_bytes() {
        *hash ^= u64::from(byte);
        *hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
}
