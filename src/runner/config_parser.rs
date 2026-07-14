use crate::core::config::{
    CellInitialConfig, ChemistryBoundaryConfig, ChemistryConfig, ChemistryHeatConfig,
    ChemistryMaterialConfig, ChemistryReactionConfig, ChemistryRepairConfig,
    ChemistryResourceConfig, ConfigError, ContractilityConfig, DecompositionConfig, DivisionConfig,
    EnvironmentConfig, GrowthConfig, LifecycleConfig, MaterialEffectConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, SynthesisConfig, WorldConfig,
};
use crate::core::genome::{
    GenomeCarrierState, GenomeOutputId, GenomeOutputValue, GenomeTemplate, GenomeTemplateId,
};
use crate::core::ids::{MaterialTypeId, ResourceTypeId};
use crate::core::material_types::{
    MaterialProperties, MaterialRegistry, ReactionProfile, RepairRequirements, SignalProperties,
};
use crate::core::resource_types::{
    PermeabilityConstraint, ReactivityProfile, ResourceProperties, ResourceRegistry, ResourceTags,
};
use crate::core::units::{
    CapacityAmount, DecayRate, DiffusionRate, EnergyAmount, EnergyCapacity, EnergyValue,
    HeatAmount, MaterialAmount, Position, Radius, ResourceAmount, Seed, SignalAmount, Strength,
    Tick, Volume, WasteAmount, WorldSize,
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
    pub genome: Option<RawCellGenome>,
}

#[derive(Deserialize, Debug)]
pub struct RawCellGenome {
    pub template: String,
}

#[derive(Deserialize, Debug)]
pub struct RawGenomeTemplate {
    pub variation_amplitude: f32,
    pub runtime_interval_ticks: Option<u64>,
    pub carrier: RawGenomeCarrier,
    #[serde(default)]
    pub outputs: HashMap<String, f32>,
}

#[derive(Deserialize, Debug)]
pub struct RawGenomeCarrier {
    pub material_id: String,
    pub amount: f32,
    pub integrity: f32,
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
pub struct RawJointConfig {
    pub enabled: Option<bool>,
    pub creation_distance_margin: Option<f32>,
    pub creation_material_cost: Option<f32>,
    pub creation_resource_cost: Option<f32>,
    pub creation_energy_cost: Option<f32>,
    pub upkeep_material_decay_per_tick: Option<f32>,
    pub break_damage_threshold: Option<f32>,
    pub max_joints_per_cell: Option<u32>,
    pub mechanical_strength: Option<f32>,
    pub resource_transfer_rate: Option<f32>,
    pub max_resource_transfer_per_tick: Option<f32>,
    pub signal_conductivity: Option<f32>,
    pub signal_decay: Option<f32>,
    pub heat_conductivity: Option<f32>,
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

#[derive(Deserialize, Debug, Default)]
pub struct RawChemistry {
    #[serde(default)]
    pub resources: HashMap<String, RawChemistryResource>,
    #[serde(default)]
    pub materials: HashMap<String, RawChemistryMaterial>,
    #[serde(default)]
    pub reactions: HashMap<String, RawChemistryReaction>,
    pub heat: Option<RawChemistryHeat>,
    pub boundary: Option<RawChemistryBoundary>,
    pub repair: Option<RawChemistryRepair>,
}

#[derive(Deserialize, Debug)]
pub struct RawChemistryResource {
    pub volume: f32,
    pub diffusion_rate: f32,
    pub energy_value: f32,
    pub decay_rate: f32,
    pub reactivity_profile: String,
    pub permeability: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct RawChemistryMaterial {
    pub volume: f32,
    pub stability: f32,
    pub strength: f32,
    pub permeability: f32,
    pub energy_capacity: f32,
    pub decay_rate: f32,
    pub repair_resource: String,
    pub repair_amount: f32,
}

#[derive(Deserialize, Debug)]
pub struct RawChemistryReaction {
    pub mode: String,
    pub process_id: Option<String>,
    #[serde(default)]
    pub inputs: HashMap<String, f32>,
    #[serde(default)]
    pub required_materials: HashMap<String, f32>,
    #[serde(default)]
    pub outputs: HashMap<String, f32>,
    pub configured_sink_amount: f32,
    pub energy_output: f32,
    pub heat_output: f32,
    pub rate: f32,
    pub probability: f32,
    pub accounting_destination: String,
}

#[derive(Deserialize, Debug)]
pub struct RawChemistryHeat {
    pub capacity: f32,
    pub dissipation_rate: f32,
    pub warning_threshold: f32,
    pub death_threshold: f32,
}

#[derive(Deserialize, Debug)]
pub struct RawChemistryBoundary {
    pub default_permeability: String,
    pub retention_rate: f32,
}

#[derive(Deserialize, Debug)]
pub struct RawChemistryRepair {
    pub enabled: bool,
    pub energy_cost: f32,
    pub max_amount_per_tick: f32,
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
    pub joints: Option<RawJointConfig>,
    pub chemistry: Option<RawChemistry>,
    #[serde(default)]
    pub genome_templates: HashMap<String, RawGenomeTemplate>,
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

