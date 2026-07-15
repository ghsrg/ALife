use crate::core::genome::{GenomeTemplate, GenomeTemplateId};
use crate::core::ids::ResourceTypeId;
use crate::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ChemistryResourceConfig {
    pub id: String,
    pub volume: f32,
    pub diffusion_rate: f32,
    pub energy_value: f32,
    pub decay_rate: f32,
    pub reactivity_profile: String,
    pub permeability: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChemistryMaterialConfig {
    pub id: String,
    pub volume: f32,
    pub stability: f32,
    pub strength: f32,
    pub permeability: f32,
    pub energy_capacity: f32,
    pub decay_rate: f32,
    pub repair_resource: String,
    pub repair_amount: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChemistryReactionConfig {
    pub id: String,
    pub mode: String,
    pub process_id: Option<String>,
    pub inputs: Vec<(String, f32)>,
    pub required_materials: Vec<(String, f32)>,
    pub outputs: Vec<(String, f32)>,
    pub configured_sink_amount: f32,
    pub energy_output: f32,
    pub heat_output: f32,
    pub rate: f32,
    pub probability: f32,
    pub accounting_destination: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChemistryHeatConfig {
    pub capacity: f32,
    pub dissipation_rate: f32,
    pub warning_threshold: f32,
    pub death_threshold: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChemistryBoundaryConfig {
    pub default_permeability: String,
    pub retention_rate: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChemistryRepairConfig {
    pub enabled: bool,
    pub energy_cost: f32,
    pub max_amount_per_tick: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChemistryConfig {
    pub resources: Vec<ChemistryResourceConfig>,
    pub materials: Vec<ChemistryMaterialConfig>,
    pub reactions: Vec<ChemistryReactionConfig>,
    pub heat: ChemistryHeatConfig,
    pub boundary: ChemistryBoundaryConfig,
    pub repair: ChemistryRepairConfig,
}

impl Default for ChemistryConfig {
    fn default() -> Self {
        Self {
            resources: Vec::new(),
            materials: Vec::new(),
            reactions: Vec::new(),
            heat: ChemistryHeatConfig {
                capacity: 0.0,
                dissipation_rate: 0.0,
                warning_threshold: 0.0,
                death_threshold: 0.0,
            },
            boundary: ChemistryBoundaryConfig {
                default_permeability: "blocked".to_string(),
                retention_rate: 1.0,
            },
            repair: ChemistryRepairConfig {
                enabled: false,
                energy_cost: 0.0,
                max_amount_per_tick: 0.0,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldConfig {
    pub tick_count: Tick,
    pub seed: Seed,
    pub size: WorldSize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpaceConfig {
    pub spatial_grid_size: f32,
    pub physics_solver_iterations: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceConfig {
    pub initial_distribution: Vec<ResourceAmount>,
    pub optional_decay_rate: f32,
}

impl ResourceConfig {
    pub fn new(
        initial_distribution: Vec<ResourceAmount>,
        optional_decay_rate: f32,
    ) -> Result<Self, ConfigError> {
        if initial_distribution.is_empty() {
            return Err(ConfigError::EmptyResourceDistribution);
        }
        if !optional_decay_rate.is_finite() || !(0.0..=1.0).contains(&optional_decay_rate) {
            return Err(ConfigError::InvalidDecayRate);
        }

        Ok(Self {
            initial_distribution,
            optional_decay_rate,
        })
    }

    pub fn layer_count(&self) -> usize {
        self.initial_distribution.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CellInitialConfig {
    pub position: Position,
    pub radius: Radius,
    pub initial_energy: EnergyAmount,
    pub energy_capacity: EnergyAmount,
    pub mandatory_cost_per_tick: EnergyAmount,
    pub passive_energy_income: EnergyAmount,
    pub capacity_limit: CapacityAmount,
    pub initial_resource_amount: ResourceAmount,
    pub initial_boundary_material: MaterialAmount,
    pub initial_transport_material: MaterialAmount,
    pub initial_metabolic_material: MaterialAmount,
    pub initial_storage_material: MaterialAmount,
    pub initial_synthesis_material: MaterialAmount,
    pub initial_structural_material: MaterialAmount,
    pub initial_repair_material: MaterialAmount,
    pub initial_contractile_material: MaterialAmount,
    pub initial_sensory_material: MaterialAmount,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnvironmentConfig {
    pub heat_current: HeatAmount,
    pub heat_generated_per_tick: HeatAmount,
    pub heat_dissipation_rate: HeatAmount,
    pub heat_warning_threshold: HeatAmount,
    pub heat_death_threshold: HeatAmount,
    pub waste_current: WasteAmount,
    pub waste_generated_per_tick: WasteAmount,
    pub waste_sink_rate: WasteAmount,
    pub waste_warning_threshold: WasteAmount,
    pub waste_death_threshold: WasteAmount,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LifecycleConfig {
    pub stress_energy_threshold: EnergyAmount,
    pub dormancy_allowed: bool,
    pub dormant_mandatory_cost_modifier: f32,
    pub critical_capacity_overrun: CapacityAmount,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResourceInteractionConfig {
    pub enabled: bool,
    pub uptake_layer_index: usize,
    pub max_uptake_per_tick: ResourceAmount,
    pub metabolism_resource_per_tick: ResourceAmount,
    pub energy_per_resource: f32,
    pub heat_per_resource: f32,
    pub waste_per_resource: f32,
}

impl ResourceInteractionConfig {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::zero(),
            metabolism_resource_per_tick: ResourceAmount::zero(),
            energy_per_resource: 0.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        }
    }

    pub fn validate(self, resources: &ResourceConfig) -> Result<(), ConfigError> {
        if !self.enabled {
            return Ok(());
        }
        if self.uptake_layer_index >= resources.layer_count() {
            return Err(ConfigError::InvalidResourceInteractionLayer);
        }
        for value in [
            self.energy_per_resource,
            self.heat_per_resource,
            self.waste_per_resource,
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(ConfigError::InvalidResourceInteractionRate);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocalInteractionConfig {
    pub enabled: bool,
    pub contact_exchange_rate: f32,
    pub max_exchange_per_pair: ResourceAmount,
    pub min_boundary_capability: f32,
    pub min_transport_capability: f32,
    pub contact_stimulus_per_overlap: f32,
    pub stimulus_decay_per_tick: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JointConfig {
    pub enabled: bool,
    pub creation_distance_margin: f32,
    pub creation_material_cost: MaterialAmount,
    pub creation_resource_cost: ResourceAmount,
    pub creation_energy_cost: EnergyAmount,
    pub upkeep_material_decay_per_tick: f32,
    pub break_damage_threshold: f32,
    pub max_joints_per_cell: u32,
    pub mechanical_strength: f32,
    pub resource_transfer_rate: f32,
    pub max_resource_transfer_per_tick: ResourceAmount,
    pub signal_conductivity: f32,
    pub signal_decay: f32,
    pub heat_conductivity: f32,
}

impl Default for JointConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            creation_distance_margin: 0.25,
            creation_material_cost: MaterialAmount::new_unchecked(1.0),
            creation_resource_cost: ResourceAmount::zero(),
            creation_energy_cost: EnergyAmount::zero(),
            upkeep_material_decay_per_tick: 0.0,
            break_damage_threshold: 1.0,
            max_joints_per_cell: 4,
            mechanical_strength: 0.25,
            resource_transfer_rate: 0.0,
            max_resource_transfer_per_tick: ResourceAmount::zero(),
            signal_conductivity: 0.0,
            signal_decay: 0.0,
            heat_conductivity: 0.0,
        }
    }
}

impl Default for LocalInteractionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            contact_exchange_rate: 0.0,
            max_exchange_per_pair: ResourceAmount::zero(),
            min_boundary_capability: 0.0,
            min_transport_capability: 0.0,
            contact_stimulus_per_overlap: 0.0,
            stimulus_decay_per_tick: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GrowthConfig {
    pub growth_cost_resource: ResourceAmount,
    pub growth_cost_energy: EnergyAmount,
    pub growth_target_radius: Radius,
    pub max_division_pressure: f32,
}

impl Default for GrowthConfig {
    fn default() -> Self {
        Self {
            growth_cost_resource: ResourceAmount::new(2.0).unwrap(),
            growth_cost_energy: EnergyAmount::new(1.0).unwrap(),
            growth_target_radius: Radius::new(2.0).unwrap(),
            max_division_pressure: 0.5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SynthesisConfig {
    pub cost_resource: ResourceAmount,
    pub cost_energy: EnergyAmount,
}

impl Default for SynthesisConfig {
    fn default() -> Self {
        Self {
            cost_resource: ResourceAmount::new(1.0).unwrap(),
            cost_energy: EnergyAmount::new(5.0).unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContractilityConfig {
    pub energy_cost: EnergyAmount,
    pub force_factor: f32,
}

impl Default for ContractilityConfig {
    fn default() -> Self {
        Self {
            energy_cost: EnergyAmount::new(1.0).unwrap(),
            force_factor: 0.1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DivisionConfig {
    pub enabled: bool,
    pub energy_cost: EnergyAmount,
    pub split_ratio: f32,
    pub daughter_spacing: f32,
    pub min_daughter_radius: Radius,
    pub partition_loss_fraction: f32,
}

impl Default for DivisionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            energy_cost: EnergyAmount::zero(),
            split_ratio: 0.5,
            daughter_spacing: 0.25,
            min_daughter_radius: Radius::new(0.5).unwrap(),
            partition_loss_fraction: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecompositionConfig {
    pub enabled: bool,
    pub resource_layer_index: usize,
    pub resources_per_tick: ResourceAmount,
    pub materials_per_tick: MaterialAmount,
    pub remove_when_empty: bool,
}

impl Default for DecompositionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            resource_layer_index: 0,
            resources_per_tick: ResourceAmount::zero(),
            materials_per_tick: MaterialAmount::zero(),
            remove_when_empty: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialEffectConfig {
    pub transport_uptake_per_unit: f32,
    pub metabolic_conversion_per_unit: f32,
    pub storage_capacity_per_unit: f32,
    pub structural_growth_per_unit: f32,
    pub contractile_force_per_unit: f32,
    pub sensory_input_per_unit: f32,
    pub boundary_retention_per_unit: f32,
    pub repair_stress_buffer_per_unit: f32,
}

impl Default for MaterialEffectConfig {
    fn default() -> Self {
        Self {
            transport_uptake_per_unit: 1.0,
            metabolic_conversion_per_unit: 1.0,
            storage_capacity_per_unit: 0.0,
            structural_growth_per_unit: 1.0,
            contractile_force_per_unit: 1.0,
            sensory_input_per_unit: 1.0,
            boundary_retention_per_unit: 1.0,
            repair_stress_buffer_per_unit: 1.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeConfig {
    pub simulation_time: SimulationTimeConfig,
    pub world: WorldConfig,
    pub space: SpaceConfig,
    pub resources: ResourceConfig,
    pub resource_interaction: ResourceInteractionConfig,
    pub cell: CellInitialConfig,
    pub environment: EnvironmentConfig,
    pub lifecycle: LifecycleConfig,
    pub growth: GrowthConfig,
    pub growth_enabled: bool,
    pub initial_cells: Vec<CellInitialConfig>,
    pub initial_typed_resources: Vec<Vec<(ResourceTypeId, ResourceAmount)>>,
    pub genome_templates: Vec<GenomeTemplate>,
    pub initial_cell_genome_templates: Vec<Option<GenomeTemplateId>>,
    pub synthesis: SynthesisConfig,
    pub contractility: ContractilityConfig,
    pub division: DivisionConfig,
    pub decomposition: DecompositionConfig,
    pub material_effects: MaterialEffectConfig,
    pub local_interaction: LocalInteractionConfig,
    pub joints: JointConfig,
    pub chemistry: ChemistryConfig,
    pub scheduler: SchedulerConfig,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SimulationTimeConfig {
    pub tick_duration_ms: u32,
}

impl Default for SimulationTimeConfig {
    fn default() -> Self {
        Self {
            tick_duration_ms: 100,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchedulerCellConfig {
    pub genome_runtime_base_ticks: u64,
    pub genome_runtime_ticks_per_layer: u64,
    pub signal_emit_ticks: u64,
    pub controlled_reaction_ticks: u64,
    pub simple_synthesis_ticks: u64,
    pub basic_repair_ticks: u64,
    pub internal_rebalance_ticks: u64,
}

impl Default for SchedulerCellConfig {
    fn default() -> Self {
        Self {
            genome_runtime_base_ticks: 1,
            genome_runtime_ticks_per_layer: 1,
            signal_emit_ticks: 1,
            controlled_reaction_ticks: 1,
            simple_synthesis_ticks: 1,
            basic_repair_ticks: 1,
            internal_rebalance_ticks: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchedulerWorldConfig {
    pub resource_diffusion_ticks: u64,
    pub resource_decay_ticks: u64,
    pub passive_reactions_ticks: u64,
    pub background_material_degradation_ticks: u64,
    pub environment_heat_diffusion_ticks: u64,
    pub field_update_ticks: u64,
}

impl Default for SchedulerWorldConfig {
    fn default() -> Self {
        Self {
            resource_diffusion_ticks: 1,
            resource_decay_ticks: 1,
            passive_reactions_ticks: 1,
            background_material_degradation_ticks: 1,
            environment_heat_diffusion_ticks: 1,
            field_update_ticks: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchedulerObserverConfig {
    pub observer_metrics_ticks: u64,
    pub resource_totals_ticks: u64,
    pub graph_analysis_ticks: u64,
    pub debug_trace_ticks: u64,
}

impl Default for SchedulerObserverConfig {
    fn default() -> Self {
        Self {
            observer_metrics_ticks: 1,
            resource_totals_ticks: 1,
            graph_analysis_ticks: 1,
            debug_trace_ticks: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SchedulerConfig {
    pub cell: SchedulerCellConfig,
    pub world: SchedulerWorldConfig,
    pub observer: SchedulerObserverConfig,
}

pub fn deterministic_genome_decision_offset(
    world_seed: u64,
    cell_id_raw: u32,
    cadence: u64,
) -> u64 {
    let cadence = cadence.max(1);
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in b"genome-runtime-stagger" {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for byte in world_seed.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for byte in cell_id_raw.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash % cadence
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigError {
    InitialEnergyExceedsCapacity,
    InvalidSpatialGridSize,
    InvalidDormancyModifier,
    InvalidDecayRate,
    EmptyResourceDistribution,
    InvalidResourceInteractionLayer,
    InvalidResourceInteractionRate,
    InvalidDivisionSplit,
    InvalidDivisionLoss,
    InvalidDaughterSpacing,
    InvalidDecompositionLayer,
    InvalidLocalInteractionRate,
    InvalidJointRate,
    InvalidSchedulerCadence,
}

impl RuntimeConfig {
    pub fn new(
        world: WorldConfig,
        space: SpaceConfig,
        resources: ResourceConfig,
        resource_interaction: ResourceInteractionConfig,
        cell: CellInitialConfig,
        environment: EnvironmentConfig,
        lifecycle: LifecycleConfig,
    ) -> Result<Self, ConfigError> {
        if cell.initial_energy.raw() > cell.energy_capacity.raw() {
            return Err(ConfigError::InitialEnergyExceedsCapacity);
        }
        if !space.spatial_grid_size.is_finite() || space.spatial_grid_size <= 0.0 {
            return Err(ConfigError::InvalidSpatialGridSize);
        }
        if !lifecycle.dormant_mandatory_cost_modifier.is_finite()
            || lifecycle.dormant_mandatory_cost_modifier < 0.0
            || lifecycle.dormant_mandatory_cost_modifier > 1.0
        {
            return Err(ConfigError::InvalidDormancyModifier);
        }

        resource_interaction.validate(&resources)?;

        let initial_cells = vec![cell];

        Ok(Self {
            simulation_time: SimulationTimeConfig::default(),
            world,
            space,
            resources,
            resource_interaction,
            cell,
            environment,
            lifecycle,
            growth: GrowthConfig::default(),
            growth_enabled: false,
            initial_cells,
            initial_typed_resources: vec![Vec::new()],
            genome_templates: Vec::new(),
            initial_cell_genome_templates: vec![None],
            synthesis: SynthesisConfig::default(),
            contractility: ContractilityConfig::default(),
            division: DivisionConfig::default(),
            decomposition: DecompositionConfig::default(),
            material_effects: MaterialEffectConfig::default(),
            local_interaction: LocalInteractionConfig::default(),
            joints: JointConfig::default(),
            chemistry: ChemistryConfig::default(),
            scheduler: SchedulerConfig::default(),
        })
    }

    pub fn validate_scheduler_options(&self) -> Result<(), ConfigError> {
        let values = [
            self.scheduler.cell.genome_runtime_base_ticks,
            self.scheduler.cell.genome_runtime_ticks_per_layer,
            self.scheduler.cell.signal_emit_ticks,
            self.scheduler.cell.controlled_reaction_ticks,
            self.scheduler.cell.simple_synthesis_ticks,
            self.scheduler.cell.basic_repair_ticks,
            self.scheduler.cell.internal_rebalance_ticks,
            self.scheduler.world.resource_diffusion_ticks,
            self.scheduler.world.resource_decay_ticks,
            self.scheduler.world.passive_reactions_ticks,
            self.scheduler.world.background_material_degradation_ticks,
            self.scheduler.world.environment_heat_diffusion_ticks,
            self.scheduler.world.field_update_ticks,
            self.scheduler.observer.observer_metrics_ticks,
            self.scheduler.observer.resource_totals_ticks,
            self.scheduler.observer.graph_analysis_ticks,
            self.scheduler.observer.debug_trace_ticks,
        ];
        if values.iter().any(|value| *value == 0) {
            return Err(ConfigError::InvalidSchedulerCadence);
        }
        Ok(())
    }

    pub fn effective_genome_runtime_cadence_ticks(&self, template_id: &str) -> Option<u64> {
        let template = self
            .genome_templates
            .iter()
            .find(|candidate| candidate.id().as_str() == template_id)?;
        Some(self.effective_genome_runtime_cadence_ticks_for_template(template))
    }

    pub fn effective_genome_runtime_cadence_ticks_for_template(
        &self,
        template: &GenomeTemplate,
    ) -> u64 {
        let base = template.runtime_interval_ticks().max(1);
        let depth = template.regulatory_depth().max(1);
        base + (depth - 1) * self.scheduler.cell.genome_runtime_ticks_per_layer.max(1)
    }

    pub fn effective_genome_runtime_cadence_ticks_for_genome(
        &self,
        genome: Option<&crate::core::genome::GenomeState>,
    ) -> u64 {
        let Some(genome) = genome else {
            return self.scheduler.cell.genome_runtime_base_ticks.max(1);
        };
        self.effective_genome_runtime_cadence_ticks(genome.template_id.as_str())
            .unwrap_or(self.scheduler.cell.genome_runtime_base_ticks.max(1))
    }

    pub fn initial_genome_runtime_offsets(&self, count: usize, cadence: u64) -> Vec<u64> {
        (0..count)
            .map(|index| {
                deterministic_genome_decision_offset(
                    self.world.seed.raw(),
                    (index as u32).saturating_add(1),
                    cadence,
                )
            })
            .collect()
    }

    pub fn validate_phase2d_options(&self) -> Result<(), ConfigError> {
        if !(0.35..=0.65).contains(&self.division.split_ratio) {
            return Err(ConfigError::InvalidDivisionSplit);
        }
        if !(0.0..=0.25).contains(&self.division.partition_loss_fraction) {
            return Err(ConfigError::InvalidDivisionLoss);
        }
        if !self.division.daughter_spacing.is_finite() || self.division.daughter_spacing < 0.0 {
            return Err(ConfigError::InvalidDaughterSpacing);
        }
        if self.decomposition.resource_layer_index >= self.resources.layer_count() {
            return Err(ConfigError::InvalidDecompositionLayer);
        }
        Ok(())
    }

    pub fn validate_phase2f_options(&self) -> Result<(), ConfigError> {
        let cfg = self.local_interaction;
        for value in [
            cfg.contact_exchange_rate,
            cfg.min_boundary_capability,
            cfg.min_transport_capability,
            cfg.contact_stimulus_per_overlap,
            cfg.stimulus_decay_per_tick,
            cfg.max_exchange_per_pair.raw(),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(ConfigError::InvalidLocalInteractionRate);
            }
        }
        Ok(())
    }

    pub fn validate_phase2h_options(&self) -> Result<(), ConfigError> {
        let cfg = self.joints;
        for value in [
            cfg.creation_distance_margin,
            cfg.creation_material_cost.raw(),
            cfg.creation_resource_cost.raw(),
            cfg.creation_energy_cost.raw(),
            cfg.upkeep_material_decay_per_tick,
            cfg.break_damage_threshold,
            cfg.mechanical_strength,
            cfg.resource_transfer_rate,
            cfg.max_resource_transfer_per_tick.raw(),
            cfg.signal_conductivity,
            cfg.signal_decay,
            cfg.heat_conductivity,
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(ConfigError::InvalidJointRate);
            }
        }
        Ok(())
    }

    pub fn with_cells(mut self, cells: Vec<CellInitialConfig>) -> Self {
        for cell in &cells {
            assert!(
                cell.initial_energy.raw() <= cell.energy_capacity.raw(),
                "Initial energy exceeds capacity for cell at {:?}",
                cell.position
            );
        }
        if let Some(first) = cells.first() {
            self.cell = *first;
        }
        if self.initial_cell_genome_templates.len() != cells.len() {
            self.initial_cell_genome_templates = vec![None; cells.len()];
        }
        self.initial_cells = cells;
        self
    }

    pub fn config_hash(&self) -> u64 {
        let mut hash = 0xcbf29ce484222325_u64;
        for value in [
            self.world.tick_count.raw(),
            self.world.seed.raw(),
            self.world.size.width().to_bits() as u64,
            self.world.size.height().to_bits() as u64,
            self.simulation_time.tick_duration_ms as u64,
            self.scheduler.cell.genome_runtime_base_ticks,
            self.scheduler.cell.genome_runtime_ticks_per_layer,
            self.scheduler.cell.signal_emit_ticks,
            self.scheduler.cell.controlled_reaction_ticks,
            self.scheduler.cell.simple_synthesis_ticks,
            self.scheduler.cell.basic_repair_ticks,
            self.scheduler.cell.internal_rebalance_ticks,
            self.scheduler.world.resource_diffusion_ticks,
            self.scheduler.world.resource_decay_ticks,
            self.scheduler.world.passive_reactions_ticks,
            self.scheduler.world.background_material_degradation_ticks,
            self.scheduler.world.environment_heat_diffusion_ticks,
            self.scheduler.world.field_update_ticks,
            self.scheduler.observer.observer_metrics_ticks,
            self.scheduler.observer.resource_totals_ticks,
            self.scheduler.observer.graph_analysis_ticks,
            self.scheduler.observer.debug_trace_ticks,
            self.space.spatial_grid_size.to_bits() as u64,
            self.space.physics_solver_iterations as u64,
            self.cell.position.x().to_bits() as u64,
            self.cell.position.y().to_bits() as u64,
            self.cell.radius.raw().to_bits() as u64,
            self.cell.initial_energy.raw().to_bits() as u64,
            self.cell.energy_capacity.raw().to_bits() as u64,
            self.cell.mandatory_cost_per_tick.raw().to_bits() as u64,
            self.cell.passive_energy_income.raw().to_bits() as u64,
            self.cell.capacity_limit.raw().to_bits() as u64,
            self.cell.initial_boundary_material.raw().to_bits() as u64,
            self.cell.initial_transport_material.raw().to_bits() as u64,
            self.cell.initial_metabolic_material.raw().to_bits() as u64,
            self.cell.initial_storage_material.raw().to_bits() as u64,
            self.cell.initial_synthesis_material.raw().to_bits() as u64,
            self.cell.initial_structural_material.raw().to_bits() as u64,
            self.cell.initial_repair_material.raw().to_bits() as u64,
            self.cell.initial_contractile_material.raw().to_bits() as u64,
            self.cell.initial_sensory_material.raw().to_bits() as u64,
            self.growth.growth_cost_resource.raw().to_bits() as u64,
            self.growth.growth_cost_energy.raw().to_bits() as u64,
            self.growth.growth_target_radius.raw().to_bits() as u64,
            self.growth.max_division_pressure.to_bits() as u64,
            self.synthesis.cost_resource.raw().to_bits() as u64,
            self.synthesis.cost_energy.raw().to_bits() as u64,
            self.contractility.energy_cost.raw().to_bits() as u64,
            self.contractility.force_factor.to_bits() as u64,
            self.lifecycle.stress_energy_threshold.raw().to_bits() as u64,
            self.lifecycle.dormancy_allowed as u64,
            self.lifecycle.dormant_mandatory_cost_modifier.to_bits() as u64,
            self.lifecycle.critical_capacity_overrun.raw().to_bits() as u64,
            self.environment.heat_current.raw().to_bits() as u64,
            self.environment.heat_generated_per_tick.raw().to_bits() as u64,
            self.environment.heat_dissipation_rate.raw().to_bits() as u64,
            self.environment.heat_warning_threshold.raw().to_bits() as u64,
            self.environment.heat_death_threshold.raw().to_bits() as u64,
            self.environment.waste_current.raw().to_bits() as u64,
            self.environment.waste_generated_per_tick.raw().to_bits() as u64,
            self.environment.waste_sink_rate.raw().to_bits() as u64,
            self.environment.waste_warning_threshold.raw().to_bits() as u64,
            self.environment.waste_death_threshold.raw().to_bits() as u64,
            self.growth_enabled as u64,
            self.division.enabled as u64,
            self.division.energy_cost.raw().to_bits() as u64,
            self.division.split_ratio.to_bits() as u64,
            self.division.daughter_spacing.to_bits() as u64,
            self.division.min_daughter_radius.raw().to_bits() as u64,
            self.division.partition_loss_fraction.to_bits() as u64,
            self.decomposition.enabled as u64,
            self.decomposition.resource_layer_index as u64,
            self.decomposition.resources_per_tick.raw().to_bits() as u64,
            self.decomposition.materials_per_tick.raw().to_bits() as u64,
            self.decomposition.remove_when_empty as u64,
            self.material_effects.transport_uptake_per_unit.to_bits() as u64,
            self.material_effects
                .metabolic_conversion_per_unit
                .to_bits() as u64,
            self.material_effects.storage_capacity_per_unit.to_bits() as u64,
            self.material_effects.structural_growth_per_unit.to_bits() as u64,
            self.material_effects.contractile_force_per_unit.to_bits() as u64,
            self.material_effects.sensory_input_per_unit.to_bits() as u64,
            self.material_effects.boundary_retention_per_unit.to_bits() as u64,
            self.material_effects
                .repair_stress_buffer_per_unit
                .to_bits() as u64,
            self.local_interaction.enabled as u64,
            self.local_interaction.contact_exchange_rate.to_bits() as u64,
            self.local_interaction.max_exchange_per_pair.raw().to_bits() as u64,
            self.local_interaction.min_boundary_capability.to_bits() as u64,
            self.local_interaction.min_transport_capability.to_bits() as u64,
            self.local_interaction
                .contact_stimulus_per_overlap
                .to_bits() as u64,
            self.local_interaction.stimulus_decay_per_tick.to_bits() as u64,
            self.joints.enabled as u64,
            self.joints.creation_distance_margin.to_bits() as u64,
            self.joints.creation_material_cost.raw().to_bits() as u64,
            self.joints.creation_resource_cost.raw().to_bits() as u64,
            self.joints.creation_energy_cost.raw().to_bits() as u64,
            self.joints.upkeep_material_decay_per_tick.to_bits() as u64,
            self.joints.break_damage_threshold.to_bits() as u64,
            self.joints.max_joints_per_cell as u64,
            self.joints.mechanical_strength.to_bits() as u64,
            self.joints.resource_transfer_rate.to_bits() as u64,
            self.joints.max_resource_transfer_per_tick.raw().to_bits() as u64,
            self.joints.signal_conductivity.to_bits() as u64,
            self.joints.signal_decay.to_bits() as u64,
            self.joints.heat_conductivity.to_bits() as u64,
        ] {
            hash ^= value;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        for amount in &self.resources.initial_distribution {
            hash ^= amount.raw().to_bits() as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= self.resources.optional_decay_rate.to_bits() as u64;
        hash = hash.wrapping_mul(0x100000001b3);

        fn add(hash: &mut u64, value: u64) {
            *hash ^= value;
            *hash = hash.wrapping_mul(0x100000001b3);
        }
        fn add_text(hash: &mut u64, value: &str) {
            for byte in value.as_bytes() {
                add(hash, *byte as u64);
            }
            add(hash, 0);
        }
        for resource in &self.chemistry.resources {
            add_text(&mut hash, &resource.id);
            for value in [
                resource.volume,
                resource.diffusion_rate,
                resource.energy_value,
                resource.decay_rate,
            ] {
                add(&mut hash, value.to_bits() as u64);
            }
            add_text(&mut hash, &resource.reactivity_profile);
            add_text(&mut hash, &resource.permeability);
            for tag in &resource.tags {
                add_text(&mut hash, tag);
            }
        }
        for material in &self.chemistry.materials {
            add_text(&mut hash, &material.id);
            for value in [
                material.volume,
                material.stability,
                material.strength,
                material.permeability,
                material.energy_capacity,
                material.decay_rate,
                material.repair_amount,
            ] {
                add(&mut hash, value.to_bits() as u64);
            }
            add_text(&mut hash, &material.repair_resource);
        }
        for reaction in &self.chemistry.reactions {
            add_text(&mut hash, &reaction.id);
            add_text(&mut hash, &reaction.mode);
            add_text(&mut hash, reaction.process_id.as_deref().unwrap_or(""));
            for (id, amount) in reaction
                .inputs
                .iter()
                .chain(&reaction.required_materials)
                .chain(&reaction.outputs)
            {
                add_text(&mut hash, id);
                add(&mut hash, amount.to_bits() as u64);
            }
            for value in [
                reaction.configured_sink_amount,
                reaction.energy_output,
                reaction.heat_output,
                reaction.rate,
                reaction.probability,
            ] {
                add(&mut hash, value.to_bits() as u64);
            }
            add_text(&mut hash, &reaction.accounting_destination);
        }
        for value in [
            self.chemistry.heat.capacity,
            self.chemistry.heat.dissipation_rate,
            self.chemistry.heat.warning_threshold,
            self.chemistry.heat.death_threshold,
            self.chemistry.boundary.retention_rate,
            self.chemistry.repair.energy_cost,
            self.chemistry.repair.max_amount_per_tick,
        ] {
            add(&mut hash, value.to_bits() as u64);
        }
        add_text(&mut hash, &self.chemistry.boundary.default_permeability);
        add(&mut hash, self.chemistry.repair.enabled as u64);

        for value in [
            self.resource_interaction.enabled as u64,
            self.resource_interaction.uptake_layer_index as u64,
            self.resource_interaction
                .max_uptake_per_tick
                .raw()
                .to_bits() as u64,
            self.resource_interaction
                .metabolism_resource_per_tick
                .raw()
                .to_bits() as u64,
            self.resource_interaction.energy_per_resource.to_bits() as u64,
            self.resource_interaction.heat_per_resource.to_bits() as u64,
            self.resource_interaction.waste_per_resource.to_bits() as u64,
        ] {
            hash ^= value;
            hash = hash.wrapping_mul(0x100000001b3);
        }

        for cell in &self.initial_cells {
            for value in [
                cell.position.x().to_bits() as u64,
                cell.position.y().to_bits() as u64,
                cell.radius.raw().to_bits() as u64,
                cell.initial_energy.raw().to_bits() as u64,
                cell.energy_capacity.raw().to_bits() as u64,
                cell.mandatory_cost_per_tick.raw().to_bits() as u64,
                cell.passive_energy_income.raw().to_bits() as u64,
                cell.capacity_limit.raw().to_bits() as u64,
                cell.initial_boundary_material.raw().to_bits() as u64,
                cell.initial_transport_material.raw().to_bits() as u64,
                cell.initial_metabolic_material.raw().to_bits() as u64,
                cell.initial_storage_material.raw().to_bits() as u64,
                cell.initial_synthesis_material.raw().to_bits() as u64,
                cell.initial_structural_material.raw().to_bits() as u64,
                cell.initial_repair_material.raw().to_bits() as u64,
                cell.initial_contractile_material.raw().to_bits() as u64,
                cell.initial_sensory_material.raw().to_bits() as u64,
            ] {
                hash ^= value;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
        for inventory in &self.initial_typed_resources {
            for (id, amount) in inventory {
                add(&mut hash, id.raw() as u64);
                add(&mut hash, amount.raw().to_bits() as u64);
            }
        }
        for template in &self.genome_templates {
            add_text(&mut hash, template.id().as_str());
            add(&mut hash, template.variation_amplitude().to_bits() as u64);
            add(&mut hash, template.runtime_interval_ticks());
            add(&mut hash, template.regulatory_depth());
            add_text(&mut hash, &template.carrier().material_id);
            add(&mut hash, template.carrier().amount.to_bits() as u64);
            add(&mut hash, template.carrier().integrity.to_bits() as u64);
            for (output_id, value) in template.outputs() {
                add_text(&mut hash, output_id.as_str());
                add(&mut hash, value.raw().to_bits() as u64);
            }
        }
        for assignment in &self.initial_cell_genome_templates {
            add_text(
                &mut hash,
                assignment.as_ref().map(|id| id.as_str()).unwrap_or(""),
            );
        }

        hash
    }
}
