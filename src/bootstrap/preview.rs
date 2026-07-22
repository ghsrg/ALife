use crate::bootstrap::BootstrapError;
use crate::bootstrap::prepare;
use crate::bootstrap::viability::ViabilityStatus;
use crate::core::units::Seed;
use crate::runner::scenario_doc::ScenarioDocument;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BootstrapPreviewOptions {
    pub max_resource_cells_per_layer: usize,
}

impl Default for BootstrapPreviewOptions {
    fn default() -> Self {
        Self {
            max_resource_cells_per_layer: 128,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BootstrapPreviewReport {
    pub scenario_id: String,
    pub scenario_hash: String,
    pub prepared_state_hash: String,
    pub root_seed: u64,
    pub tick_executed: bool,
    pub generator_versions: Vec<String>,
    pub seed_domains: Vec<SeedDomainPreview>,
    pub world_summary: WorldPreview,
    pub cell_summary: CellPreview,
    pub resource_layers: Vec<ResourceLayerPreview>,
    pub field_layers: Vec<FieldLayerPreview>,
    pub viability: ViabilityPreview,
    pub warnings: Vec<BootstrapPreviewWarning>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SeedDomainPreview {
    pub label: String,
    pub domain_seed: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WorldPreview {
    pub width: f32,
    pub height: f32,
    pub spatial_grid_size: f32,
    pub initial_cells: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CellPreview {
    pub initial_cells: usize,
    pub genome_assigned_cells: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResourceLayerPreview {
    pub layer_index: usize,
    pub width: usize,
    pub height: usize,
    pub total: f32,
    pub min: f32,
    pub max: f32,
    pub sampled_cells: Vec<ResourceCellPreview>,
    pub truncated: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResourceCellPreview {
    pub x: usize,
    pub y: usize,
    pub amount: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FieldLayerPreview {
    pub field_id: String,
    pub min: f32,
    pub max: f32,
    pub spatial_grid_available: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ViabilityPreview {
    pub status: String,
    pub checks: Vec<ViabilityCheckPreview>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ViabilityCheckPreview {
    pub code: String,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BootstrapPreviewWarning {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeedSweepOptions {
    pub first_seed: u64,
    pub seed_count: usize,
    pub max_resource_cells_per_layer: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SeedSweepReport {
    pub scenario_id: String,
    pub scenario_hash: String,
    pub first_seed: u64,
    pub seed_count: usize,
    pub rows: Vec<SeedSweepRow>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SeedSweepRow {
    pub seed: u64,
    pub prepared_state_hash: String,
    pub tick_executed: bool,
    pub viability_status: String,
    pub resource_layer_count: usize,
    pub initial_cells: usize,
    pub warnings: Vec<String>,
}

pub fn build_bootstrap_preview(
    document: &ScenarioDocument,
    options: BootstrapPreviewOptions,
) -> Result<BootstrapPreviewReport, BootstrapError> {
    let prepared = prepare(document)?;
    let manifest = &prepared.manifest;
    let resource_layers = prepared
        .runtime_config
        .prepared_resource_layers
        .as_ref()
        .map(|layers| {
            layers
                .iter()
                .enumerate()
                .map(|(layer_index, cells)| {
                    let width = manifest.world_summary.width;
                    let height = manifest.world_summary.height;
                    let grid = manifest.world_summary.spatial_grid_size;
                    let grid_width = (width / grid).ceil().max(1.0) as usize;
                    let grid_height = (height / grid).ceil().max(1.0) as usize;
                    let sampled_cells = cells
                        .iter()
                        .take(options.max_resource_cells_per_layer)
                        .enumerate()
                        .map(|(index, amount)| ResourceCellPreview {
                            x: index % grid_width,
                            y: index / grid_width,
                            amount: amount.raw(),
                        })
                        .collect::<Vec<_>>();
                    let summary = manifest
                        .resource_summary
                        .iter()
                        .find(|summary| summary.layer_index == layer_index);
                    ResourceLayerPreview {
                        layer_index,
                        width: grid_width,
                        height: grid_height,
                        total: summary.map(|summary| summary.total).unwrap_or(0.0),
                        min: summary.map(|summary| summary.min).unwrap_or(0.0),
                        max: summary.map(|summary| summary.max).unwrap_or(0.0),
                        sampled_cells,
                        truncated: cells.len() > options.max_resource_cells_per_layer,
                    }
                })
                .collect()
        })
        .unwrap_or_else(Vec::new);

    Ok(BootstrapPreviewReport {
        scenario_id: document.id.clone(),
        scenario_hash: document.scenario_hash.to_string(),
        prepared_state_hash: manifest.prepared_state_hash.to_string(),
        root_seed: manifest.root_seed,
        tick_executed: false,
        generator_versions: manifest
            .generator_versions
            .iter()
            .map(|version| version.name.clone())
            .collect(),
        seed_domains: manifest
            .seed_domains
            .iter()
            .map(|domain| SeedDomainPreview {
                label: domain.label.clone(),
                domain_seed: domain.domain_seed,
            })
            .collect(),
        world_summary: WorldPreview {
            width: manifest.world_summary.width,
            height: manifest.world_summary.height,
            spatial_grid_size: manifest.world_summary.spatial_grid_size,
            initial_cells: manifest.world_summary.initial_cells,
        },
        cell_summary: CellPreview {
            initial_cells: manifest.cell_summary.initial_cells,
            genome_assigned_cells: manifest.cell_summary.genome_assigned_cells,
        },
        resource_layers,
        field_layers: manifest
            .field_summary
            .iter()
            .map(|field| FieldLayerPreview {
                field_id: field.field_id.clone(),
                min: field.min,
                max: field.max,
                spatial_grid_available: false,
            })
            .collect(),
        viability: ViabilityPreview {
            status: viability_status_label(&manifest.viability.status).to_string(),
            checks: manifest
                .viability
                .checks
                .iter()
                .map(|check| ViabilityCheckPreview {
                    code: check.code.clone(),
                    passed: check.passed,
                })
                .collect(),
            warnings: manifest.viability.warnings.clone(),
        },
        warnings: manifest
            .warnings
            .iter()
            .map(|warning| BootstrapPreviewWarning {
                code: warning.code.clone(),
                message: warning.message.clone(),
            })
            .collect(),
    })
}

pub fn run_bootstrap_seed_sweep(
    document: &ScenarioDocument,
    options: SeedSweepOptions,
) -> Result<SeedSweepReport, BootstrapError> {
    let mut rows = Vec::with_capacity(options.seed_count);
    for offset in 0..options.seed_count {
        let seed = options.first_seed + offset as u64;
        let mut seeded_document = document.clone();
        seeded_document.runtime_config.world.seed = Seed::from_raw(seed);
        let preview = build_bootstrap_preview(
            &seeded_document,
            BootstrapPreviewOptions {
                max_resource_cells_per_layer: options.max_resource_cells_per_layer,
            },
        )?;
        rows.push(SeedSweepRow {
            seed,
            prepared_state_hash: preview.prepared_state_hash,
            tick_executed: preview.tick_executed,
            viability_status: preview.viability.status,
            resource_layer_count: preview.resource_layers.len(),
            initial_cells: preview.cell_summary.initial_cells,
            warnings: preview
                .warnings
                .into_iter()
                .map(|warning| warning.code)
                .collect(),
        });
    }

    Ok(SeedSweepReport {
        scenario_id: document.id.clone(),
        scenario_hash: document.scenario_hash.to_string(),
        first_seed: options.first_seed,
        seed_count: options.seed_count,
        rows,
    })
}

fn viability_status_label(status: &ViabilityStatus) -> &'static str {
    match status {
        ViabilityStatus::Pass => "pass",
        ViabilityStatus::Warn => "warn",
    }
}
