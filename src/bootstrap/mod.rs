pub mod cell_placement;
pub mod field_layers;
pub mod generator_spec;
pub mod manifest;
pub mod prepared;
pub mod preview;
pub mod resource_layers;
pub mod seed_domains;
pub mod starter_state;
pub mod viability;
pub mod world_families;

use crate::bootstrap::manifest::{
    BootstrapManifest, BootstrapWarning, CellSummary, FieldLayerSummary, GeneratorVersion,
    WorldSummary,
};
use crate::bootstrap::prepared::{PreparedWorld, prepared_state_hash_v1};
use crate::bootstrap::resource_layers::{
    PreparedResourceLayer, ResourceLayerError, generate_gradient_resource_layer,
    generate_patch_resource_layer, uniform_prepared_resource_layer,
};
use crate::bootstrap::seed_domains::{
    BOOTSTRAP_SEED_DOMAIN_VERSION, SeedDomain, derive_seed_domain, seed_domain_records,
};
use crate::bootstrap::viability::{ViabilityError, validate_prepared_config};
use crate::bootstrap::world_families::{WORLD_FAMILY_GENERATOR_VERSION, resolve_world_family};
use crate::core::units::FieldValue;
use crate::runner::scenario_doc::ScenarioDocument;
use std::fmt;

#[derive(Debug)]
pub enum BootstrapError {
    Viability(ViabilityError),
    Generator(ResourceLayerError),
    GeneratorSpec(crate::bootstrap::generator_spec::BootstrapGeneratorSpecError),
}

impl BootstrapError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Viability(_) => "BOOTSTRAP_VIABILITY_FAILED",
            Self::Generator(err) => err.code(),
            Self::GeneratorSpec(err) => err.code(),
        }
    }
}

impl fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Viability(err) => write!(f, "bootstrap viability failed: {err}"),
            Self::Generator(err) => write!(f, "bootstrap generator failed: {err}"),
            Self::GeneratorSpec(err) => write!(f, "bootstrap generator spec failed: {err}"),
        }
    }
}

impl std::error::Error for BootstrapError {}

pub fn prepare(document: &ScenarioDocument) -> Result<PreparedWorld, BootstrapError> {
    let mut runtime_config = document.runtime_config.clone();
    let root_seed = runtime_config.world.seed.raw();
    let mut seed_domains = seed_domain_records(root_seed, document.scenario_hash, SeedDomain::ALL);
    let mut generator_versions = vec![
        GeneratorVersion::new(BOOTSTRAP_SEED_DOMAIN_VERSION),
        GeneratorVersion::new("prepared_world.v1"),
        GeneratorVersion::new("viability.v1"),
    ];
    let world_family = resolve_world_family(
        document
            .bootstrap_spec
            .as_ref()
            .and_then(|spec| spec.family.as_deref()),
    )
    .map_err(BootstrapError::GeneratorSpec)?;
    if world_family.is_some() {
        generator_versions.push(GeneratorVersion::new(WORLD_FAMILY_GENERATOR_VERSION));
    }

    let grid_width = (runtime_config.world.size.width() / runtime_config.space.spatial_grid_size)
        .ceil()
        .max(1.0) as usize;
    let grid_height = (runtime_config.world.size.height() / runtime_config.space.spatial_grid_size)
        .ceil()
        .max(1.0) as usize;
    let prepared_resources = prepare_resource_layers(
        document,
        grid_width,
        grid_height,
        &mut seed_domains,
        &mut generator_versions,
    )?;
    let resource_summary = prepared_resources
        .iter()
        .map(|layer| layer.summary.clone())
        .collect::<Vec<_>>();
    if document.bootstrap_spec.is_some() {
        runtime_config.prepared_resource_layers = Some(
            prepared_resources
                .iter()
                .map(|layer| layer.quantities.clone())
                .collect(),
        );
    }
    runtime_config.prepared_field_layers = prepare_field_layers(
        document,
        grid_width,
        grid_height,
        &mut seed_domains,
        &mut generator_versions,
    )?;
    let viability = validate_prepared_config(&runtime_config).map_err(BootstrapError::Viability)?;
    let field_summary = vec![
        FieldLayerSummary {
            field_id: "heat".to_string(),
            min: runtime_config.environment.heat_current.raw(),
            max: runtime_config.environment.heat_current.raw(),
        },
        FieldLayerSummary {
            field_id: "waste".to_string(),
            min: runtime_config.environment.waste_current.raw(),
            max: runtime_config.environment.waste_current.raw(),
        },
    ]
    .into_iter()
    .chain(generated_field_summaries(document))
    .collect::<Vec<_>>();
    let cell_count = runtime_config.initial_cells.len();
    let world_summary = WorldSummary {
        width: runtime_config.world.size.width(),
        height: runtime_config.world.size.height(),
        spatial_grid_size: runtime_config.space.spatial_grid_size,
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
        world_family,
        world_summary,
        resource_summary,
        field_summary,
        cell_summary,
        viability,
        warnings: generated_warnings(document),
    };

    Ok(PreparedWorld {
        runtime_config,
        manifest,
        prepared_state_hash,
    })
}

