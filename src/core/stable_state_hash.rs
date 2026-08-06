use crate::core::cell_store::{LifecycleState, RuntimeFlags};
use crate::core::fields::FieldLayerIndex;
use crate::core::genome::GenomeOutputId;
use crate::core::ids::JointId;
use crate::core::materials::MaterialSlot;
use crate::core::process::ProcessId;
use crate::core::resources::ResourceLayerIndex;
use crate::core::units::GridCoord;
use crate::core::world::WorldState;

pub struct StableStateHasher;

impl StableStateHasher {
    pub fn hash_world(world: &WorldState) -> u64 {
        let mut hasher = StableHasher::new("scheduler-determinism-v1");
        hasher.add_u64(world.tick().raw());
        hasher.add_u64(world.config().config_hash());
        hasher.add_cells(world);
        hasher.add_resources(world);
        hasher.add_fields(world);
        hasher.add_environment(world);
        hasher.add_fragments(world);
        hasher.add_joints(world);
        hasher.finish()
    }
}

struct StableHasher {
    hash: u64,
}

impl StableHasher {
    fn new(domain: &str) -> Self {
        let mut hasher = Self {
            hash: 0xcbf2_9ce4_8422_2325,
        };
        hasher.add_str(domain);
        hasher
    }

    fn add_u64(&mut self, value: u64) {
        for byte in value.to_le_bytes() {
            self.hash ^= u64::from(byte);
            self.hash = self.hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn add_u32(&mut self, value: u32) {
        self.add_u64(u64::from(value));
    }

    fn add_usize(&mut self, value: usize) {
        self.add_u64(value as u64);
    }

    fn add_f32(&mut self, value: f32) {
        self.add_u32(value.to_bits());
    }

    fn add_bool(&mut self, value: bool) {
        self.add_u64(u64::from(value));
    }

    fn add_str(&mut self, value: &str) {
        self.add_usize(value.len());
        for byte in value.as_bytes() {
            self.hash ^= u64::from(*byte);
            self.hash = self.hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn add_cells(&mut self, world: &WorldState) {
        let cells = world.cells();
        self.add_usize(cells.len());
        for index in cells.iter_indices() {
            self.add_u32(cells.id_at(index).raw());
            self.add_position(cells.position(index));
            self.add_f32(cells.radius(index).raw());
            self.add_f32(cells.energy(index).current().raw());
            self.add_f32(cells.energy(index).capacity().raw());
            self.add_lifecycle(cells.lifecycle_state(index));
            self.add_runtime_flags(cells.runtime_flags(index));
            self.add_f32(cells.generic_resource_amount(index).raw());
            for slot in [
                MaterialSlot::Boundary,
                MaterialSlot::Transport,
                MaterialSlot::Metabolic,
                MaterialSlot::Storage,
                MaterialSlot::Synthesis,
                MaterialSlot::Structural,
                MaterialSlot::Repair,
                MaterialSlot::Contractile,
                MaterialSlot::Sensory,
            ] {
                self.add_f32(cells.material_amount_for_slot(index, slot).raw());
                self.add_f32(cells.material_damage(index, slot));
            }
            self.add_f32(cells.capacity_limit(index).raw());
            self.add_f32(cells.temperature(index).raw());
            self.add_f32(cells.contact_pressure(index));
            self.add_f32(cells.contact_stimulus(index));
            match cells.genome_id(index) {
                Some(genome_id) => {
                    self.add_bool(true);
                    self.add_u32(genome_id.raw());
                    if let Some(genome) = world.genome(genome_id) {
                        self.add_str(genome.template_id.as_str());
                        self.add_str(&genome.carrier.material_id);
                        self.add_f32(genome.carrier.amount);
                        self.add_f32(genome.carrier.integrity);
                        self.add_usize(genome.outputs.len());
                        for (output_id, value) in &genome.outputs {
                            self.add_genome_output(*output_id);
                            self.add_f32(value.raw());
                        }
                    }
                }
                None => self.add_bool(false),
            }
            match cells.copied_genome_id(index) {
                Some(genome_id) => {
                    self.add_bool(true);
                    self.add_u32(genome_id.raw());
                    if let Some(genome) = world.genome(genome_id) {
                        self.add_str(genome.template_id.as_str());
                        self.add_str(&genome.carrier.material_id);
                        self.add_f32(genome.carrier.amount);
                        self.add_f32(genome.carrier.integrity);
                        self.add_usize(genome.outputs.len());
                        for (output_id, value) in &genome.outputs {
                            self.add_genome_output(*output_id);
                            self.add_f32(value.raw());
                        }
                    }
                }
                None => self.add_bool(false),
            }
            self.add_f32(cells.genome_copy_progress(index));
            self.add_f32(cells.copied_genome_carrier_amount(index));
            self.add_usize(cells.action_plan(index).ordered_processes().len());
            for process in cells.action_plan(index).ordered_processes() {
                self.add_process_id(*process);
            }
            self.add_u64(cells.next_genome_decision_due_tick(index));
            self.add_u64(cells.genome_decision_offset(index));
        }
    }

    fn add_resources(&mut self, world: &WorldState) {
        let resources = world.resources();
        self.add_usize(resources.width());
        self.add_usize(resources.height());
        self.add_usize(resources.layer_count());
        for layer in 0..resources.layer_count() {
            for y in 0..resources.height() {
                for x in 0..resources.width() {
                    let amount = resources
                        .amount_at(ResourceLayerIndex::from_raw(layer), GridCoord::new(x, y))
                        .expect("resource hash iterates valid grid coordinates");
                    self.add_f32(amount.raw());
                }
            }
        }
    }

    fn add_fields(&mut self, world: &WorldState) {
        let Some(fields) = world.fields() else {
            self.add_usize(0);
            return;
        };
        self.add_usize(fields.width());
        self.add_usize(fields.height());
        self.add_usize(fields.layer_count());
        for layer in 0..fields.layer_count() {
            for y in 0..fields.height() {
                for x in 0..fields.width() {
                    let value = fields
                        .value_at(FieldLayerIndex::from_raw(layer), GridCoord::new(x, y))
                        .expect("field hash iterates valid grid coordinates");
                    self.add_f32(value.raw());
                }
            }
        }
    }

    fn add_environment(&mut self, world: &WorldState) {
        let environment = world.environment();
        self.add_f32(environment.heat().raw());
        self.add_f32(environment.waste().raw());
    }

    fn add_fragments(&mut self, world: &WorldState) {
        let fragments = world.fragments();
        let mut count = 0_usize;
        for _ in fragments.iter() {
            count += 1;
        }
        self.add_usize(count);
        for (id, fragment) in fragments.iter() {
            self.add_u32(id.raw());
            self.add_u32(fragment.material_type_id().raw());
            self.add_f32(fragment.amount().raw());
            self.add_position(fragment.position());
            self.add_u64(fragment.created_tick().raw());
        }
    }

    fn add_joints(&mut self, world: &WorldState) {
        let joints = world.joints();
        self.add_usize(joints.len());
        for id in joints.all_ids() {
            self.add_joint_id(id);
            if let Some(endpoints) = joints.endpoints(id) {
                self.add_usize(endpoints.a.raw());
                self.add_usize(endpoints.b.raw());
            }
            if let Some(amount) = joints.material_amount(id) {
                self.add_f32(amount.raw());
            }
            if let Some(config) = joints.config(id) {
                self.add_f32(config.mechanical_strength);
                self.add_f32(config.resource_transfer_rate);
                self.add_f32(config.max_resource_transfer_per_tick);
                self.add_f32(config.signal_conductivity);
                self.add_f32(config.signal_decay);
                self.add_f32(config.heat_conductivity);
            }
            self.add_bool(joints.is_active(id).unwrap_or(false));
            self.add_bool(joints.is_broken(id).unwrap_or(false));
            self.add_f32(joints.readable_signal(id, world.tick()).unwrap_or(0.0));
        }
    }

    fn add_position(&mut self, position: crate::core::units::Position) {
        self.add_f32(position.x());
        self.add_f32(position.y());
    }
    fn add_joint_id(&mut self, id: JointId) {
        self.add_u32(id.raw());
    }

    fn add_lifecycle(&mut self, state: LifecycleState) {
        self.add_u64(match state {
            LifecycleState::Alive => 0,
            LifecycleState::Stressed => 1,
            LifecycleState::Dormant => 2,
            LifecycleState::Dead => 3,
        });
    }

    fn add_runtime_flags(&mut self, flags: RuntimeFlags) {
        self.add_bool(flags.mandatory_paid);
        self.add_bool(flags.stalled);
        self.add_bool(flags.over_capacity);
        self.add_bool(flags.inert);
        self.add_bool(flags.division_ready);
    }

    fn add_process_id(&mut self, process: ProcessId) {
        self.add_u64(match process {
            ProcessId::MandatoryUpkeep => 0,
            ProcessId::LocalResourceUptake => 1,
            ProcessId::MetabolismEnergyConversion => 2,
            ProcessId::MaterialSynthesis => 3,
            ProcessId::GrowthResourceAllocation => 4,
            ProcessId::Division => 5,
            ProcessId::ContractileDisplacement => 6,
            ProcessId::PassiveContactExchange => 7,
            ProcessId::RepairBoundary => 8,
            ProcessId::GenomeCopying => 9,
            ProcessId::JointCreate => 10,
            ProcessId::JointRepair => 11,
            ProcessId::GenomeRecombination => 12,
        });
    }

    fn add_genome_output(&mut self, output: GenomeOutputId) {
        self.add_u64(match output {
            GenomeOutputId::ResourceUptakePriority => 0,
            GenomeOutputId::EnergyConversionPriority => 1,
            GenomeOutputId::MaterialSynthesisPriority => 2,
            GenomeOutputId::RepairPriority => 3,
            GenomeOutputId::MovementPriority => 4,
            GenomeOutputId::DivisionPreparationPriority => 5,
            GenomeOutputId::GenomeCopyingPriority => 6,
            GenomeOutputId::GenomeRecombinationPriority => 7,
        });
    }

    fn finish(self) -> u64 {
        self.hash
    }
}
