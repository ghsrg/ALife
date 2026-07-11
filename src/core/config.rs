use crate::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

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
    pub synthesis: SynthesisConfig,
    pub contractility: ContractilityConfig,
    pub division: DivisionConfig,
    pub decomposition: DecompositionConfig,
    pub material_effects: MaterialEffectConfig,
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
            synthesis: SynthesisConfig::default(),
            contractility: ContractilityConfig::default(),
            division: DivisionConfig::default(),
            decomposition: DecompositionConfig::default(),
            material_effects: MaterialEffectConfig::default(),
        })
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

        hash
    }
}
