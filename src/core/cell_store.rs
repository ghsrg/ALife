use crate::core::action_plan::ActionPlan;
use crate::core::genome::GenomeId;
use crate::core::ids::{CellId, ResourceTypeId};
use crate::core::materials::{MaterialComposition, MaterialSlot};
use crate::core::process::MaterialCapability;
use crate::core::resource_types::PermeabilityConstraint;
use crate::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellIndex(usize);

impl CellIndex {
    pub const fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnergyBuffer {
    current: EnergyAmount,
    capacity: EnergyAmount,
}

impl EnergyBuffer {
    pub fn new(current: EnergyAmount, capacity: EnergyAmount) -> Self {
        Self {
            current: current.clamp_max(capacity),
            capacity,
        }
    }

    pub const fn current(self) -> EnergyAmount {
        self.current
    }

    pub const fn capacity(self) -> EnergyAmount {
        self.capacity
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleState {
    Alive,
    Stressed,
    Dormant,
    Dead,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RuntimeFlags {
    pub mandatory_paid: bool,
    pub stalled: bool,
    pub over_capacity: bool,
    pub inert: bool,
    /// Set after contact sensing: radius >= growth_target_radius AND contact_pressure <= max_division_pressure.
    pub division_ready: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypedResourceInventoryError {
    AlreadyInitialized,
    DuplicateType(ResourceTypeId),
    UnknownType(ResourceTypeId),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitialCellState {
    pub position: Position,
    pub radius: Radius,
    pub energy: EnergyBuffer,
    pub resources: ResourceAmount,
    pub boundary_material: MaterialAmount,
    pub transport_material: MaterialAmount,
    pub metabolic_material: MaterialAmount,
    pub storage_material: MaterialAmount,
    pub synthesis_material: MaterialAmount,
    pub structural_material: MaterialAmount,
    pub repair_material: MaterialAmount,
    pub contractile_material: MaterialAmount,
    pub sensory_material: MaterialAmount,
    pub capacity_limit: CapacityAmount,
    pub temperature: Temperature,
}

#[derive(Clone, Debug, Default)]
pub struct CellStore {
    ids: Vec<CellId>,
    positions: Vec<Position>,
    radii: Vec<Radius>,
    energy_buffers: Vec<EnergyBuffer>,
    resources: Vec<ResourceAmount>,
    typed_resource_types: Vec<ResourceTypeId>,
    typed_resources: Vec<ResourceAmount>,
    boundary_materials: Vec<MaterialAmount>,
    transport_materials: Vec<MaterialAmount>,
    metabolic_materials: Vec<MaterialAmount>,
    storage_materials: Vec<MaterialAmount>,
    synthesis_materials: Vec<MaterialAmount>,
    structural_materials: Vec<MaterialAmount>,
    repair_materials: Vec<MaterialAmount>,
    contractile_materials: Vec<MaterialAmount>,
    sensory_materials: Vec<MaterialAmount>,
    material_damage: Vec<[f32; 9]>,
    capacity_limits: Vec<CapacityAmount>,
    temperatures: Vec<Temperature>,
    lifecycle_states: Vec<LifecycleState>,
    runtime_flags: Vec<RuntimeFlags>,
    pressures: Vec<f32>,
    contact_stimulus_current: Vec<f32>,
    contact_stimulus_next: Vec<f32>,
    genome_ids: Vec<Option<GenomeId>>,
    genome_carrier_amounts: Vec<f32>,
    action_plans: Vec<ActionPlan>,
    next_genome_decision_due_ticks: Vec<u64>,
    genome_decision_offsets: Vec<u64>,
    next_cell_id: u32,
}

impl CellStore {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ids: Vec::with_capacity(capacity),
            positions: Vec::with_capacity(capacity),
            radii: Vec::with_capacity(capacity),
            energy_buffers: Vec::with_capacity(capacity),
            resources: Vec::with_capacity(capacity),
            typed_resource_types: Vec::new(),
            typed_resources: Vec::new(),
            boundary_materials: Vec::with_capacity(capacity),
            transport_materials: Vec::with_capacity(capacity),
            metabolic_materials: Vec::with_capacity(capacity),
            storage_materials: Vec::with_capacity(capacity),
            synthesis_materials: Vec::with_capacity(capacity),
            structural_materials: Vec::with_capacity(capacity),
            repair_materials: Vec::with_capacity(capacity),
            contractile_materials: Vec::with_capacity(capacity),
            sensory_materials: Vec::with_capacity(capacity),
            material_damage: Vec::with_capacity(capacity),
            capacity_limits: Vec::with_capacity(capacity),
            temperatures: Vec::with_capacity(capacity),
            lifecycle_states: Vec::with_capacity(capacity),
            runtime_flags: Vec::with_capacity(capacity),
            pressures: Vec::with_capacity(capacity),
            contact_stimulus_current: Vec::with_capacity(capacity),
            contact_stimulus_next: Vec::with_capacity(capacity),
            genome_ids: Vec::with_capacity(capacity),
            genome_carrier_amounts: Vec::with_capacity(capacity),
            action_plans: Vec::with_capacity(capacity),
            next_genome_decision_due_ticks: Vec::with_capacity(capacity),
            genome_decision_offsets: Vec::with_capacity(capacity),
            next_cell_id: 1,
        }
    }

    pub fn insert_initial(&mut self, cell: InitialCellState) -> CellId {
        let id = CellId::from_raw(self.next_cell_id);
        self.next_cell_id += 1;
        self.ids.push(id);
        self.positions.push(cell.position);
        self.radii.push(cell.radius);
        self.energy_buffers.push(cell.energy);
        self.resources.push(cell.resources);
        self.typed_resources.extend(std::iter::repeat_n(
            ResourceAmount::zero(),
            self.typed_resource_types.len(),
        ));
        self.boundary_materials.push(cell.boundary_material);
        self.transport_materials.push(cell.transport_material);
        self.metabolic_materials.push(cell.metabolic_material);
        self.storage_materials.push(cell.storage_material);
        self.synthesis_materials.push(cell.synthesis_material);
        self.structural_materials.push(cell.structural_material);
        self.repair_materials.push(cell.repair_material);
        self.contractile_materials.push(cell.contractile_material);
        self.sensory_materials.push(cell.sensory_material);
        self.material_damage.push([0.0; 9]);
        self.capacity_limits.push(cell.capacity_limit);
        self.temperatures.push(cell.temperature);
        self.lifecycle_states.push(LifecycleState::Alive);
        self.runtime_flags.push(RuntimeFlags::default());
        self.pressures.push(0.0);
        self.contact_stimulus_current.push(0.0);
        self.contact_stimulus_next.push(0.0);
        self.genome_ids.push(None);
        self.genome_carrier_amounts.push(0.0);
        self.action_plans.push(ActionPlan::empty());
        self.next_genome_decision_due_ticks.push(1);
        self.genome_decision_offsets.push(0);
        id
    }

    pub fn insert_partitioned_daughter(&mut self, cell: InitialCellState) -> CellId {
        self.insert_initial(cell)
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn iter_indices(&self) -> impl Iterator<Item = CellIndex> {
        (0..self.len()).map(CellIndex::from_raw)
    }

    pub fn resolve_id_cold(&self, id: CellId) -> Option<CellIndex> {
        self.ids
            .iter()
            .position(|candidate| *candidate == id)
            .map(CellIndex)
    }

    pub fn id_at(&self, index: CellIndex) -> CellId {
        self.ids[index.raw()]
    }

    pub fn temperature(&self, index: CellIndex) -> Temperature {
        self.temperatures[index.raw()]
    }

    pub fn position(&self, index: CellIndex) -> Position {
        self.positions[index.raw()]
    }

    pub fn radius(&self, index: CellIndex) -> Radius {
        self.radii[index.raw()]
    }

    pub fn energy(&self, index: CellIndex) -> EnergyBuffer {
        self.energy_buffers[index.raw()]
    }

    pub fn lifecycle_state(&self, index: CellIndex) -> LifecycleState {
        self.lifecycle_states[index.raw()]
    }

    pub fn runtime_flags(&self, index: CellIndex) -> RuntimeFlags {
        self.runtime_flags[index.raw()]
    }

    pub fn genome_id(&self, index: CellIndex) -> Option<GenomeId> {
        self.genome_ids[index.raw()]
    }

    pub fn set_genome_id(&mut self, index: CellIndex, genome_id: Option<GenomeId>) {
        self.genome_ids[index.raw()] = genome_id;
    }

    pub fn set_genome_carrier_amount(&mut self, index: CellIndex, amount: f32) {
        self.genome_carrier_amounts[index.raw()] = amount.max(0.0);
    }

    pub fn action_plan(&self, index: CellIndex) -> &ActionPlan {
        &self.action_plans[index.raw()]
    }

    pub fn set_action_plan(&mut self, index: CellIndex, plan: ActionPlan) {
        self.action_plans[index.raw()] = plan;
    }

    pub fn next_genome_decision_due_tick(&self, index: CellIndex) -> u64 {
        self.next_genome_decision_due_ticks[index.raw()]
    }

    pub fn set_next_genome_decision_due_tick(&mut self, index: CellIndex, tick: u64) {
        self.next_genome_decision_due_ticks[index.raw()] = tick;
    }

    pub fn genome_decision_offset(&self, index: CellIndex) -> u64 {
        self.genome_decision_offsets[index.raw()]
    }

    pub fn set_genome_decision_offset(&mut self, index: CellIndex, offset: u64) {
        self.genome_decision_offsets[index.raw()] = offset;
    }

    pub fn used_capacity(&self, index: CellIndex) -> CapacityAmount {
        let genome_capacity_used = self.genome_carrier_amounts[index.raw()];
        let internal_fragments_capacity_used = 0.0;
        let used = self.resource_amount(index).raw()
            + self.total_materials(index).raw()
            + genome_capacity_used
            + internal_fragments_capacity_used;
        CapacityAmount::new(used).expect("resource/material amounts are validated")
    }

    pub fn capacity_limit(&self, index: CellIndex) -> CapacityAmount {
        self.capacity_limits[index.raw()]
    }

    pub fn free_capacity(&self, index: CellIndex) -> CapacityAmount {
        let free =
            (self.capacity_limits[index.raw()].raw() - self.used_capacity(index).raw()).max(0.0);
        CapacityAmount::new(free).expect("free capacity is clamped")
    }

    pub fn effective_capacity_limit(
        &self,
        index: CellIndex,
        storage_capacity_per_unit: f32,
    ) -> CapacityAmount {
        let storage_bonus = self
            .material_amount_for_slot(index, MaterialSlot::Storage)
            .raw()
            * storage_capacity_per_unit;
        CapacityAmount::new((self.capacity_limits[index.raw()].raw() + storage_bonus).max(0.0))
            .expect("effective capacity is clamped")
    }

    pub fn effective_free_capacity(
        &self,
        index: CellIndex,
        storage_capacity_per_unit: f32,
    ) -> CapacityAmount {
        let free = (self
            .effective_capacity_limit(index, storage_capacity_per_unit)
            .raw()
            - self.used_capacity(index).raw())
        .max(0.0);
        CapacityAmount::new(free).expect("effective free capacity is clamped")
    }

    pub fn resource_amount(&self, index: CellIndex) -> ResourceAmount {
        self.resources[index.raw()].saturating_add(self.typed_resource_total(index))
    }

    pub fn generic_resource_amount(&self, index: CellIndex) -> ResourceAmount {
        self.resources[index.raw()]
    }

    pub fn configure_typed_resource_types(
        &mut self,
        mut resource_types: Vec<ResourceTypeId>,
    ) -> Result<(), TypedResourceInventoryError> {
        if !self.ids.is_empty() || !self.typed_resource_types.is_empty() {
            return Err(TypedResourceInventoryError::AlreadyInitialized);
        }
        resource_types.sort();
        if let Some(duplicate) = resource_types
            .windows(2)
            .find(|pair| pair[0] == pair[1])
            .map(|pair| pair[0])
        {
            return Err(TypedResourceInventoryError::DuplicateType(duplicate));
        }
        self.typed_resource_types = resource_types;
        Ok(())
    }

    pub fn typed_resource_amount(
        &self,
        index: CellIndex,
        resource_type: ResourceTypeId,
    ) -> Result<ResourceAmount, TypedResourceInventoryError> {
        let offset = self.typed_resource_offset(index, resource_type)?;
        Ok(self.typed_resources[offset])
    }

    pub fn set_typed_resource_amount(
        &mut self,
        index: CellIndex,
        resource_type: ResourceTypeId,
        amount: ResourceAmount,
    ) -> Result<(), TypedResourceInventoryError> {
        let offset = self.typed_resource_offset(index, resource_type)?;
        self.typed_resources[offset] = amount;
        Ok(())
    }

    pub fn partition_typed_resources(
        &mut self,
        source: CellIndex,
        target: CellIndex,
        source_ratio: f32,
        retained_fraction: f32,
    ) -> Result<(), TypedResourceInventoryError> {
        let width = self.typed_resource_types.len();
        if width == 0 {
            return Ok(());
        }
        let source_start = source.raw() * width;
        let target_start = target.raw() * width;
        let target_ratio = 1.0 - source_ratio;
        for offset in 0..width {
            let current = self.typed_resources[source_start + offset].raw();
            self.typed_resources[source_start + offset] =
                ResourceAmount::new_unchecked(current * source_ratio * retained_fraction);
            self.typed_resources[target_start + offset] =
                ResourceAmount::new_unchecked(current * target_ratio * retained_fraction);
        }
        Ok(())
    }

    pub fn consume_typed_resource(
        &mut self,
        index: CellIndex,
        resource_type: ResourceTypeId,
        requested: ResourceAmount,
    ) -> Result<ResourceAmount, TypedResourceInventoryError> {
        let offset = self.typed_resource_offset(index, resource_type)?;
        let consumed = self.typed_resources[offset].saturating_sub(requested);
        let actual = self.typed_resources[offset].saturating_sub(consumed);
        self.typed_resources[offset] = consumed;
        Ok(actual)
    }

    fn typed_resource_total(&self, index: CellIndex) -> ResourceAmount {
        let width = self.typed_resource_types.len();
        if width == 0 {
            return ResourceAmount::zero();
        }
        let start = index.raw() * width;
        let total = self.typed_resources[start..start + width]
            .iter()
            .map(|amount| amount.raw())
            .sum();
        ResourceAmount::new(total).expect("typed resource amounts are validated")
    }

    fn typed_resource_offset(
        &self,
        index: CellIndex,
        resource_type: ResourceTypeId,
    ) -> Result<usize, TypedResourceInventoryError> {
        let type_index = self
            .typed_resource_types
            .binary_search(&resource_type)
            .map_err(|_| TypedResourceInventoryError::UnknownType(resource_type))?;
        Ok(index.raw() * self.typed_resource_types.len() + type_index)
    }

    pub fn add_resources_limited_by_capacity(
        &mut self,
        index: CellIndex,
        requested: ResourceAmount,
    ) -> ResourceAmount {
        self.add_resources_limited_by_effective_capacity(index, requested, 0.0)
    }

    pub fn add_resources_limited_by_effective_capacity(
        &mut self,
        index: CellIndex,
        requested: ResourceAmount,
        storage_capacity_per_unit: f32,
    ) -> ResourceAmount {
        let accepted_raw = requested.raw().min(
            self.effective_free_capacity(index, storage_capacity_per_unit)
                .raw(),
        );
        let accepted = ResourceAmount::new(accepted_raw).expect("accepted uptake is clamped");
        self.resources[index.raw()] = self.resources[index.raw()].saturating_add(accepted);
        accepted
    }

    pub fn consume_resources(
        &mut self,
        index: CellIndex,
        requested: ResourceAmount,
    ) -> ResourceAmount {
        let available = self.resources[index.raw()];
        let consumed_raw = requested.raw().min(available.raw());
        let consumed = ResourceAmount::new(consumed_raw).expect("consumed resource is clamped");
        self.resources[index.raw()] = available.saturating_sub(consumed);
        consumed
    }

    pub fn transfer_resources_limited_by_effective_capacity(
        &mut self,
        source: CellIndex,
        target: CellIndex,
        requested: ResourceAmount,
        storage_capacity_per_unit: f32,
    ) -> ResourceAmount {
        let source_available = self.generic_resource_amount(source).raw();
        let target_free = self
            .effective_free_capacity(target, storage_capacity_per_unit)
            .raw();
        let accepted_raw = requested.raw().min(source_available).min(target_free);
        let accepted = ResourceAmount::new(accepted_raw).expect("accepted transfer is clamped");
        self.resources[source.raw()] = self.resources[source.raw()].saturating_sub(accepted);
        self.resources[target.raw()] = self.resources[target.raw()].saturating_add(accepted);
        accepted
    }

    pub fn set_energy(&mut self, index: CellIndex, energy: EnergyBuffer) {
        self.energy_buffers[index.raw()] = energy;
    }

    pub fn set_position(&mut self, index: CellIndex, position: Position) {
        self.positions[index.raw()] = position;
    }

    pub fn set_temperature(&mut self, index: CellIndex, temperature: Temperature) {
        self.temperatures[index.raw()] = temperature;
    }

    pub fn set_lifecycle_state(&mut self, index: CellIndex, state: LifecycleState) {
        self.lifecycle_states[index.raw()] = state;
    }

    pub fn set_runtime_flags(&mut self, index: CellIndex, flags: RuntimeFlags) {
        self.runtime_flags[index.raw()] = flags;
    }

    pub fn has_capability(&self, index: CellIndex, capability: MaterialCapability) -> bool {
        self.capability_level(index, capability) > 0.0
    }

    pub fn capability_level(&self, index: CellIndex, capability: MaterialCapability) -> f32 {
        if self.lifecycle_state(index) == LifecycleState::Dead {
            return 0.0;
        }
        match capability {
            MaterialCapability::BoundaryPermeability => self
                .material_amount_for_slot(index, MaterialSlot::Boundary)
                .raw(),
            MaterialCapability::ResourceUptake => self
                .material_amount_for_slot(index, MaterialSlot::Transport)
                .raw(),
            MaterialCapability::Metabolism => self
                .material_amount_for_slot(index, MaterialSlot::Metabolic)
                .raw(),
            MaterialCapability::StorageCapacity => self
                .material_amount_for_slot(index, MaterialSlot::Storage)
                .raw(),
            MaterialCapability::MaterialSynthesis => self
                .material_amount_for_slot(index, MaterialSlot::Synthesis)
                .raw(),
            MaterialCapability::StructuralGrowth => self
                .material_amount_for_slot(index, MaterialSlot::Structural)
                .raw(),
            MaterialCapability::Repair => self
                .material_amount_for_slot(index, MaterialSlot::Repair)
                .raw(),
            MaterialCapability::Contractility => self
                .material_amount_for_slot(index, MaterialSlot::Contractile)
                .raw(),
            MaterialCapability::ResourceSensing
            | MaterialCapability::PressureSensing
            | MaterialCapability::DamageSensing => self
                .material_amount_for_slot(index, MaterialSlot::Sensory)
                .raw(),
        }
    }

    pub fn contact_pressure(&self, index: CellIndex) -> f32 {
        self.pressures[index.raw()]
    }

    pub fn set_contact_pressure(&mut self, index: CellIndex, pressure: f32) {
        self.pressures[index.raw()] = pressure;
    }

    pub fn contact_stimulus(&self, index: CellIndex) -> f32 {
        self.contact_stimulus_current[index.raw()]
    }

    pub fn add_next_contact_stimulus(&mut self, index: CellIndex, amount: f32) {
        self.contact_stimulus_next[index.raw()] =
            (self.contact_stimulus_next[index.raw()] + amount.max(0.0)).clamp(0.0, 1.0);
    }

    pub fn commit_contact_stimulus(&mut self, decay_per_tick: f32) {
        let decay = decay_per_tick.clamp(0.0, 1.0);
        for i in 0..self.contact_stimulus_current.len() {
            let decayed_current = self.contact_stimulus_current[i] * (1.0 - decay);
            self.contact_stimulus_current[i] =
                (decayed_current + self.contact_stimulus_next[i]).clamp(0.0, 1.0);
            self.contact_stimulus_next[i] = 0.0;
        }
    }

    pub fn material_amount(&self, index: CellIndex) -> MaterialAmount {
        self.total_materials(index)
    }

    pub fn material_amount_for_slot(&self, index: CellIndex, slot: MaterialSlot) -> MaterialAmount {
        match slot {
            MaterialSlot::Boundary => self.boundary_material(index),
            MaterialSlot::Transport => self.transport_material(index),
            MaterialSlot::Metabolic => self.metabolic_material(index),
            MaterialSlot::Storage => self.storage_material(index),
            MaterialSlot::Synthesis => self.synthesis_material(index),
            MaterialSlot::Structural => self.structural_material(index),
            MaterialSlot::Repair => self.repair_material(index),
            MaterialSlot::Contractile => self.contractile_material(index),
            MaterialSlot::Sensory => self.sensory_material(index),
        }
    }

    pub fn set_material_amount_for_slot(
        &mut self,
        index: CellIndex,
        slot: MaterialSlot,
        amount: MaterialAmount,
    ) {
        match slot {
            MaterialSlot::Boundary => self.set_boundary_material(index, amount),
            MaterialSlot::Transport => self.set_transport_material(index, amount),
            MaterialSlot::Metabolic => self.set_metabolic_material(index, amount),
            MaterialSlot::Storage => self.set_storage_material(index, amount),
            MaterialSlot::Synthesis => self.set_synthesis_material(index, amount),
            MaterialSlot::Structural => self.set_structural_material(index, amount),
            MaterialSlot::Repair => self.set_repair_material(index, amount),
            MaterialSlot::Contractile => self.set_contractile_material(index, amount),
            MaterialSlot::Sensory => self.set_sensory_material(index, amount),
        }
    }

    pub fn material_damage(&self, index: CellIndex, slot: MaterialSlot) -> f32 {
        self.material_damage[index.raw()][slot.index()]
    }

    pub fn set_material_damage(&mut self, index: CellIndex, slot: MaterialSlot, damage: f32) {
        self.material_damage[index.raw()][slot.index()] = damage.clamp(0.0, 1.0);
    }

    pub fn apply_thermal_damage(
        &mut self,
        index: CellIndex,
        slot: MaterialSlot,
        temperature: Temperature,
        tolerance: f32,
        damage_rate: f32,
    ) -> MaterialAmount {
        if temperature.raw() <= tolerance || damage_rate <= 0.0 {
            return MaterialAmount::zero();
        }

        let current = self.material_amount_for_slot(index, slot);
        if current.raw() <= 0.0 {
            return MaterialAmount::zero();
        }

        let damage_delta = damage_rate.clamp(0.0, 1.0);
        let previous_damage = self.material_damage(index, slot);
        let next_damage = (previous_damage + damage_delta).clamp(0.0, 1.0);
        let effective_delta = (next_damage - previous_damage).max(0.0);
        if effective_delta <= 0.0 {
            return MaterialAmount::zero();
        }

        let degraded =
            MaterialAmount::new_unchecked((current.raw() * effective_delta).min(current.raw()));
        let remaining = MaterialAmount::new_unchecked((current.raw() - degraded.raw()).max(0.0));
        self.set_material_amount_for_slot(index, slot, remaining);
        self.set_material_damage(index, slot, next_damage);
        degraded
    }

    pub fn boundary_allows_passive_exchange(&self, index: CellIndex) -> bool {
        self.boundary_material(index).raw() > 0.0
            && self.transport_material(index).raw() > 0.0
            && self.material_damage(index, MaterialSlot::Boundary) < 1.0
    }

    pub fn boundary_leakage_rate(
        &self,
        index: CellIndex,
        resource_rule: PermeabilityConstraint,
        material_permeability: f32,
    ) -> f32 {
        if resource_rule != PermeabilityConstraint::Passive
            || self.boundary_material(index).raw() <= 0.0
            || material_permeability <= 0.0
        {
            return 0.0;
        }

        let damage = self.material_damage(index, MaterialSlot::Boundary);
        (material_permeability * damage).clamp(0.0, 1.0)
    }

    pub fn material_composition(&self, index: CellIndex) -> MaterialComposition {
        MaterialComposition::from_slots([
            (
                MaterialSlot::Boundary,
                self.material_amount_for_slot(index, MaterialSlot::Boundary),
            ),
            (
                MaterialSlot::Transport,
                self.material_amount_for_slot(index, MaterialSlot::Transport),
            ),
            (
                MaterialSlot::Metabolic,
                self.material_amount_for_slot(index, MaterialSlot::Metabolic),
            ),
            (
                MaterialSlot::Storage,
                self.material_amount_for_slot(index, MaterialSlot::Storage),
            ),
            (
                MaterialSlot::Synthesis,
                self.material_amount_for_slot(index, MaterialSlot::Synthesis),
            ),
            (
                MaterialSlot::Structural,
                self.material_amount_for_slot(index, MaterialSlot::Structural),
            ),
            (
                MaterialSlot::Repair,
                self.material_amount_for_slot(index, MaterialSlot::Repair),
            ),
            (
                MaterialSlot::Contractile,
                self.material_amount_for_slot(index, MaterialSlot::Contractile),
            ),
            (
                MaterialSlot::Sensory,
                self.material_amount_for_slot(index, MaterialSlot::Sensory),
            ),
        ])
    }

    pub fn set_radius(&mut self, index: CellIndex, radius: Radius) {
        self.radii[index.raw()] = radius;
    }

    pub fn set_capacity_limit(&mut self, index: CellIndex, limit: CapacityAmount) {
        self.capacity_limits[index.raw()] = limit;
    }

    pub fn set_materials(&mut self, index: CellIndex, amount: MaterialAmount) {
        let share = MaterialAmount::new_unchecked(amount.raw() / 9.0);
        self.boundary_materials[index.raw()] = share;
        self.transport_materials[index.raw()] = share;
        self.metabolic_materials[index.raw()] = share;
        self.storage_materials[index.raw()] = share;
        self.synthesis_materials[index.raw()] = share;
        self.structural_materials[index.raw()] = share;
        self.repair_materials[index.raw()] = share;
        self.contractile_materials[index.raw()] = share;
        self.sensory_materials[index.raw()] = share;
    }

    pub fn set_resources(&mut self, index: CellIndex, amount: ResourceAmount) {
        self.resources[index.raw()] = amount;
    }

    // Getters for specific materials
    pub fn boundary_material(&self, index: CellIndex) -> MaterialAmount {
        self.boundary_materials[index.raw()]
    }
    pub fn transport_material(&self, index: CellIndex) -> MaterialAmount {
        self.transport_materials[index.raw()]
    }
    pub fn metabolic_material(&self, index: CellIndex) -> MaterialAmount {
        self.metabolic_materials[index.raw()]
    }
    pub fn storage_material(&self, index: CellIndex) -> MaterialAmount {
        self.storage_materials[index.raw()]
    }
    pub fn synthesis_material(&self, index: CellIndex) -> MaterialAmount {
        self.synthesis_materials[index.raw()]
    }
    pub fn structural_material(&self, index: CellIndex) -> MaterialAmount {
        self.structural_materials[index.raw()]
    }
    pub fn repair_material(&self, index: CellIndex) -> MaterialAmount {
        self.repair_materials[index.raw()]
    }
    pub fn contractile_material(&self, index: CellIndex) -> MaterialAmount {
        self.contractile_materials[index.raw()]
    }
    pub fn sensory_material(&self, index: CellIndex) -> MaterialAmount {
        self.sensory_materials[index.raw()]
    }

    // Setters for specific materials
    pub fn set_boundary_material(&mut self, index: CellIndex, amount: MaterialAmount) {
        self.boundary_materials[index.raw()] = amount;
    }
    pub fn set_transport_material(&mut self, index: CellIndex, amount: MaterialAmount) {
        self.transport_materials[index.raw()] = amount;
    }
    pub fn set_metabolic_material(&mut self, index: CellIndex, amount: MaterialAmount) {
        self.metabolic_materials[index.raw()] = amount;
    }
    pub fn set_storage_material(&mut self, index: CellIndex, amount: MaterialAmount) {
        self.storage_materials[index.raw()] = amount;
    }
    pub fn set_synthesis_material(&mut self, index: CellIndex, amount: MaterialAmount) {
        self.synthesis_materials[index.raw()] = amount;
    }
    pub fn set_structural_material(&mut self, index: CellIndex, amount: MaterialAmount) {
        self.structural_materials[index.raw()] = amount;
    }
    pub fn set_repair_material(&mut self, index: CellIndex, amount: MaterialAmount) {
        self.repair_materials[index.raw()] = amount;
    }
    pub fn set_contractile_material(&mut self, index: CellIndex, amount: MaterialAmount) {
        self.contractile_materials[index.raw()] = amount;
    }
    pub fn set_sensory_material(&mut self, index: CellIndex, amount: MaterialAmount) {
        self.sensory_materials[index.raw()] = amount;
    }

    pub fn total_materials(&self, index: CellIndex) -> MaterialAmount {
        let total = self.boundary_materials[index.raw()].raw()
            + self.transport_materials[index.raw()].raw()
            + self.metabolic_materials[index.raw()].raw()
            + self.storage_materials[index.raw()].raw()
            + self.synthesis_materials[index.raw()].raw()
            + self.structural_materials[index.raw()].raw()
            + self.repair_materials[index.raw()].raw()
            + self.contractile_materials[index.raw()].raw()
            + self.sensory_materials[index.raw()].raw();
        MaterialAmount::new_unchecked(total)
    }
}
