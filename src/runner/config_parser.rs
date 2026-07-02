use crate::core::config::{
    CellInitialConfig, ConfigError, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
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
pub struct RawScenarioConfig {
    pub scenario_id: String,
    pub seed: u64,
    pub tick_count: u64,
    pub world: RawWorld,
    pub space: RawSpace,
    pub resources: RawResources,
    pub resource_interaction: Option<RawResourceInteraction>,
    pub cell: RawCell,
    pub environment: RawEnvironment,
    pub lifecycle: RawLifecycle,
}

#[derive(Debug)]
pub enum ParseError {
    TomlError(toml::de::Error),
    ConfigValidationError(ConfigError),
    ValidationError(String),
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

        let initial_resources_sum: f32 = self.cell.initial_resources.values().sum();
        let initial_materials_sum: f32 = self.cell.initial_materials.values().sum();

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
            initial_material_amount: MaterialAmount::new(initial_materials_sum).map_err(|e| {
                ParseError::ValidationError(format!("Invalid initial_material_amount: {:?}", e))
            })?,
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

        RuntimeConfig::new(
            world,
            space,
            resources,
            resource_interaction,
            cell,
            environment,
            lifecycle,
        )
        .map_err(ParseError::ConfigValidationError)
    }
}