fn prepare_field_layers(
    document: &ScenarioDocument,
    width: usize,
    height: usize,
    seed_domains: &mut Vec<crate::bootstrap::seed_domains::SeedDomainRecord>,
    generator_versions: &mut Vec<GeneratorVersion>,
) -> Result<Option<Vec<Vec<FieldValue>>>, BootstrapError> {
    let Some(spec) = document.bootstrap_spec.as_ref() else {
        return Ok(None);
    };
    if spec.fields.is_empty() || document.runtime_config.fields.is_empty() {
        return Ok(None);
    }

    let cell_count = width * height;
    let mut matched = false;
    let mut layers = document
        .runtime_config
        .fields
        .iter()
        .map(|field| vec![field.initial_value; cell_count])
        .collect::<Vec<_>>();

    for field_spec in &spec.fields {
        let Some(layer_index) = document
            .runtime_config
            .fields
            .iter()
            .position(|field| field.id == field_spec.field_id)
        else {
            continue;
        };
        let runtime_field = &document.runtime_config.fields[layer_index];
        let min_value = field_spec
            .min_value
            .unwrap_or(runtime_field.initial_value.raw());
        let max_value = field_spec
            .max_value
            .unwrap_or(runtime_field.initial_value.raw());
        if !min_value.is_finite() || !max_value.is_finite() || min_value > max_value {
            return Err(BootstrapError::Generator(ResourceLayerError::new(
                "BOOTSTRAP_INVALID_RESOURCE_LAYER",
            )));
        }
        let domain = derive_seed_domain(
            root_seed(document),
            document.scenario_hash,
            &field_spec.seed_domain,
        );
        seed_domains.push(domain);
        generator_versions.push(GeneratorVersion::new(field_spec.version.clone()));
        layers[layer_index] = match field_spec.generator.as_str() {
            "band" => vec![field_value_midpoint(min_value, max_value, runtime_field)?; cell_count],
            "gradient" => {
                gradient_field_values(width, height, min_value, max_value, runtime_field)?
            }
            _ => {
                return Err(BootstrapError::Generator(ResourceLayerError::new(
                    "BOOTSTRAP_UNKNOWN_FIELD_GENERATOR",
                )));
            }
        };
        matched = true;
    }

    Ok(matched.then_some(layers))
}

fn field_value_midpoint(
    min_value: f32,
    max_value: f32,
    runtime_field: &crate::core::fields::FieldRuntimeConfig,
) -> Result<FieldValue, BootstrapError> {
    let value = ((min_value + max_value) * 0.5)
        .clamp(runtime_field.min_value.raw(), runtime_field.max_value.raw());
    FieldValue::new(value).map_err(|_| {
        BootstrapError::Generator(ResourceLayerError::new("BOOTSTRAP_INVALID_RESOURCE_LAYER"))
    })
}

