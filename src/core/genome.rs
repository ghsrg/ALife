use crate::core::ids::CellId;
use crate::core::process::{MaterialCapability, ProcessId, ProcessSpec};
use crate::core::units::Tick;
use std::fmt;

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
    GenomeCopyingPriority,
}

impl GenomeOutputId {
    pub fn disposition_for(value: &'static str) -> GenomeOutputDisposition {
        match value {
            "resource_uptake_priority" => GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::LocalResourceUptake,
            },
            "resource_export_priority" => GenomeOutputDisposition::Deferred {
                reason: "resource export execution is not yet integrated into ActionPlan",
            },
            "energy_conversion_priority" => GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::MetabolismEnergyConversion,
            },
            "material_synthesis_priority" => GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::MaterialSynthesis,
            },
            "repair_priority" => GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::RepairBoundary,
            },
            "signal_emit_priority" => GenomeOutputDisposition::Deferred {
                reason: "signal emit execution is not yet integrated into ActionPlan",
            },
            "movement_priority" => GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::ContractileDisplacement,
            },
            "division_preparation_priority" => GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::GrowthResourceAllocation,
            },
            "genome_copying_priority" => GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::GenomeCopying,
            },
            "division_partition_priority" => GenomeOutputDisposition::Deferred {
                reason: "division partition execution is not yet integrated into ActionPlan",
            },
            "dormancy_bias" => GenomeOutputDisposition::Deferred {
                reason: "dormancy bias execution is not yet integrated into ActionPlan",
            },
            "internal_rebalance_priority" => GenomeOutputDisposition::Deferred {
                reason: "internal rebalance execution is not yet integrated into ActionPlan",
            },
            output_name => {
                GenomeOutputDisposition::UnsupportedUntilRegistryChange(UnsupportedGenomeOutput {
                    output_name,
                })
            }
        }
    }

    pub fn parse(value: &str) -> Result<Self, GenomeError> {
        match value {
            "resource_uptake_priority" => Ok(Self::ResourceUptakePriority),
            "energy_conversion_priority" => Ok(Self::EnergyConversionPriority),
            "material_synthesis_priority" => Ok(Self::MaterialSynthesisPriority),
            "repair_priority" => Ok(Self::RepairPriority),
            "movement_priority" => Ok(Self::MovementPriority),
            "division_preparation_priority" => Ok(Self::DivisionPreparationPriority),
            "genome_copying_priority" => Ok(Self::GenomeCopyingPriority),
            "resource_export_priority"
            | "signal_emit_priority"
            | "division_partition_priority"
            | "dormancy_bias"
            | "internal_rebalance_priority" => {
                Err(GenomeError::DeferredOutputId(value.to_string()))
            }
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
            Self::GenomeCopyingPriority => "genome_copying_priority",
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
            Self::GenomeCopyingPriority => ProcessId::GenomeCopying,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenomeOutputDisposition {
    EnabledNow { process_id: ProcessId },
    Deferred { reason: &'static str },
    UnsupportedUntilRegistryChange(UnsupportedGenomeOutput),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnsupportedGenomeOutput {
    pub output_name: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenomeRuntimeInputs {
    local_energy_level: f32,
    crowding_pressure: f32,
    capacity_used_fraction: f32,
    capabilities: Vec<(MaterialCapability, bool)>,
}

impl GenomeRuntimeInputs {
    pub fn new<I>(
        local_energy_level: f32,
        crowding_pressure: f32,
        capacity_limit: f32,
        capabilities: I,
    ) -> Self
    where
        I: IntoIterator<Item = (MaterialCapability, bool)>,
    {
        let capacity_used_fraction = if capacity_limit.is_finite() && capacity_limit > 0.0 {
            (crowding_pressure / capacity_limit).clamp(0.0, 1.0)
        } else {
            0.0
        };
        Self {
            local_energy_level: normalize_unit(local_energy_level),
            crowding_pressure: normalize_unit(crowding_pressure),
            capacity_used_fraction,
            capabilities: capabilities.into_iter().collect(),
        }
    }

    pub const fn local_energy_level(&self) -> f32 {
        self.local_energy_level
    }

    pub const fn crowding_pressure(&self) -> f32 {
        self.crowding_pressure
    }

    pub const fn capacity_used_fraction(&self) -> f32 {
        self.capacity_used_fraction
    }

    pub fn capability_available(&self, capability: MaterialCapability) -> bool {
        self.capabilities
            .iter()
            .find(|(candidate, _)| *candidate == capability)
            .map(|(_, available)| *available)
            .unwrap_or(false)
    }

    pub fn can_emit_to(&self, process_id: ProcessId) -> bool {
        ProcessSpec::for_id(process_id)
            .required_capabilities
            .iter()
            .all(|capability| self.capability_available(*capability))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenomeRuntimeTrace {
    tick: Tick,
    cell_id: CellId,
    inputs: GenomeRuntimeInputs,
    outputs: Vec<GenomeTraceOutput>,
    action_plan: Vec<ProcessId>,
    feasibility_result: String,
}

impl GenomeRuntimeTrace {
    pub fn new<O, A>(
        tick: Tick,
        cell_id: CellId,
        inputs: GenomeRuntimeInputs,
        outputs: O,
        action_plan: A,
        feasibility_result: impl Into<String>,
    ) -> Self
    where
        O: IntoIterator<Item = (&'static str, f32)>,
        A: IntoIterator<Item = ProcessId>,
    {
        Self {
            tick,
            cell_id,
            inputs,
            outputs: outputs
                .into_iter()
                .map(|(output_id, value)| GenomeTraceOutput {
                    output_id,
                    value: normalize_signed(value),
                })
                .collect(),
            action_plan: action_plan.into_iter().collect(),
            feasibility_result: feasibility_result.into(),
        }
    }

    pub const fn tick(&self) -> Tick {
        self.tick
    }

    pub const fn cell_id(&self) -> CellId {
        self.cell_id
    }

    pub const fn inputs(&self) -> &GenomeRuntimeInputs {
        &self.inputs
    }

    pub fn outputs(&self) -> &[GenomeTraceOutput] {
        &self.outputs
    }

    pub fn action_plan(&self) -> &[ProcessId] {
        &self.action_plan
    }

    pub fn feasibility_result(&self) -> &str {
        &self.feasibility_result
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GenomeTraceOutput {
    output_id: &'static str,
    value: f32,
}

impl GenomeTraceOutput {
    pub const fn output_id(self) -> &'static str {
        self.output_id
    }

    pub const fn value(self) -> f32 {
        self.value
    }
}

fn normalize_unit(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

fn normalize_signed(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(-1.0, 1.0)
    } else {
        0.0
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
    regulatory_depth: u64,
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
            regulatory_depth: 1,
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

    pub const fn regulatory_depth(&self) -> u64 {
        self.regulatory_depth
    }

    pub fn with_regulatory_depth(mut self, regulatory_depth: u64) -> Result<Self, GenomeError> {
        if regulatory_depth == 0 {
            return Err(GenomeError::InvalidRuntimeInterval);
        }
        self.regulatory_depth = regulatory_depth;
        Ok(self)
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

impl GenomeState {
    pub fn output(&self, id: GenomeOutputId) -> Option<GenomeOutputValue> {
        self.outputs
            .iter()
            .find(|(candidate, _)| *candidate == id)
            .map(|(_, value)| *value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenomeError {
    EmptyTemplateId,
    UnknownOutputId(String),
    DeferredOutputId(String),
    EmptyCarrierMaterialId,
    InvalidCarrierAmount,
    InvalidCarrierIntegrity,
    InvalidVariationAmplitude,
    InvalidRuntimeInterval,
    UnknownTemplate(String),
}

impl fmt::Display for GenomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTemplateId => f.write_str("empty Genome template id"),
            Self::UnknownOutputId(output_id) => {
                write!(f, "unknown Genome output id: {output_id}")
            }
            Self::DeferredOutputId(output_id) => {
                write!(f, "deferred Genome output id: {output_id}")
            }
            Self::EmptyCarrierMaterialId => f.write_str("empty Genome carrier material id"),
            Self::InvalidCarrierAmount => f.write_str("invalid Genome carrier amount"),
            Self::InvalidCarrierIntegrity => f.write_str("invalid Genome carrier integrity"),
            Self::InvalidVariationAmplitude => f.write_str("invalid Genome variation amplitude"),
            Self::InvalidRuntimeInterval => f.write_str("invalid Genome runtime interval"),
            Self::UnknownTemplate(template_id) => {
                write!(f, "unknown Genome template id: {template_id}")
            }
        }
    }
}

impl std::error::Error for GenomeError {}