        if let Some(ref raw_joints) = self.joints {
            runtime_config.joints.enabled = raw_joints.enabled.unwrap_or(false);
            runtime_config.joints.creation_distance_margin = raw_joints
                .creation_distance_margin
                .unwrap_or(runtime_config.joints.creation_distance_margin);
            runtime_config.joints.creation_material_cost = MaterialAmount::new(
                raw_joints
                    .creation_material_cost
                    .unwrap_or(runtime_config.joints.creation_material_cost.raw()),
            )
            .map_err(|e| {
                ParseError::ValidationError(format!(
                    "Invalid joints creation_material_cost: {:?}",
                    e
                ))
            })?;
            runtime_config.joints.creation_resource_cost = ResourceAmount::new(
                raw_joints
                    .creation_resource_cost
                    .unwrap_or(runtime_config.joints.creation_resource_cost.raw()),
            )
            .map_err(|e| {
                ParseError::ValidationError(format!(
                    "Invalid joints creation_resource_cost: {:?}",
                    e
                ))
            })?;
            runtime_config.joints.creation_energy_cost = EnergyAmount::new(
                raw_joints
                    .creation_energy_cost
                    .unwrap_or(runtime_config.joints.creation_energy_cost.raw()),
            )
            .map_err(|e| {
                ParseError::ValidationError(format!("Invalid joints creation_energy_cost: {:?}", e))
            })?;
            runtime_config.joints.upkeep_material_decay_per_tick = raw_joints
                .upkeep_material_decay_per_tick
                .unwrap_or(runtime_config.joints.upkeep_material_decay_per_tick);
            runtime_config.joints.break_damage_threshold = raw_joints
                .break_damage_threshold
                .unwrap_or(runtime_config.joints.break_damage_threshold);
            runtime_config.joints.max_joints_per_cell = raw_joints
                .max_joints_per_cell
                .unwrap_or(runtime_config.joints.max_joints_per_cell);
            runtime_config.joints.mechanical_strength = raw_joints
                .mechanical_strength
                .unwrap_or(runtime_config.joints.mechanical_strength);
            runtime_config.joints.resource_transfer_rate = raw_joints
                .resource_transfer_rate
                .unwrap_or(runtime_config.joints.resource_transfer_rate);
            runtime_config.joints.max_resource_transfer_per_tick = ResourceAmount::new(
                raw_joints
                    .max_resource_transfer_per_tick
                    .unwrap_or(runtime_config.joints.max_resource_transfer_per_tick.raw()),
            )
            .map_err(|e| {
                ParseError::ValidationError(format!(
                    "Invalid joints max_resource_transfer_per_tick: {:?}",
                    e
                ))
            })?;
            runtime_config.joints.signal_conductivity = raw_joints
                .signal_conductivity
                .unwrap_or(runtime_config.joints.signal_conductivity);
            runtime_config.joints.signal_decay = raw_joints
                .signal_decay
                .unwrap_or(runtime_config.joints.signal_decay);
            runtime_config.joints.heat_conductivity = raw_joints
                .heat_conductivity
                .unwrap_or(runtime_config.joints.heat_conductivity);
        }

        runtime_config.chemistry = parse_chemistry(
            self.chemistry.unwrap_or_default(),
            &self.resources.resource_type_ids,
        )?;
        runtime_config.initial_typed_resources = if runtime_config.chemistry.resources.is_empty() {
            if let Some(raw_cells) = &self.cells {
                vec![Vec::new(); raw_cells.len()]
            } else {
                vec![Vec::new()]
            }
        } else {
            let mut raw_cell_inventories = vec![typed_resource_inventory(
                &self.cell,
                &self.resources.resource_type_ids,
            )?];
            if let Some(raw_cells) = &self.cells {
                raw_cell_inventories = raw_cells
                    .iter()
                    .map(|cell| typed_resource_inventory(cell, &self.resources.resource_type_ids))
                    .collect::<Result<_, _>>()?;
            }
            raw_cell_inventories
        };

