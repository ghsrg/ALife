use crate::core::config::{
    CellInitialConfig, ConfigError, ContractilityConfig, DecompositionConfig, DivisionConfig,
    EnvironmentConfig, GrowthConfig, LifecycleConfig, MaterialEffectConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, SynthesisConfig, WorldConfig,
};
use crate::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct RawWorld {
    pub size: [f32; 2],
}

#[derive(Deserialize, Debug)]
pub struct RawSpace {
    pub spatial_grid_size: f32,
    pub physics_solver_iterations: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct RawResources {
    pub resource_type_ids: Vec<String>,
    pub initial_distribution: Vec<f32>,
    pub optional_decay_rate: Option<f32>,
    pub passive_energy_income_placeholder: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct RawCell {
    pub initial_position: [f32; 2],
    pub radius: f32,
    pub initial_resources: HashMap<String, f32>,
    pub initial_materials: HashMap<String, f32>,
    pub initial_energy: f32,
    pub energy_capacity: f32,
    pub mandatory_cost_per_tick: f32,
    pub dormant_mandatory_cost_modifier: Option<f32>,
    pub capacity_limit: f32,
}

#[derive(Deserialize, Debug)]
pub struct RawEnvironment {
    pub heat_current: f32,
    pub heat_generated_per_tick: f32,
    pub heat_dissipation_rate: f32,
    pub heat_warning_threshold: f32,
    pub heat_death_threshold: f32,
    pub waste_current: f32,
    pub waste_generated_per_tick: f32,
    pub waste_sink_rate: f32,
    pub waste_warning_threshold: f32,
    pub waste_death_threshold: f32,
}

#[derive(Deserialize, Debug)]
pub struct RawLifecycle {
    pub stress_energy_threshold: f32,
    pub dormancy_allowed: bool,
    pub dormant_mandatory_cost_modifier: Option<f32>,
    pub critical_capacity_overrun: f32,
}

#[derive(Deserialize, Debug)]
pub struct RawGrowth {
    pub growth_cost_resource: f32,
    pub growth_cost_energy: f32,
    pub growth_target_radius: f32,
    pub max_division_pressure: f32,
}

#[derive(Deserialize, Debug)]
pub struct RawSynthesis {
    pub cost_resource: Option<f32>,
    pub cost_energy: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct RawDivision {
    pub enabled: Option<bool>,
    pub energy_cost: Option<f32>,
    pub split_ratio: Option<f32>,
    pub daughter_spacing: Option<f32>,
    pub min_daughter_radius: Option<f32>,
    pub partition_loss_fraction: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct RawDecomposition {
    pub enabled: Option<bool>,
    pub resource_layer_index: Option<usize>,
    pub resources_per_tick: Option<f32>,
    pub materials_per_tick: Option<f32>,
    pub remove_when_empty: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct RawContractility {
    pub energy_cost: Option<f32>,
    pub force_factor: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct RawResourceInteraction {
    pub enabled: Option<bool>,
    pub uptake_layer_index: Option<usize>,
    pub max_uptake_per_tick: Option<f32>,
    pub metabolism_resource_per_tick: Option<f32>,
    pub energy_per_resource: Option<f32>,
    pub heat_per_resource: Option<f32>,
    pub waste_per_resource: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct RawLocalInteraction {
    pub enabled: Option<bool>,
    pub contact_exchange_rate: Option<f32>,
    pub max_exchange_per_pair: Option<f32>,
    pub min_boundary_capability: Option<f32>,
    pub min_transport_capability: Option<f32>,
    pub contact_stimulus_per_overlap: Option<f32>,
    pub stimulus_decay_per_tick: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct RawMaterialEffects {
    pub transport_uptake_per_unit: Option<f32>,
    pub metabolic_conversion_per_unit: Option<f32>,
    pub storage_capacity_per_unit: Option<f32>,
    pub structural_growth_per_unit: Option<f32>,
    pub contractile_force_per_unit: Option<f32>,
    pub sensory_input_per_unit: Option<f32>,
    pub boundary_retention_per_unit: Option<f32>,
    pub repair_stress_buffer_per_unit: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct RawScenarioConfig {
    pub scenario_id: String,
    pub seed: u64,
    pub tick_count: u64,
    pub legacy_material_distribution: Option<bool>,
    pub world: RawWorld,
    pub space: RawSpace,
    pub resources: RawResources,
    pub resource_interaction: Option<RawResourceInteraction>,
    pub local_interaction: Option<RawLocalInteraction>,
    pub cell: RawCell,
    pub cells: Option<Vec<RawCell>>,
    pub environment: RawEnvironment,
    pub lifecycle: RawLifecycle,
    pub growth: Option<RawGrowth>,
    pub synthesis: Option<RawSynthesis>,
    pub contractility: Option<RawContractility>,
    pub division: Option<RawDivision>,
    pub decomposition: Option<RawDecomposition>,
    pub material_effects: Option<RawMaterialEffects>,
}

#[derive(Debug)]
pub enum ParseError {
    TomlError(toml::de::Error),
    ConfigValidationError(ConfigError),
    ValidationError(String),
    UnknownMaterialName(String),
}

impl RawScenarioConfig {
    pub fn parse(toml_str: &str) -> Result<RuntimeConfig, ParseError> {
        let raw: Self = toml::from_str(toml_str).map_err(ParseError::TomlError)?;
        raw.to_runtime_config()
    }

    pub fn to_runtime_config(self) -> Result<RuntimeConfig, ParseError> {
        if self.environment.heat_warning_threshold > self.environment.heat_death_threshold {
            return Err(ParseError::ValidationError(
                "heat_warning_threshold exceeds heat_death_threshold".to_string(),
            ));
        }
        if self.environment.waste_warning_threshold > self.environment.waste_death_threshold {
            return Err(ParseError::ValidationError(
                "waste_warning_threshold exceeds waste_death_threshold".to_string(),
            ));
        }

        let legacy = self.legacy_material_distribution.unwrap_or(false);
        let initial_resources_sum: f32 = self.cell.initial_resources.values().sum();
        let (
            boundary,
            transport,
            metabolic,
            storage,
            synthesis,
            structural,
            repair,
            contractile,
            sensory,
        ) = parse_materials_inventory(&self.cell.initial_materials, legacy)?;

        let size = WorldSize::new(self.world.size[0], self.world.size[1])
            .map_err(|e| ParseError::ValidationError(format!("Invalid world size: {:?}", e)))?;
        let optional_decay_rate = self.resources.optional_decay_rate.unwrap_or(0.0);

        if self.resources.resource_type_ids.len() != self.resources.initial_distribution.len() {
            return Err(ParseError::ValidationError(
                "resource_type_ids length must match initial_distribution length".to_string(),
            ));
        }

        let resource_amounts = self
            .resources
            .initial_distribution
            .iter()
            .map(|value| {
                ResourceAmount::new(*value).map_err(|e| {
                    ParseError::ValidationError(format!(
                        "Invalid resource initial_distribution: {:?}",
                        e
                    ))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let resources = ResourceConfig::new(resource_amounts, optional_decay_rate)
            .map_err(ParseError::ConfigValidationError)?;

        let resource_interaction = if let Some(raw_interaction) = self.resource_interaction {
            ResourceInteractionConfig {
                enabled: raw_interaction.enabled.unwrap_or(false),
                uptake_layer_index: raw_interaction.uptake_layer_index.unwrap_or(0),
                max_uptake_per_tick: ResourceAmount::new(
                    raw_interaction.max_uptake_per_tick.unwrap_or(0.0),
                )
                .map_err(|e| {
                    ParseError::ValidationError(format!("Invalid max_uptake_per_tick: {:?}", e))
                })?,
                metabolism_resource_per_tick: ResourceAmount::new(
                    raw_interaction.metabolism_resource_per_tick.unwrap_or(0.0),
                )
                .map_err(|e| {
                    ParseError::ValidationError(format!(
                        "Invalid metabolism_resource_per_tick: {:?}",
                        e
                    ))
                })?,
                energy_per_resource: raw_interaction.energy_per_resource.unwrap_or(0.0),
                heat_per_resource: raw_interaction.heat_per_resource.unwrap_or(0.0),
                waste_per_resource: raw_interaction.waste_per_resource.unwrap_or(0.0),
            }
        } else {
            ResourceInteractionConfig::disabled()
        };

        let world = WorldConfig {
            tick_count: Tick::from_raw(self.tick_count),
            seed: Seed::from_raw(self.seed),
            size,
        };

        let space = SpaceConfig {
            spatial_grid_size: self.space.spatial_grid_size,
            physics_solver_iterations: self.space.physics_solver_iterations.unwrap_or(4),
        };

        let passive_income = self
            .resources
            .passive_energy_income_placeholder
            .unwrap_or(0.0);

        let cell = CellInitialConfig {
            position: Position::new(self.cell.initial_position[0], self.cell.initial_position[1]),
            radius: Radius::new(self.cell.radius)
                .map_err(|e| ParseError::ValidationError(format!("Invalid radius: {:?}", e)))?,
            initial_energy: EnergyAmount::new(self.cell.initial_energy).map_err(|e| {
                ParseError::ValidationError(format!("Invalid initial_energy: {:?}", e))
            })?,
            energy_capacity: EnergyAmount::new(self.cell.energy_capacity).map_err(|e| {
                ParseError::ValidationError(format!("Invalid energy_capacity: {:?}", e))
            })?,
            mandatory_cost_per_tick: EnergyAmount::new(self.cell.mandatory_cost_per_tick).map_err(
                |e| {
                    ParseError::ValidationError(format!("Invalid mandatory_cost_per_tick: {:?}", e))
                },
            )?,
            passive_energy_income: EnergyAmount::new(passive_income).map_err(|e| {
                ParseError::ValidationError(format!("Invalid passive_energy_income: {:?}", e))
            })?,
            capacity_limit: CapacityAmount::new(self.cell.capacity_limit).map_err(|e| {
                ParseError::ValidationError(format!("Invalid capacity_limit: {:?}", e))
            })?,
            initial_resource_amount: ResourceAmount::new(initial_resources_sum).map_err(|e| {
                ParseError::ValidationError(format!("Invalid initial_resource_amount: {:?}", e))
            })?,
            initial_boundary_material: boundary,
            initial_transport_material: transport,
            initial_metabolic_material: metabolic,
            initial_storage_material: storage,
            initial_synthesis_material: synthesis,
            initial_structural_material: structural,
            initial_repair_material: repair,
            initial_contractile_material: contractile,
            initial_sensory_material: sensory,
        };

        let environment = EnvironmentConfig {
            heat_current: HeatAmount::new(self.environment.heat_current).map_err(|e| {
                ParseError::ValidationError(format!("Invalid heat_current: {:?}", e))
            })?,
            heat_generated_per_tick: HeatAmount::new(self.environment.heat_generated_per_tick)
                .map_err(|e| {
                    ParseError::ValidationError(format!("Invalid heat_generated_per_tick: {:?}", e))
                })?,
            heat_dissipation_rate: HeatAmount::new(self.environment.heat_dissipation_rate)
                .map_err(|e| {
                    ParseError::ValidationError(format!("Invalid heat_dissipation_rate: {:?}", e))
                })?,
            heat_warning_threshold: HeatAmount::new(self.environment.heat_warning_threshold)
                .map_err(|e| {
                    ParseError::ValidationError(format!("Invalid heat_warning_threshold: {:?}", e))
                })?,
            heat_death_threshold: HeatAmount::new(self.environment.heat_death_threshold).map_err(
                |e| ParseError::ValidationError(format!("Invalid heat_death_threshold: {:?}", e)),
            )?,
            waste_current: WasteAmount::new(self.environment.waste_current).map_err(|e| {
                ParseError::ValidationError(format!("Invalid waste_current: {:?}", e))
            })?,
            waste_generated_per_tick: WasteAmount::new(self.environment.waste_generated_per_tick)
                .map_err(|e| {
                ParseError::ValidationError(format!("Invalid waste_generated_per_tick: {:?}", e))
            })?,
            waste_sink_rate: WasteAmount::new(self.environment.waste_sink_rate).map_err(|e| {
                ParseError::ValidationError(format!("Invalid waste_sink_rate: {:?}", e))
            })?,
            waste_warning_threshold: WasteAmount::new(self.environment.waste_warning_threshold)
                .map_err(|e| {
                    ParseError::ValidationError(format!("Invalid waste_warning_threshold: {:?}", e))
                })?,
            waste_death_threshold: WasteAmount::new(self.environment.waste_death_threshold)
                .map_err(|e| {
                    ParseError::ValidationError(format!("Invalid waste_death_threshold: {:?}", e))
                })?,
        };

        let dormant_modifier = self
            .cell
            .dormant_mandatory_cost_modifier
            .or(self.lifecycle.dormant_mandatory_cost_modifier)
            .unwrap_or(0.1);

        let lifecycle = LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(self.lifecycle.stress_energy_threshold)
                .map_err(|e| {
                    ParseError::ValidationError(format!("Invalid stress_energy_threshold: {:?}", e))
                })?,
            dormancy_allowed: self.lifecycle.dormancy_allowed,
            dormant_mandatory_cost_modifier: dormant_modifier,
            critical_capacity_overrun: CapacityAmount::new(
                self.lifecycle.critical_capacity_overrun,
            )
            .map_err(|e| {
                ParseError::ValidationError(format!("Invalid critical_capacity_overrun: {:?}", e))
            })?,
        };

        let mut initial_cells = Vec::new();
        if let Some(ref raw_cells) = self.cells {
            for raw_cell in raw_cells {
                let cell_initial_resources_sum: f32 = raw_cell.initial_resources.values().sum();
                let (
                    boundary_c,
                    transport_c,
                    metabolic_c,
                    storage_c,
                    synthesis_c,
                    structural_c,
                    repair_c,
                    contractile_c,
                    sensory_c,
                ) = parse_materials_inventory(&raw_cell.initial_materials, legacy)?;
                let cell_conf = CellInitialConfig {
                    position: Position::new(
                        raw_cell.initial_position[0],
                        raw_cell.initial_position[1],
                    ),
                    radius: Radius::new(raw_cell.radius).map_err(|e| {
                        ParseError::ValidationError(format!("Invalid cell radius: {:?}", e))
                    })?,
                    initial_energy: EnergyAmount::new(raw_cell.initial_energy).map_err(|e| {
                        ParseError::ValidationError(format!("Invalid initial_energy: {:?}", e))
                    })?,
                    energy_capacity: EnergyAmount::new(raw_cell.energy_capacity).map_err(|e| {
                        ParseError::ValidationError(format!("Invalid energy_capacity: {:?}", e))
                    })?,
                    mandatory_cost_per_tick: EnergyAmount::new(raw_cell.mandatory_cost_per_tick)
                        .map_err(|e| {
                            ParseError::ValidationError(format!("Invalid mandatory_cost: {:?}", e))
                        })?,
                    passive_energy_income: EnergyAmount::zero(),
                    capacity_limit: CapacityAmount::new(raw_cell.capacity_limit).map_err(|e| {
                        ParseError::ValidationError(format!("Invalid capacity_limit: {:?}", e))
                    })?,
                    initial_resource_amount: ResourceAmount::new(cell_initial_resources_sum)
                        .map_err(|e| {
                            ParseError::ValidationError(format!(
                                "Invalid initial_resource_amount: {:?}",
                                e
                            ))
                        })?,
                    initial_boundary_material: boundary_c,
                    initial_transport_material: transport_c,
                    initial_metabolic_material: metabolic_c,
                    initial_storage_material: storage_c,
                    initial_synthesis_material: synthesis_c,
                    initial_structural_material: structural_c,
                    initial_repair_material: repair_c,
                    initial_contractile_material: contractile_c,
                    initial_sensory_material: sensory_c,
                };
                initial_cells.push(cell_conf);
            }
        }

        let mut runtime_config = RuntimeConfig::new(
            world,
            space,
            resources,
            resource_interaction,
            cell,
            environment,
            lifecycle,
        )
        .map_err(ParseError::ConfigValidationError)?;

        if let Some(ref raw_growth) = self.growth {
            runtime_config.growth = GrowthConfig {
                growth_cost_resource: ResourceAmount::new(raw_growth.growth_cost_resource)
                    .map_err(|e| {
                        ParseError::ValidationError(format!(
                            "Invalid growth_cost_resource: {:?}",
                            e
                        ))
                    })?,
                growth_cost_energy: EnergyAmount::new(raw_growth.growth_cost_energy).map_err(
                    |e| ParseError::ValidationError(format!("Invalid growth_cost_energy: {:?}", e)),
                )?,
                growth_target_radius: Radius::new(raw_growth.growth_target_radius).map_err(
                    |e| {
                        ParseError::ValidationError(format!(
                            "Invalid growth_target_radius: {:?}",
                            e
                        ))
                    },
                )?,
                max_division_pressure: raw_growth.max_division_pressure,
            };
            runtime_config.growth_enabled = true;
        }

        if let Some(ref raw_synth) = self.synthesis {
            let cost_res = raw_synth.cost_resource.unwrap_or(1.0);
            let cost_eng = raw_synth.cost_energy.unwrap_or(5.0);
            runtime_config.synthesis = SynthesisConfig {
                cost_resource: ResourceAmount::new(cost_res).map_err(|e| {
                    ParseError::ValidationError(format!("Invalid synthesis cost_resource: {:?}", e))
                })?,
                cost_energy: EnergyAmount::new(cost_eng).map_err(|e| {
                    ParseError::ValidationError(format!("Invalid synthesis cost_energy: {:?}", e))
                })?,
            };
        }

        if let Some(ref raw_contract) = self.contractility {
            let cost_eng = raw_contract.energy_cost.unwrap_or(1.0);
            let force = raw_contract.force_factor.unwrap_or(0.1);
            runtime_config.contractility = ContractilityConfig {
                energy_cost: EnergyAmount::new(cost_eng).map_err(|e| {
                    ParseError::ValidationError(format!(
                        "Invalid contractility energy_cost: {:?}",
                        e
                    ))
                })?,
                force_factor: force,
            };
        }

        if let Some(ref raw_div) = self.division {
            runtime_config.division = DivisionConfig {
                enabled: raw_div.enabled.unwrap_or(false),
                energy_cost: EnergyAmount::new(raw_div.energy_cost.unwrap_or(0.0)).map_err(
                    |e| {
                        ParseError::ValidationError(format!(
                            "Invalid division energy_cost: {:?}",
                            e
                        ))
                    },
                )?,
                split_ratio: raw_div.split_ratio.unwrap_or(0.5),
                daughter_spacing: raw_div.daughter_spacing.unwrap_or(0.25),
                min_daughter_radius: Radius::new(raw_div.min_daughter_radius.unwrap_or(0.5))
                    .map_err(|e| {
                        ParseError::ValidationError(format!(
                            "Invalid division min_daughter_radius: {:?}",
                            e
                        ))
                    })?,
                partition_loss_fraction: raw_div.partition_loss_fraction.unwrap_or(0.0),
            };
        }

        if let Some(ref raw_dec) = self.decomposition {
            runtime_config.decomposition = DecompositionConfig {
                enabled: raw_dec.enabled.unwrap_or(false),
                resource_layer_index: raw_dec.resource_layer_index.unwrap_or(0),
                resources_per_tick: ResourceAmount::new(raw_dec.resources_per_tick.unwrap_or(0.0))
                    .map_err(|e| {
                        ParseError::ValidationError(format!(
                            "Invalid decomposition resources_per_tick: {:?}",
                            e
                        ))
                    })?,
                materials_per_tick: MaterialAmount::new(raw_dec.materials_per_tick.unwrap_or(0.0))
                    .map_err(|e| {
                        ParseError::ValidationError(format!(
                            "Invalid decomposition materials_per_tick: {:?}",
                            e
                        ))
                    })?,
                remove_when_empty: raw_dec.remove_when_empty.unwrap_or(false),
            };
        }

        if let Some(ref raw_effects) = self.material_effects {
            let defaults = MaterialEffectConfig::default();
            runtime_config.material_effects = MaterialEffectConfig {
                transport_uptake_per_unit: raw_effects
                    .transport_uptake_per_unit
                    .unwrap_or(defaults.transport_uptake_per_unit),
                metabolic_conversion_per_unit: raw_effects
                    .metabolic_conversion_per_unit
                    .unwrap_or(defaults.metabolic_conversion_per_unit),
                storage_capacity_per_unit: raw_effects
                    .storage_capacity_per_unit
                    .unwrap_or(defaults.storage_capacity_per_unit),
                structural_growth_per_unit: raw_effects
                    .structural_growth_per_unit
                    .unwrap_or(defaults.structural_growth_per_unit),
                contractile_force_per_unit: raw_effects
                    .contractile_force_per_unit
                    .unwrap_or(defaults.contractile_force_per_unit),
                sensory_input_per_unit: raw_effects
                    .sensory_input_per_unit
                    .unwrap_or(defaults.sensory_input_per_unit),
                boundary_retention_per_unit: raw_effects
                    .boundary_retention_per_unit
                    .unwrap_or(defaults.boundary_retention_per_unit),
                repair_stress_buffer_per_unit: raw_effects
                    .repair_stress_buffer_per_unit
                    .unwrap_or(defaults.repair_stress_buffer_per_unit),
            };
            for (name, value) in [
                (
                    "transport_uptake_per_unit",
                    runtime_config.material_effects.transport_uptake_per_unit,
                ),
                (
                    "metabolic_conversion_per_unit",
                    runtime_config
                        .material_effects
                        .metabolic_conversion_per_unit,
                ),
                (
                    "storage_capacity_per_unit",
                    runtime_config.material_effects.storage_capacity_per_unit,
                ),
                (
                    "structural_growth_per_unit",
                    runtime_config.material_effects.structural_growth_per_unit,
                ),
                (
                    "contractile_force_per_unit",
                    runtime_config.material_effects.contractile_force_per_unit,
                ),
                (
                    "sensory_input_per_unit",
                    runtime_config.material_effects.sensory_input_per_unit,
                ),
                (
                    "boundary_retention_per_unit",
                    runtime_config.material_effects.boundary_retention_per_unit,
                ),
                (
                    "repair_stress_buffer_per_unit",
                    runtime_config
                        .material_effects
                        .repair_stress_buffer_per_unit,
                ),
            ] {
                if !value.is_finite() || value < 0.0 {
                    return Err(ParseError::ValidationError(format!(
                        "Invalid material effect {}: {}",
                        name, value
                    )));
                }
            }
        }

        if let Some(ref raw_local) = self.local_interaction {
            runtime_config.local_interaction.enabled = raw_local.enabled.unwrap_or(false);
            runtime_config.local_interaction.contact_exchange_rate =
                raw_local.contact_exchange_rate.unwrap_or(0.0);
            runtime_config.local_interaction.max_exchange_per_pair = ResourceAmount::new(
                raw_local.max_exchange_per_pair.unwrap_or(0.0),
            )
            .map_err(|e| {
                ParseError::ValidationError(format!(
                    "Invalid local_interaction max_exchange_per_pair: {:?}",
                    e
                ))
            })?;
            runtime_config.local_interaction.min_boundary_capability =
                raw_local.min_boundary_capability.unwrap_or(0.0);
            runtime_config.local_interaction.min_transport_capability =
                raw_local.min_transport_capability.unwrap_or(0.0);
            runtime_config
                .local_interaction
                .contact_stimulus_per_overlap =
                raw_local.contact_stimulus_per_overlap.unwrap_or(0.0);
            runtime_config.local_interaction.stimulus_decay_per_tick =
                raw_local.stimulus_decay_per_tick.unwrap_or(0.0);
        }

        runtime_config
            .validate_phase2d_options()
            .map_err(ParseError::ConfigValidationError)?;
        runtime_config
            .validate_phase2f_options()
            .map_err(ParseError::ConfigValidationError)?;

        if !initial_cells.is_empty() {
            runtime_config = runtime_config.with_cells(initial_cells);
        }

        Ok(runtime_config)
    }
}

#[allow(clippy::type_complexity)]
fn parse_materials_inventory(
    initial_materials: &std::collections::HashMap<String, f32>,
    legacy: bool,
) -> Result<
    (
        MaterialAmount,
        MaterialAmount,
        MaterialAmount,
        MaterialAmount,
        MaterialAmount,
        MaterialAmount,
        MaterialAmount,
        MaterialAmount,
        MaterialAmount,
    ),
    ParseError,
> {
    let mut boundary = 0.0;
    let mut transport = 0.0;
    let mut metabolic = 0.0;
    let mut storage = 0.0;
    let mut synthesis = 0.0;
    let mut structural = 0.0;
    let mut repair = 0.0;
    let mut contractile = 0.0;
    let mut sensory = 0.0;

    let has_specific = initial_materials.keys().any(|k| {
        matches!(
            k.as_str(),
            "boundary"
                | "transport"
                | "metabolic"
                | "storage"
                | "synthesis"
                | "structural"
                | "repair"
                | "contractile"
                | "sensory"
        )
    });

    let initial_materials_sum: f32 = initial_materials.values().sum();

    if !has_specific && initial_materials_sum > 0.0 {
        if legacy {
            let share = initial_materials_sum / 9.0;
            boundary = share;
            transport = share;
            metabolic = share;
            storage = share;
            synthesis = share;
            structural = share;
            repair = share;
            contractile = share;
            sensory = share;
        } else {
            if let Some(k) = initial_materials.keys().next() {
                return Err(ParseError::UnknownMaterialName(k.clone()));
            }
        }
    } else {
        for (k, &v) in initial_materials {
            match k.as_str() {
                "boundary" | "membrane" | "envelope" => boundary = v,
                "transport" | "pump" => transport = v,
                "metabolic" | "metabolism" | "converter" => metabolic = v,
                "storage" | "vacuolar" => storage = v,
                "synthesis" | "producer" => synthesis = v,
                "structural" | "skeleton" | "wall" | "cell_wall" => structural = v,
                "repair" => repair = v,
                "contractile" | "motor" => contractile = v,
                "sensory" | "receptor" => sensory = v,
                other => {
                    if legacy {
                        structural = v;
                    } else {
                        return Err(ParseError::UnknownMaterialName(other.to_string()));
                    }
                }
            }
        }
    }

    let wrap = |val: f32, name: &str| {
        MaterialAmount::new(val).map_err(|e| {
            ParseError::ValidationError(format!("Invalid initial {} material: {:?}", name, e))
        })
    };

    Ok((
        wrap(boundary, "boundary")?,
        wrap(transport, "transport")?,
        wrap(metabolic, "metabolic")?,
        wrap(storage, "storage")?,
        wrap(synthesis, "synthesis")?,
        wrap(structural, "structural")?,
        wrap(repair, "repair")?,
        wrap(contractile, "contractile")?,
        wrap(sensory, "sensory")?,
    ))
}
