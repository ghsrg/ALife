use crate::core::cell_store::LifecycleState;
use crate::observer::balance::BalanceOutcome;
use crate::observer::classifiers::{ClassificationMode, ClassificationStatus, LabelResult};
use crate::observer::contract::WarningDisposition;
use crate::observer::projection::EntityType;
use crate::observer::projection_envelope::ProjectionCompleteness;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionSourceMetricRef {
    pub field_id: String,
    pub source_owner: String,
    pub source_path: String,
}

impl ProjectionSourceMetricRef {
    pub fn new(
        field_id: impl Into<String>,
        source_owner: impl Into<String>,
        source_path: impl Into<String>,
    ) -> Self {
        Self {
            field_id: field_id.into(),
            source_owner: source_owner.into(),
            source_path: source_path.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialAmountPayload {
    pub material_type_id: u32,
    pub amount: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceAmountPayload {
    pub resource_type_id: u32,
    pub amount: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PhenotypeTraitPayload {
    pub flagella_count: u8,
    pub spike_count: u8,
    pub receptor_halo_intensity: f32,
    pub lineage_hue: u16,
    pub division_flash_intensity: f32,
}

impl Default for PhenotypeTraitPayload {
    fn default() -> Self {
        Self {
            flagella_count: 0,
            spike_count: 0,
            receptor_halo_intensity: 0.0,
            lineage_hue: 180,
            division_flash_intensity: 0.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VisualCellPayload {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub energy: f32,
    pub energy_capacity: f32,
    pub lifecycle_state: LifecycleState,
    pub materials: Vec<MaterialAmountPayload>,
    pub internal_resources: Vec<ResourceAmountPayload>,
    pub local_external_resources: Vec<ResourceAmountPayload>,
    pub phenotype_traits: PhenotypeTraitPayload,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceLayerCellPayload {
    pub x: u32,
    pub y: u32,
    pub amount: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceLayerSummaryPayload {
    pub layer_index: u32,
    pub resource_type_id: u32,
    pub resource_id: String,
    pub width: u32,
    pub height: u32,
    pub total_amount: f32,
    pub cells: Vec<ResourceLayerCellPayload>,
    pub completeness: ProjectionCompleteness,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldSummaryPayload {
    pub field_id: String,
    pub value: f32,
    pub source_metric: ProjectionSourceMetricRef,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VisualJointPayload {
    pub id: u32,
    pub cell1_id: u32,
    pub cell2_id: u32,
    pub rest_length: f32,
    pub pulse_intensity: f32,
    pub signal_speed: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VisualOrganismPayload {
    pub id: u32,
    pub cell_ids: Vec<u32>,
    pub hull_color_hue: u16,
    pub organic_membrane_tension: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VisualWorldProjection {
    pub tick: u64,
    pub cells: Vec<VisualCellPayload>,
    pub joints: Vec<VisualJointPayload>,
    pub organisms: Vec<VisualOrganismPayload>,
    pub resource_layers: Vec<ResourceLayerSummaryPayload>,
    pub fields: Vec<FieldSummaryPayload>,
    pub completeness: ProjectionCompleteness,
    pub source_metrics: Vec<ProjectionSourceMetricRef>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClassificationEvidencePayload {
    pub feature: String,
    pub expected: String,
    pub actual: f32,
    pub matched: bool,
    pub source_metric: ProjectionSourceMetricRef,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClassificationProjectionPayload {
    pub classification_id: String,
    pub dimension_id: String,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub mode: ClassificationMode,
    pub primary_label: Option<String>,
    pub secondary_labels: Vec<LabelResult>,
    pub status: ClassificationStatus,
    pub confidence: f32,
    pub tick_start: u64,
    pub tick_end: u64,
    pub classifier_version: String,
    pub registry_version: String,
    pub source_projection: String,
    pub completeness: ProjectionCompleteness,
    pub evidence: Vec<ClassificationEvidencePayload>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObserverProjectionPayloadError {
    UnknownCoverageStatus(String),
    UnknownWarningCode(String),
}

impl fmt::Display for ObserverProjectionPayloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCoverageStatus(status) => {
                write!(f, "unknown coverage status: {}", status)
            }
            Self::UnknownWarningCode(code) => write!(f, "unknown warning code: {}", code),
        }
    }
}

impl std::error::Error for ObserverProjectionPayloadError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoverageMechanismPayload {
    pub mechanism_id: String,
    pub status_id: String,
    pub source_report: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoverageProjectionPayload {
    pub mechanisms: Vec<CoverageMechanismPayload>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WarningProjectionPayload {
    pub code: String,
    pub disposition: WarningDisposition,
    pub affected_scope: String,
    pub source_report: String,
    pub recommended_reruns: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BalanceFindingProjectionPayload {
    pub finding_id: String,
    pub compared_profiles: (String, String),
    pub equal_requirements: bool,
    pub reported_result: BalanceOutcome,
    pub claimed_result: BalanceOutcome,
    pub evidence_metrics: Vec<String>,
    pub dominance_rate: f32,
    pub affected_scenarios: Vec<String>,
    pub suspected_cause: Option<String>,
    pub recommendation: Option<String>,
    pub recommended_reruns: Vec<String>,
    pub confidence: f32,
    pub source_report: String,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrganismViewPayload {
    pub organism_id: u32,
    pub cell_ids: Vec<u32>,
    pub primary_cell_id: u32,
    pub total_cells_count: usize,
    pub total_mass: f32,
    pub total_energy: f32,
    pub total_joints_count: usize,
    pub centroid_x: f32,
    pub centroid_y: f32,
    pub confidence: f32,
    pub completeness: ProjectionCompleteness,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrganismViewProjection {
    pub tick: u64,
    pub organisms: Vec<OrganismViewPayload>,
    pub total_organisms_count: usize,
    pub unattached_cells_count: usize,
    pub completeness: ProjectionCompleteness,
    pub source_metrics: Vec<ProjectionSourceMetricRef>,
}
