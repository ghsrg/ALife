pub mod cell_placement;
pub mod field_layers;
pub mod manifest;
pub mod prepared;
pub mod resource_layers;
pub mod seed_domains;
pub mod starter_state;
pub mod viability;

use crate::bootstrap::manifest::{
    BootstrapManifest, CellSummary, FieldLayerSummary, GeneratorVersion, ResourceLayerSummary,
    WorldSummary,
};
use crate::bootstrap::prepared::{PreparedWorld, prepared_state_hash_v1};
use crate::bootstrap::seed_domains::{
    BOOTSTRAP_SEED_DOMAIN_VERSION, SeedDomain, seed_domain_records,
};
use crate::bootstrap::viability::{ViabilityError, validate_prepared_config};
use crate::runner::scenario_doc::ScenarioDocument;
use std::fmt;

#[derive(Debug)]
pub enum BootstrapError {
    Viability(ViabilityError),
}

impl BootstrapError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Viability(_) => "BOOTSTRAP_VIABILITY_FAILED",
        }
    }
}

impl fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Viability(err) => write!(f, "bootstrap viability failed: {err}"),
        }
    }
}

impl std::error::Error for BootstrapError {}

pub fn prepare(document: &ScenarioDocument) -> Result<PreparedWorld, BootstrapError> {
    let viability =
        validate_prepared_config(&document.runtime_config).map_err(BootstrapError::Viability)?;
    let root_seed = document.runtime_config.world.seed.raw();
    let seed_domains = seed_domain_records(root_seed, document.scenario_hash, SeedDomain::ALL);
    let resource_summary = document
        .runtime_config
        .resources
        .initial_distribution
        .iter()
        .enumerate()
        .map(|(index, amount)| ResourceLayerSummary {
            layer_index: index,
            total: amount.raw(),
            min: amount.raw(),
            max: amount.raw(),
        })
        .collect::<Vec<_>>();
    let field_summary = vec![
        FieldLayerSummary {
            field_id: "heat".to_string(),
            min: document.runtime_config.environment.heat_current.raw(),
            max: document.runtime_config.environment.heat_current.raw(),
        },
        FieldLayerSummary {
            field_id: "waste".to_string(),
            min: document.runtime_config.environment.waste_current.raw(),
            max: document.runtime_config.environment.waste_current.raw(),
        },
    ];
    let cell_count = document.runtime_config.initial_cells.len();
    let world_summary = WorldSummary {
        width: document.runtime_config.world.size.width(),
        height: document.runtime_config.world.size.height(),
        spatial_grid_size: document.runtime_config.space.spatial_grid_size,
        initial_cells: cell_count,
    };
    let cell_summary = CellSummary {
        initial_cells: cell_count,
        genome_assigned_cells: document
            .runtime_config
            .initial_cell_genome_templates
            .iter()
            .filter(|assignment| assignment.is_some())
            .count(),
    };
    let generator_versions = vec![
        GeneratorVersion::new(BOOTSTRAP_SEED_DOMAIN_VERSION),
        GeneratorVersion::new("prepared_world.v1"),
        GeneratorVersion::new("viability.v1"),
    ];
    let prepared_state_hash = prepared_state_hash_v1(
        document.scenario_hash,
        root_seed,
        &world_summary,
        &cell_summary,
        &resource_summary,
        &field_summary,
    );
    let manifest = BootstrapManifest {
        schema_version: 1,
        scenario_hash: document.scenario_hash,
        prepared_state_hash,
        root_seed,
        generator_versions,
        seed_domains,
        world_summary,
        resource_summary,
        field_summary,
        cell_summary,
        viability,
        warnings: Vec::new(),
    };

    Ok(PreparedWorld {
        runtime_config: document.runtime_config.clone(),
        manifest,
        prepared_state_hash,
    })
}
