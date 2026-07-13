use crate::core::deltas::{ReactionDestination, ReactionLocation, ReactionSource};
use crate::core::ids::{MaterialTypeId, ResourceTypeId};
use crate::core::units::{
    EnergyAmount, HeatAmount, MaterialAmount, Radius, ResourceAmount, Temperature,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReactionId(u32);

impl ReactionId {
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReactionMode {
    Passive,
    Controlled,
    Degradation,
    Decay,
    Synthesis,
    Conversion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ControlledEnergyToken(());

impl ControlledEnergyToken {
    #[allow(dead_code)]
    pub(crate) const fn from_feasibility() -> Self {
        Self(())
    }
}

#[cfg(test)]
mod tests {
    use super::ControlledEnergyToken;

    #[test]
    fn controlled_energy_token_is_constructed_inside_core_after_feasibility() {
        let _ = ControlledEnergyToken::from_feasibility();
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReactionTerm {
    Resource {
        id: ResourceTypeId,
        amount: ResourceAmount,
    },
    Material {
        id: MaterialTypeId,
        amount: MaterialAmount,
    },
}

impl ReactionTerm {
    pub const fn resource(id: ResourceTypeId, amount: ResourceAmount) -> Self {
        Self::Resource { id, amount }
    }

    pub const fn material(id: MaterialTypeId, amount: MaterialAmount) -> Self {
        Self::Material { id, amount }
    }

    fn same_type(self, other: Self) -> bool {
        match (self, other) {
            (Self::Resource { id: lhs, .. }, Self::Resource { id: rhs, .. }) => lhs == rhs,
            (Self::Material { id: lhs, .. }, Self::Material { id: rhs, .. }) => lhs == rhs,
            _ => false,
        }
    }

    pub const fn amount(self) -> f32 {
        match self {
            Self::Resource { amount, .. } => amount.raw(),
            Self::Material { amount, .. } => amount.raw(),
        }
    }

    pub const fn sort_key(self) -> (u8, u32) {
        match self {
            Self::Resource { id, .. } => (0, id.raw()),
            Self::Material { id, .. } => (1, id.raw()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReactionDelta {
    pub location: ReactionLocation,
    pub source: ReactionSource,
    pub reaction_id: ReactionId,
    pub mode: ReactionMode,
    pub inputs: Vec<ReactionTerm>,
    pub destinations: Vec<ReactionDestination>,
    pub heat_output: HeatAmount,
    pub energy_output: EnergyAmount,
    pub controlled_energy_token: Option<ControlledEnergyToken>,
}

impl ReactionDelta {
    pub fn builder(
        location: ReactionLocation,
        source: ReactionSource,
        reaction_id: ReactionId,
        mode: ReactionMode,
    ) -> ReactionDeltaBuilder {
        ReactionDeltaBuilder {
            location,
            source,
            reaction_id,
            mode,
            inputs: Vec::new(),
            destinations: Vec::new(),
            heat_output: HeatAmount::zero(),
            energy_output: EnergyAmount::zero(),
            controlled_energy_token: None,
        }
    }
}

pub struct ReactionDeltaBuilder {
    location: ReactionLocation,
    source: ReactionSource,
    reaction_id: ReactionId,
    mode: ReactionMode,
    inputs: Vec<ReactionTerm>,
    destinations: Vec<ReactionDestination>,
    heat_output: HeatAmount,
    energy_output: EnergyAmount,
    controlled_energy_token: Option<ControlledEnergyToken>,
}

impl ReactionDeltaBuilder {
    pub fn inputs(mut self, inputs: Vec<ReactionTerm>) -> Self {
        self.inputs = inputs;
        self
    }
    pub fn destinations(mut self, destinations: Vec<ReactionDestination>) -> Self {
        self.destinations = destinations;
        self
    }
    pub fn heat_output(mut self, heat_output: HeatAmount) -> Self {
        self.heat_output = heat_output;
        self
    }
    pub fn energy_output(mut self, energy_output: EnergyAmount) -> Self {
        self.energy_output = energy_output;
        self
    }
    pub fn controlled_energy_token(mut self, token: ControlledEnergyToken) -> Self {
        self.controlled_energy_token = Some(token);
        self
    }
    pub fn build(self) -> ReactionDelta {
        ReactionDelta {
            location: self.location,
            source: self.source,
            reaction_id: self.reaction_id,
            mode: self.mode,
            inputs: self.inputs,
            destinations: self.destinations,
            heat_output: self.heat_output,
            energy_output: self.energy_output,
            controlled_energy_token: self.controlled_energy_token,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReactionConditionsError {
    NonFiniteTemperature,
    InvalidTemperatureRange,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReactionConditions {
    minimum_temperature: Temperature,
    maximum_temperature: Temperature,
}

impl ReactionConditions {
    pub fn temperature_between(
        minimum: f32,
        maximum: f32,
    ) -> Result<Self, ReactionConditionsError> {
        if !minimum.is_finite() || !maximum.is_finite() {
            return Err(ReactionConditionsError::NonFiniteTemperature);
        }
        if minimum > maximum {
            return Err(ReactionConditionsError::InvalidTemperatureRange);
        }
        Ok(Self {
            minimum_temperature: Temperature::new(minimum),
            maximum_temperature: Temperature::new(maximum),
        })
    }

    fn matches(self, temperature: Temperature) -> bool {
        temperature.raw().is_finite()
            && (self.minimum_temperature.raw()..=self.maximum_temperature.raw())
                .contains(&temperature.raw())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CatalystRequirement {
    material_id: MaterialTypeId,
    minimum_amount: MaterialAmount,
}

impl CatalystRequirement {
    pub const fn new(material_id: MaterialTypeId, minimum_amount: MaterialAmount) -> Self {
        Self {
            material_id,
            minimum_amount,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Locality {
    radius: Radius,
}

impl Locality {
    pub const fn new(radius: Radius) -> Self {
        Self { radius }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReactionContext {
    resources: Vec<ReactionTerm>,
    materials: Vec<ReactionTerm>,
    temperature: Temperature,
    locality_radius: Radius,
}

impl ReactionContext {
    pub fn new(
        resources: Vec<ReactionTerm>,
        materials: Vec<ReactionTerm>,
        temperature: Temperature,
        locality_radius: Radius,
    ) -> Self {
        Self {
            resources,
            materials,
            temperature,
            locality_radius,
        }
    }

    fn available_amount(&self, requirement: ReactionTerm) -> f32 {
        let terms = match requirement {
            ReactionTerm::Resource { .. } => &self.resources,
            ReactionTerm::Material { .. } => &self.materials,
        };
        terms
            .iter()
            .copied()
            .filter(|term| term.same_type(requirement))
            .map(ReactionTerm::amount)
            .sum()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReactionBuildError {
    OutputsRequireInputs,
    InvalidRate,
    InvalidProbability,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Reaction {
    id: ReactionId,
    mode: ReactionMode,
    inputs: Vec<ReactionTerm>,
    outputs: Vec<ReactionTerm>,
    conditions: Option<ReactionConditions>,
    catalyst: Option<CatalystRequirement>,
    rate: f32,
    probability: f32,
    locality: Option<Locality>,
}

impl Reaction {
    pub fn builder(id: ReactionId, mode: ReactionMode) -> ReactionBuilder {
        ReactionBuilder::new(id, mode)
    }

    pub const fn id(&self) -> ReactionId {
        self.id
    }

    pub const fn mode(&self) -> ReactionMode {
        self.mode
    }

    pub fn inputs(&self) -> &[ReactionTerm] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[ReactionTerm] {
        &self.outputs
    }

    pub const fn rate(&self) -> f32 {
        self.rate
    }

    pub const fn probability(&self) -> f32 {
        self.probability
    }

    pub fn matches(&self, context: &ReactionContext) -> bool {
        let inputs_match = self.inputs.iter().copied().all(|required| {
            let required_amount: f32 = self
                .inputs
                .iter()
                .copied()
                .filter(|term| term.same_type(required))
                .map(ReactionTerm::amount)
                .sum();
            context.available_amount(required) >= required_amount
        });
        let temperature_matches = self
            .conditions
            .is_none_or(|conditions| conditions.matches(context.temperature));
        let catalyst_matches = self.catalyst.is_none_or(|catalyst| {
            context.available_amount(ReactionTerm::material(
                catalyst.material_id,
                catalyst.minimum_amount,
            )) >= catalyst.minimum_amount.raw()
        });
        let locality_matches = self
            .locality
            .is_none_or(|locality| context.locality_radius.raw() <= locality.radius.raw());

        inputs_match && temperature_matches && catalyst_matches && locality_matches
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReactionBuilder {
    id: ReactionId,
    mode: ReactionMode,
    inputs: Vec<ReactionTerm>,
    outputs: Vec<ReactionTerm>,
    conditions: Option<ReactionConditions>,
    catalyst: Option<CatalystRequirement>,
    rate: f32,
    probability: f32,
    locality: Option<Locality>,
}

impl ReactionBuilder {
    fn new(id: ReactionId, mode: ReactionMode) -> Self {
        Self {
            id,
            mode,
            inputs: Vec::new(),
            outputs: Vec::new(),
            conditions: None,
            catalyst: None,
            rate: 1.0,
            probability: 1.0,
            locality: None,
        }
    }

    pub fn inputs(mut self, inputs: Vec<ReactionTerm>) -> Self {
        self.inputs = inputs;
        self
    }

    pub fn outputs(mut self, outputs: Vec<ReactionTerm>) -> Self {
        self.outputs = outputs;
        self
    }

    pub const fn conditions(mut self, conditions: ReactionConditions) -> Self {
        self.conditions = Some(conditions);
        self
    }

    pub const fn catalyst(mut self, catalyst: CatalystRequirement) -> Self {
        self.catalyst = Some(catalyst);
        self
    }

    pub const fn rate(mut self, rate: f32) -> Self {
        self.rate = rate;
        self
    }

    pub const fn probability(mut self, probability: f32) -> Self {
        self.probability = probability;
        self
    }

    pub const fn locality(mut self, locality: Locality) -> Self {
        self.locality = Some(locality);
        self
    }

    pub fn build(self) -> Result<Reaction, ReactionBuildError> {
        if !self.outputs.is_empty() && self.inputs.is_empty() {
            return Err(ReactionBuildError::OutputsRequireInputs);
        }
        if !self.rate.is_finite() || self.rate < 0.0 {
            return Err(ReactionBuildError::InvalidRate);
        }
        if !self.probability.is_finite() || !(0.0..=1.0).contains(&self.probability) {
            return Err(ReactionBuildError::InvalidProbability);
        }
        Ok(Reaction {
            id: self.id,
            mode: self.mode,
            inputs: self.inputs,
            outputs: self.outputs,
            conditions: self.conditions,
            catalyst: self.catalyst,
            rate: self.rate,
            probability: self.probability,
            locality: self.locality,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReactionRegistryError {
    DuplicateId(ReactionId),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReactionRegistry {
    reactions: Vec<Reaction>,
}

impl ReactionRegistry {
    pub fn new(mut reactions: Vec<Reaction>) -> Result<Self, ReactionRegistryError> {
        reactions.sort_by_key(Reaction::id);
        for pair in reactions.windows(2) {
            if pair[0].id() == pair[1].id() {
                return Err(ReactionRegistryError::DuplicateId(pair[0].id()));
            }
        }
        Ok(Self { reactions })
    }

    pub fn matching_candidates<'a>(
        &'a self,
        context: &'a ReactionContext,
    ) -> impl Iterator<Item = &'a Reaction> + 'a {
        self.reactions
            .iter()
            .filter(move |reaction| reaction.matches(context))
    }
}
