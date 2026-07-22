use crate::bootstrap::prepared::PreparedStateHash;
use crate::bootstrap::seed_domains::SeedDomainRecord;
use crate::bootstrap::viability::ViabilityReport;
use crate::runner::scenario_doc::ScenarioHash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratorVersion {
    pub name: String,
}

impl GeneratorVersion {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorldSummary {
    pub width: f32,
    pub height: f32,
    pub spatial_grid_size: f32,
    pub initial_cells: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldFamilySummary {
    pub family_id: String,
    pub generator_version: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceLayerSummary {
    pub layer_index: usize,
    pub total: f32,
    pub min: f32,
    pub max: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldLayerSummary {
    pub field_id: String,
    pub min: f32,
    pub max: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellSummary {
    pub initial_cells: usize,
    pub genome_assigned_cells: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BootstrapWarning {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BootstrapManifest {
    pub schema_version: u32,
    pub scenario_hash: ScenarioHash,
    pub prepared_state_hash: PreparedStateHash,
    pub root_seed: u64,
    pub generator_versions: Vec<GeneratorVersion>,
    pub seed_domains: Vec<SeedDomainRecord>,
    pub world_family: Option<WorldFamilySummary>,
    pub world_summary: WorldSummary,
    pub resource_summary: Vec<ResourceLayerSummary>,
    pub field_summary: Vec<FieldLayerSummary>,
    pub cell_summary: CellSummary,
    pub viability: ViabilityReport,
    pub warnings: Vec<BootstrapWarning>,
}
