use crate::core::process::ProcessId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GenomeId(u32);

impl GenomeId {
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GenomeTemplateId(String);

impl GenomeTemplateId {
    pub fn new(value: impl Into<String>) -> Result<Self, GenomeError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(GenomeError::EmptyTemplateId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GenomeOutputId {
    ResourceUptakePriority,
    EnergyConversionPriority,
    MaterialSynthesisPriority,
    RepairPriority,
    MovementPriority,
    DivisionPreparationPriority,
}

impl GenomeOutputId {
    pub fn parse(value: &str) -> Result<Self, GenomeError> {
        match value {
            "resource_uptake_priority" => Ok(Self::ResourceUptakePriority),
            "energy_conversion_priority" => Ok(Self::EnergyConversionPriority),
            "material_synthesis_priority" => Ok(Self::MaterialSynthesisPriority),
            "repair_priority" => Ok(Self::RepairPriority),
            "movement_priority" => Ok(Self::MovementPriority),
            "division_preparation_priority" => Ok(Self::DivisionPreparationPriority),
            other => Err(GenomeError::UnknownOutputId(other.to_string())),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResourceUptakePriority => "resource_uptake_priority",
            Self::EnergyConversionPriority => "energy_conversion_priority",
            Self::MaterialSynthesisPriority => "material_synthesis_priority",
            Self::RepairPriority => "repair_priority",
            Self::MovementPriority => "movement_priority",
            Self::DivisionPreparationPriority => "division_preparation_priority",
        }
    }

    pub const fn process_id(self) -> ProcessId {
        match self {
            Self::ResourceUptakePriority => ProcessId::LocalResourceUptake,
            Self::EnergyConversionPriority => ProcessId::MetabolismEnergyConversion,
            Self::MaterialSynthesisPriority => ProcessId::MaterialSynthesis,
            Self::RepairPriority => ProcessId::RepairBoundary,
            Self::MovementPriority => ProcessId::ContractileDisplacement,
            Self::DivisionPreparationPriority => ProcessId::GrowthResourceAllocation,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GenomeOutputValue(f32);

impl GenomeOutputValue {
    pub fn new(value: f32) -> Self {
        if !value.is_finite() {
            return Self(0.0);
        }
        Self(value.clamp(-1.0, 1.0))
    }

    pub const fn raw(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenomeCarrierState {
    pub material_id: String,
    pub amount: f32,
    pub integrity: f32,
}

impl GenomeCarrierState {
    pub fn new(material_id: String, amount: f32, integrity: f32) -> Result<Self, GenomeError> {
        if material_id.trim().is_empty() {
            return Err(GenomeError::EmptyCarrierMaterialId);
        }
        if !amount.is_finite() || amount <= 0.0 {
            return Err(GenomeError::InvalidCarrierAmount);
        }
        if !integrity.is_finite() || !(0.0..=1.0).contains(&integrity) {
            return Err(GenomeError::InvalidCarrierIntegrity);
        }
        Ok(Self {
            material_id,
            amount,
            integrity,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenomeTemplate {
    id: GenomeTemplateId,
    variation_amplitude: f32,
    runtime_interval_ticks: u64,
    carrier: GenomeCarrierState,
    outputs: Vec<(GenomeOutputId, GenomeOutputValue)>,
}

impl GenomeTemplate {
    pub fn new(
        id: GenomeTemplateId,
        variation_amplitude: f32,
        runtime_interval_ticks: u64,
        carrier: GenomeCarrierState,
        mut outputs: Vec<(GenomeOutputId, GenomeOutputValue)>,
    ) -> Result<Self, GenomeError> {
        if !variation_amplitude.is_finite() || !(0.0..=1.0).contains(&variation_amplitude) {
            return Err(GenomeError::InvalidVariationAmplitude);
        }
        if runtime_interval_ticks == 0 {
            return Err(GenomeError::InvalidRuntimeInterval);
        }
        outputs.sort_by_key(|(id, _)| id.as_str());
        outputs.dedup_by_key(|(id, _)| *id);
        Ok(Self {
            id,
            variation_amplitude,
            runtime_interval_ticks,
            carrier,
            outputs,
        })
    }

    pub fn id(&self) -> &GenomeTemplateId {
        &self.id
    }

    pub const fn variation_amplitude(&self) -> f32 {
        self.variation_amplitude
    }

    pub const fn runtime_interval_ticks(&self) -> u64 {
        self.runtime_interval_ticks
    }

    pub fn carrier(&self) -> &GenomeCarrierState {
        &self.carrier
    }

    pub fn outputs(&self) -> &[(GenomeOutputId, GenomeOutputValue)] {
        &self.outputs
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenomeState {
    pub id: GenomeId,
    pub template_id: GenomeTemplateId,
    pub carrier: GenomeCarrierState,
    pub outputs: Vec<(GenomeOutputId, GenomeOutputValue)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenomeError {
    EmptyTemplateId,
    UnknownOutputId(String),
    EmptyCarrierMaterialId,
    InvalidCarrierAmount,
    InvalidCarrierIntegrity,
    InvalidVariationAmplitude,
    InvalidRuntimeInterval,
    UnknownTemplate(String),
}