        runtime_config
            .validate_phase2d_options()
            .map_err(ParseError::ConfigValidationError)?;
        runtime_config
            .validate_phase2f_options()
            .map_err(ParseError::ConfigValidationError)?;
        runtime_config
            .validate_phase2h_options()
            .map_err(ParseError::ConfigValidationError)?;

        if !initial_cells.is_empty() {
            runtime_config = runtime_config.with_cells(initial_cells);
        }

        let genome_templates = parse_genome_templates(&self.genome_templates)?;
        let known_templates: std::collections::HashSet<_> = genome_templates
            .iter()
            .map(|template| template.id().as_str())
            .collect();
        let mut initial_cell_genome_templates = vec![
            self.cell
                .genome
                .as_ref()
                .map(|genome| GenomeTemplateId::new(genome.template.clone()))
                .transpose()
                .map_err(|error| {
                    ParseError::ValidationError(format!(
                        "Invalid Genome template reference: {error:?}"
                    ))
                })?,
        ];

        if let Some(raw_cells) = &self.cells {
            initial_cell_genome_templates = raw_cells
                .iter()
                .map(|cell| {
                    cell.genome
                        .as_ref()
                        .map(|genome| GenomeTemplateId::new(genome.template.clone()))
                        .transpose()
                        .map_err(|error| {
                            ParseError::ValidationError(format!(
                                "Invalid Genome template reference: {error:?}"
                            ))
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
        }

        for assignment in initial_cell_genome_templates.iter().flatten() {
            if !known_templates.contains(assignment.as_str()) {
                return Err(ParseError::ValidationError(format!(
                    "Unknown Genome template: {}",
                    assignment.as_str()
                )));
            }
        }

        runtime_config.genome_templates = genome_templates;
        runtime_config.initial_cell_genome_templates = initial_cell_genome_templates;

        Ok(runtime_config)
    }
}

fn parse_genome_templates(
    raw: &HashMap<String, RawGenomeTemplate>,
) -> Result<Vec<GenomeTemplate>, ParseError> {
    let mut names: Vec<_> = raw.keys().cloned().collect();
    names.sort();
    names
        .into_iter()
        .map(|name| {
            let value = &raw[&name];
            let outputs = value
                .outputs
                .iter()
                .map(|(id, output)| {
                    let id = GenomeOutputId::parse(id).map_err(|_| {
                        ParseError::ValidationError(format!("Unknown Genome output: {id}"))
                    })?;
                    Ok((id, GenomeOutputValue::new(*output)))
                })
                .collect::<Result<Vec<_>, ParseError>>()?;
            GenomeTemplate::new(
                GenomeTemplateId::new(name.clone()).map_err(|error| {
                    ParseError::ValidationError(format!("Invalid Genome template id: {error:?}"))
                })?,
                value.variation_amplitude,
                value.runtime_interval_ticks.unwrap_or(1),
                GenomeCarrierState::new(
                    value.carrier.material_id.clone(),
                    value.carrier.amount,
                    value.carrier.integrity,
                )
                .map_err(|error| {
                    ParseError::ValidationError(format!("Invalid Genome carrier: {error:?}"))
                })?,
                outputs,
            )
            .map_err(|error| {
                ParseError::ValidationError(format!("Invalid Genome template: {error:?}"))
            })
        })
        .collect()
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

fn parse_chemistry(
    raw: RawChemistry,
    declared_resource_ids: &[String],
) -> Result<ChemistryConfig, ParseError> {
    if raw.resources.is_empty()
        && raw.materials.is_empty()
        && raw.reactions.is_empty()
        && raw.heat.is_none()
        && raw.boundary.is_none()
        && raw.repair.is_none()
    {
        return Ok(ChemistryConfig::default());
    }
    let mut declared = declared_resource_ids.to_vec();
    declared.sort();
    if declared.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(ParseError::ValidationError(
            "Duplicate resource type id".to_string(),
        ));
    }
    if declared.iter().any(|id| !raw.resources.contains_key(id)) {
        return Err(ParseError::ValidationError(
            "Unknown declared resource type id".to_string(),
        ));
    }
    if raw.resources.len() != declared.len() {
        return Err(ParseError::ValidationError(
            "Chemistry resources must match declared resource type ids".to_string(),
        ));
    }
    let resource_names = declared;
    let mut material_names: Vec<_> = raw.materials.keys().cloned().collect();
    material_names.sort();

    let mut resource_values = Vec::new();
    let mut resource_types = Vec::new();
    for (index, id) in resource_names.iter().enumerate() {
        let value = &raw.resources[id];
        let volume =
            Volume::new(value.volume).map_err(|e| chemistry_value("resource volume", e))?;
        let diffusion = DiffusionRate::new(value.diffusion_rate)
            .map_err(|e| chemistry_value("resource diffusion_rate", e))?;
        let energy = EnergyValue::new(value.energy_value)
            .map_err(|e| chemistry_value("resource energy_value", e))?;
        let decay = DecayRate::new(value.decay_rate)
            .map_err(|e| chemistry_value("resource decay_rate", e))?;
        let reactivity_profile = match value.reactivity_profile.as_str() {
            "stable" => ("stable", ReactivityProfile::Stable),
            "reactive" => ("reactive", ReactivityProfile::Reactive),
            other => {
                return Err(ParseError::ValidationError(format!(
                    "Unknown reactivity profile: {other}"
                )));
            }
        };
        let permeability = match value.permeability.as_str() {
            "blocked" => ("blocked", PermeabilityConstraint::Blocked),
            "passive" => ("passive", PermeabilityConstraint::Passive),
            "active_required" => ("active_required", PermeabilityConstraint::ActiveRequired),
            other => {
                return Err(ParseError::ValidationError(format!(
                    "Unknown permeability: {other}"
                )));
            }
        };
        let tags = value
            .tags
            .iter()
            .map(|tag| match tag.as_str() {
                "energy_source" => Ok(crate::core::resource_types::ResourceTag::EnergySource),
                "dissolved" => Ok(crate::core::resource_types::ResourceTag::Dissolved),
                "structural_precursor" => {
                    Ok(crate::core::resource_types::ResourceTag::StructuralPrecursor)
                }
                "waste" => Ok(crate::core::resource_types::ResourceTag::Waste),
                other => Err(ParseError::ValidationError(format!(
                    "Unknown resource tag: {other}"
                ))),
            })
            .collect::<Result<ResourceTags, _>>()?;
        resource_types.push(crate::core::resource_types::ResourceType::new(
            ResourceTypeId::from_raw(index as u32),
            ResourceProperties::new(
                volume,
                diffusion,
                energy,
                decay,
                reactivity_profile.1,
                permeability.1,
                tags,
            ),
        ));
        resource_values.push(ChemistryResourceConfig {
            id: id.clone(),
            volume: value.volume,
            diffusion_rate: value.diffusion_rate,
            energy_value: value.energy_value,
            decay_rate: value.decay_rate,
            reactivity_profile: reactivity_profile.0.to_string(),
            permeability: permeability.0.to_string(),
            tags: value.tags.clone(),
        });
    }
    ResourceRegistry::new(resource_types)
        .map_err(|e| ParseError::ValidationError(format!("Invalid resource registry: {e:?}")))?;

    let mut material_values = Vec::new();
    let mut material_types = Vec::new();
    for (index, id) in material_names.iter().enumerate() {
        let value = &raw.materials[id];
        let volume =
            Volume::new(value.volume).map_err(|e| chemistry_value("material volume", e))?;
        let stability =
            Strength::new(value.stability).map_err(|e| chemistry_value("material stability", e))?;
        let strength =
            Strength::new(value.strength).map_err(|e| chemistry_value("material strength", e))?;
        let permeability = Strength::new(value.permeability)
            .map_err(|e| chemistry_value("material permeability", e))?;
        let energy_capacity = EnergyCapacity::new(value.energy_capacity)
            .map_err(|e| chemistry_value("material energy_capacity", e))?;
        let decay = DecayRate::new(value.decay_rate)
            .map_err(|e| chemistry_value("material decay_rate", e))?;
        let repair_amount = non_negative(value.repair_amount, "material repair_amount")?;
        material_types.push(crate::core::material_types::MaterialType::new(
            MaterialTypeId::from_raw(index as u32),
            MaterialProperties::new(
                volume,
                stability,
                strength,
                permeability,
                energy_capacity,
                decay,
                RepairRequirements::new(volume),
                ReactionProfile::Passive,
                SignalProperties::new(
                    Strength::new(0.0).unwrap(),
                    SignalAmount::new(0.0).unwrap(),
                    Strength::new(0.0).unwrap(),
                ),
            ),
        ));
        material_values.push(ChemistryMaterialConfig {
            id: id.clone(),
            volume: value.volume,
            stability: value.stability,
            strength: value.strength,
            permeability: value.permeability,
            energy_capacity: value.energy_capacity,
            decay_rate: value.decay_rate,
            repair_resource: value.repair_resource.clone(),
            repair_amount,
        });
    }
    MaterialRegistry::new(material_types)
        .map_err(|e| ParseError::ValidationError(format!("Invalid material registry: {e:?}")))?;

    let resource_ids: std::collections::HashSet<_> = resource_names.iter().collect();
    let material_ids: std::collections::HashSet<_> = material_names.iter().collect();
    for material in &material_values {
        if !resource_ids.contains(&material.repair_resource) {
            return Err(ParseError::ValidationError(format!(
                "Unknown repair resource: {}",
                material.repair_resource
            )));
        }
    }
    let mut reaction_names: Vec<_> = raw.reactions.keys().cloned().collect();
    reaction_names.sort();
    let mut reactions = Vec::new();
    for id in reaction_names {
        let value = &raw.reactions[&id];
        if !matches!(value.mode.as_str(), "passive" | "controlled") {
            return Err(ParseError::ValidationError(format!(
                "Unknown reaction mode: {}",
                value.mode
            )));
        }
        let process_id = match value.mode.as_str() {
            "passive" => {
                if value.process_id.is_some() {
                    return Err(ParseError::ValidationError(
                        "Passive reaction cannot declare process_id".to_string(),
                    ));
                }
                None
            }
            "controlled" => match value.process_id.as_deref() {
                Some("energy_conversion") => Some("energy_conversion".to_string()),
                Some(other) => {
                    return Err(ParseError::ValidationError(format!(
                        "Unsupported controlled reaction process_id: {other}"
                    )));
                }
                None => {
                    return Err(ParseError::ValidationError(
                        "Controlled reaction requires process_id = energy_conversion".to_string(),
                    ));
                }
            },
            _ => unreachable!("reaction mode is validated above"),
        };
        let rate = non_negative(value.rate, "reaction rate")?;
        if !value.probability.is_finite() || !(0.0..=1.0).contains(&value.probability) {
            return Err(ParseError::ValidationError(
                "Reaction probability must be in 0..=1".to_string(),
            ));
        }
        if value.accounting_destination.is_empty()
            || !resource_ids.contains(&value.accounting_destination)
        {
            return Err(ParseError::ValidationError(
                "Reaction accounting destination is unknown or missing".to_string(),
            ));
        }
        let inputs = normalize_amounts(&value.inputs, &resource_ids, "reaction input")?;
        let outputs = normalize_amounts(&value.outputs, &resource_ids, "reaction output")?;
        let configured_sink_amount = non_negative(
            value.configured_sink_amount,
            "reaction configured_sink_amount",
        )?;
        let energy_output = non_negative(value.energy_output, "reaction energy_output")?;
        let heat_output = non_negative(value.heat_output, "reaction heat_output")?;
        let required_materials = normalize_amounts(
            &value.required_materials,
            &material_ids,
            "reaction catalyst",
        )?;
        if inputs.is_empty() && !outputs.is_empty() {
            return Err(ParseError::ValidationError(
                "Reaction products require inputs".to_string(),
            ));
        }
        let input_total: f32 = inputs.iter().map(|(_, amount)| amount).sum();
        let destination_total: f32 =
            outputs.iter().map(|(_, amount)| amount).sum::<f32>() + configured_sink_amount;
        if (input_total - destination_total).abs() > 1e-5 {
            return Err(ParseError::ValidationError(
                "Reaction inputs must be fully accounted by outputs and configured sink"
                    .to_string(),
            ));
        }
        reactions.push(ChemistryReactionConfig {
            id,
            mode: value.mode.clone(),
            process_id,
            inputs,
            required_materials,
            outputs,
            configured_sink_amount,
            energy_output,
            heat_output,
            rate,
            probability: value.probability,
            accounting_destination: value.accounting_destination.clone(),
        });
    }

    let heat = raw
        .heat
        .map(|value| ChemistryHeatConfig {
            capacity: value.capacity,
            dissipation_rate: value.dissipation_rate,
            warning_threshold: value.warning_threshold,
            death_threshold: value.death_threshold,
        })
        .unwrap_or(ChemistryHeatConfig {
            capacity: 0.0,
            dissipation_rate: 0.0,
            warning_threshold: 0.0,
            death_threshold: 0.0,
        });
    for value in [
        heat.capacity,
        heat.dissipation_rate,
        heat.warning_threshold,
        heat.death_threshold,
    ] {
        non_negative(value, "heat value")?;
    }
    if heat.warning_threshold > heat.death_threshold {
        return Err(ParseError::ValidationError(
            "Heat warning threshold exceeds death threshold".to_string(),
        ));
    }
    let boundary = raw
        .boundary
        .map(|value| {
            if !matches!(
                value.default_permeability.as_str(),
                "blocked" | "passive" | "active_required"
            ) {
                return Err(ParseError::ValidationError(
                    "Unknown boundary permeability".to_string(),
                ));
            }
            Ok(ChemistryBoundaryConfig {
                default_permeability: value.default_permeability,
                retention_rate: value.retention_rate,
            })
        })
        .transpose()?
        .unwrap_or(ChemistryBoundaryConfig {
            default_permeability: "blocked".to_string(),
            retention_rate: 1.0,
        });
    if !boundary.retention_rate.is_finite() || !(0.0..=1.0).contains(&boundary.retention_rate) {
        return Err(ParseError::ValidationError(
            "Boundary retention_rate must be in 0..=1".to_string(),
        ));
    }
    let repair = raw
        .repair
        .map(|value| {
            Ok(ChemistryRepairConfig {
                enabled: value.enabled,
                energy_cost: non_negative(value.energy_cost, "repair energy_cost")?,
                max_amount_per_tick: non_negative(
                    value.max_amount_per_tick,
                    "repair max_amount_per_tick",
                )?,
            })
        })
        .transpose()?
        .unwrap_or(ChemistryRepairConfig {
            enabled: false,
            energy_cost: 0.0,
            max_amount_per_tick: 0.0,
        });
    Ok(ChemistryConfig {
        resources: resource_values,
        materials: material_values,
        reactions,
        heat,
        boundary,
        repair,
    })
}

fn normalize_amounts(
    values: &HashMap<String, f32>,
    known: &std::collections::HashSet<&String>,
    label: &str,
) -> Result<Vec<(String, f32)>, ParseError> {
    let mut ids: Vec<_> = values.keys().cloned().collect();
    ids.sort();
    ids.into_iter()
        .map(|id| {
            if !known.contains(&id) {
                return Err(ParseError::ValidationError(format!(
                    "Unknown {label}: {id}"
                )));
            }
            Ok((id.clone(), non_negative(values[&id], label)?))
        })
        .collect()
}

fn non_negative(value: f32, label: &str) -> Result<f32, ParseError> {
    if !value.is_finite() || value < 0.0 {
        Err(ParseError::ValidationError(format!(
            "Invalid {label}: {value}"
        )))
    } else {
        Ok(value)
    }
}

fn typed_resource_inventory(
    raw: &RawCell,
    declared_resource_ids: &[String],
) -> Result<Vec<(ResourceTypeId, ResourceAmount)>, ParseError> {
    let mut inventory = Vec::new();
    for (id, amount) in &raw.initial_resources {
        let type_index = declared_resource_ids
            .iter()
            .position(|known| known == id)
            .ok_or_else(|| {
                ParseError::ValidationError(format!("Unknown initial cell resource: {id}"))
            })?;
        inventory.push((
            ResourceTypeId::from_raw(type_index as u32),
            ResourceAmount::new(*amount).map_err(|error| {
                ParseError::ValidationError(format!("Invalid initial cell resource: {error:?}"))
            })?,
        ));
    }
    inventory.sort_by_key(|(id, _)| *id);
    Ok(inventory)
}

fn chemistry_value(label: &str, error: crate::core::units::AmountError) -> ParseError {
    ParseError::ValidationError(format!("Invalid {label}: {error:?}"))
}