fn gradient_field_values(
    width: usize,
    height: usize,
    min_value: f32,
    max_value: f32,
    runtime_field: &crate::core::fields::FieldRuntimeConfig,
) -> Result<Vec<FieldValue>, BootstrapError> {
    let denominator = width.saturating_sub(1).max(1) as f32;
    let mut values = Vec::with_capacity(width * height);
    for _y in 0..height {
        for x in 0..width {
            let t = x as f32 / denominator;
            let value = (min_value + (max_value - min_value) * t)
                .clamp(runtime_field.min_value.raw(), runtime_field.max_value.raw());
            values.push(FieldValue::new(value).map_err(|_| {
                BootstrapError::Generator(ResourceLayerError::new(
                    "BOOTSTRAP_INVALID_RESOURCE_LAYER",
                ))
            })?);
        }
    }
    Ok(values)
}

fn prepare_resource_layers(
    document: &ScenarioDocument,
    width: usize,
    height: usize,
    seed_domains: &mut Vec<crate::bootstrap::seed_domains::SeedDomainRecord>,
    generator_versions: &mut Vec<GeneratorVersion>,
) -> Result<Vec<PreparedResourceLayer>, BootstrapError> {
    let cell_count = width * height;
    let mut layers = document
        .runtime_config
        .resources
        .initial_distribution
        .iter()
        .enumerate()
        .map(|(index, amount)| uniform_prepared_resource_layer(index, cell_count, amount.raw()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(BootstrapError::Generator)?;

    let Some(spec) = document.bootstrap_spec.as_ref() else {
        return Ok(layers);
    };

    for resource in &spec.resources {
        let Some(layer_index) = document
            .resource_type_ids
            .iter()
            .position(|id| id == &resource.resource_type_id)
        else {
            return Err(BootstrapError::GeneratorSpec(
                crate::bootstrap::generator_spec::BootstrapGeneratorSpecError::new(
                    "BOOTSTRAP_UNKNOWN_RESOURCE_TYPE",
                ),
            ));
        };
        let domain = derive_seed_domain(
            root_seed(document),
            document.scenario_hash,
            &resource.seed_domain,
        );
        seed_domains.push(domain.clone());
        generator_versions.push(GeneratorVersion::new(resource.version.clone()));
        let mut rng = crate::bootstrap::seed_domains::SplitMix64::new(domain.domain_seed);
        layers[layer_index] = match resource.generator.as_str() {
            "patches" => generate_patch_resource_layer(
                layer_index,
                width,
                height,
                resource.patches.unwrap_or(3),
                resource.min_amount.unwrap_or(0.0),
                resource.max_amount.unwrap_or(1.0),
                resource.falloff.unwrap_or(0.35),
                &mut rng,
            ),
            "gradient" => generate_gradient_resource_layer(
                layer_index,
                width,
                height,
                resource.min_amount.unwrap_or(0.0),
                resource.max_amount.unwrap_or(1.0),
            ),
            _ => Err(ResourceLayerError::new(
                "BOOTSTRAP_UNKNOWN_RESOURCE_GENERATOR",
            )),
        }
        .map_err(BootstrapError::Generator)?;
    }
    seed_domains.sort_by(|a, b| a.label.cmp(&b.label));
    Ok(layers)
}

fn generated_field_summaries(document: &ScenarioDocument) -> Vec<FieldLayerSummary> {
    document
        .bootstrap_spec
        .as_ref()
        .map(|spec| {
            spec.fields
                .iter()
                .map(|field| FieldLayerSummary {
                    field_id: field.field_id.clone(),
                    min: field.min_value.unwrap_or(0.0),
                    max: field.max_value.unwrap_or(0.0),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn generated_warnings(document: &ScenarioDocument) -> Vec<BootstrapWarning> {
    if document.bootstrap_spec.as_ref().is_some_and(|spec| {
        spec.fields.iter().any(|field_spec| {
            !document
                .runtime_config
                .fields
                .iter()
                .any(|field| field.id == field_spec.field_id)
        })
    }) {
        vec![BootstrapWarning {
            code: "BOOTSTRAP_FIELD_LAYER_NOT_CORE_INTEGRATED".to_string(),
            message:
                "Field generator summaries are manifest-only until Core field grids are integrated."
                    .to_string(),
        }]
    } else {
        Vec::new()
    }
}

fn root_seed(document: &ScenarioDocument) -> u64 {
    document.runtime_config.world.seed.raw()
}
