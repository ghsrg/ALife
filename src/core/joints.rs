use crate::core::cell_store::CellIndex;
use crate::core::ids::JointId;
use crate::core::units::{MaterialAmount, Tick};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JointEndpoints {
    pub a: CellIndex,
    pub b: CellIndex,
}

impl JointEndpoints {
    pub fn new(a: CellIndex, b: CellIndex) -> Option<Self> {
        if a == b {
            return None;
        }
        Some(if a.raw() < b.raw() {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        })
    }

    pub fn contains(self, cell: CellIndex) -> bool {
        self.a == cell || self.b == cell
    }

    pub fn other(self, cell: CellIndex) -> Option<CellIndex> {
        if self.a == cell {
            Some(self.b)
        } else if self.b == cell {
            Some(self.a)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JointChannelConfig {
    pub mechanical_strength: f32,
    pub resource_transfer_rate: f32,
    pub max_resource_transfer_per_tick: f32,
    pub signal_conductivity: f32,
    pub signal_decay: f32,
    pub heat_conductivity: f32,
}

impl JointChannelConfig {
    pub const fn mechanical_only(mechanical_strength: f32) -> Self {
        Self {
            mechanical_strength,
            resource_transfer_rate: 0.0,
            max_resource_transfer_per_tick: 0.0,
            signal_conductivity: 0.0,
            signal_decay: 0.0,
            heat_conductivity: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JointLifecycle {
    Active,
    Inert,
    Broken,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JointStoreError {
    UnknownJoint,
}

#[derive(Clone, Debug, Default)]
pub struct JointStore {
    endpoints: Vec<JointEndpoints>,
    material_amounts: Vec<MaterialAmount>,
    configs: Vec<JointChannelConfig>,
    lifecycle: Vec<JointLifecycle>,
    damage: Vec<f32>,
    created_tick: Vec<Tick>,
    broken_tick: Vec<Option<Tick>>,
    signal_current: Vec<f32>,
    signal_next: Vec<f32>,
    signal_readable_from: Vec<Tick>,
}

impl JointStore {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            endpoints: Vec::with_capacity(capacity),
            material_amounts: Vec::with_capacity(capacity),
            configs: Vec::with_capacity(capacity),
            lifecycle: Vec::with_capacity(capacity),
            damage: Vec::with_capacity(capacity),
            created_tick: Vec::with_capacity(capacity),
            broken_tick: Vec::with_capacity(capacity),
            signal_current: Vec::with_capacity(capacity),
            signal_next: Vec::with_capacity(capacity),
            signal_readable_from: Vec::with_capacity(capacity),
        }
    }

    pub fn create(
        &mut self,
        endpoints: JointEndpoints,
        material_amount: MaterialAmount,
        config: JointChannelConfig,
        tick: Tick,
    ) -> JointId {
        let id = JointId::from_raw(self.endpoints.len() as u32);
        self.endpoints.push(endpoints);
        self.material_amounts.push(material_amount);
        self.configs.push(config);
        self.lifecycle.push(JointLifecycle::Active);
        self.damage.push(0.0);
        self.created_tick.push(tick);
        self.broken_tick.push(None);
        self.signal_current.push(0.0);
        self.signal_next.push(0.0);
        self.signal_readable_from.push(Tick::from_raw(u64::MAX));
        id
    }

    pub fn len(&self) -> usize {
        self.endpoints.len()
    }

    pub fn is_empty(&self) -> bool {
        self.endpoints.is_empty()
    }

    pub fn endpoints(&self, id: JointId) -> Option<JointEndpoints> {
        self.endpoints.get(id.raw() as usize).copied()
    }

    pub fn config(&self, id: JointId) -> Option<JointChannelConfig> {
        self.configs.get(id.raw() as usize).copied()
    }

    pub fn material_amount(&self, id: JointId) -> Option<MaterialAmount> {
        self.material_amounts.get(id.raw() as usize).copied()
    }

    pub fn is_active(&self, id: JointId) -> Option<bool> {
        self.lifecycle
            .get(id.raw() as usize)
            .map(|state| *state == JointLifecycle::Active)
    }

    pub fn is_broken(&self, id: JointId) -> Option<bool> {
        self.lifecycle
            .get(id.raw() as usize)
            .map(|state| *state == JointLifecycle::Broken)
    }

    pub fn active_ids(&self) -> impl Iterator<Item = JointId> + '_ {
        self.lifecycle
            .iter()
            .enumerate()
            .filter_map(|(index, state)| {
                (*state == JointLifecycle::Active).then_some(JointId::from_raw(index as u32))
            })
    }

    pub fn all_ids(&self) -> impl Iterator<Item = JointId> + '_ {
        (0..self.endpoints.len()).map(|index| JointId::from_raw(index as u32))
    }

    pub fn has_active_between(&self, endpoints: JointEndpoints) -> bool {
        self.active_ids()
            .any(|id| self.endpoints(id) == Some(endpoints))
    }

    pub fn readable_signal(&self, id: JointId, tick: Tick) -> Option<f32> {
        let index = id.raw() as usize;
        let readable_from = self.signal_readable_from.get(index).copied()?;
        if readable_from <= tick {
            self.signal_current.get(index).copied()
        } else {
            Some(0.0)
        }
    }

    pub fn add_next_signal(&mut self, id: JointId, amount: f32, readable_from: Tick) {
        let index = id.raw() as usize;
        if let Some(next) = self.signal_next.get_mut(index) {
            *next = (*next + amount).clamp(0.0, 1.0);
            self.signal_readable_from[index] = readable_from;
        }
    }

    pub fn begin_tick_signal_rollover(&mut self, decay: f32) {
        let retain = 1.0 - decay.clamp(0.0, 1.0);
        for index in 0..self.signal_current.len() {
            self.signal_current[index] = (self.signal_next[index] * retain).clamp(0.0, 1.0);
            self.signal_next[index] = 0.0;
        }
    }

    pub fn break_joint(&mut self, id: JointId, tick: Tick) -> Result<(), JointStoreError> {
        let index = id.raw() as usize;
        let Some(state) = self.lifecycle.get_mut(index) else {
            return Err(JointStoreError::UnknownJoint);
        };
        *state = JointLifecycle::Broken;
        self.broken_tick[index] = Some(tick);
        Ok(())
    }

    pub fn degrade_active(&mut self, rate: f32, threshold: f32, tick: Tick) -> (f32, u32) {
        let mut degraded = 0.0_f32;
        let mut broken = 0_u32;
        let rate = rate.clamp(0.0, 1.0);
        for index in 0..self.lifecycle.len() {
            if self.lifecycle[index] != JointLifecycle::Active {
                continue;
            }
            let current = self.material_amounts[index].raw();
            let amount = current * rate;
            if amount > 0.0 {
                degraded += amount;
                self.material_amounts[index] =
                    MaterialAmount::new_unchecked((current - amount).max(0.0));
                self.damage[index] += amount;
            }
            if self.damage[index] >= threshold || self.material_amounts[index].raw() <= 0.0 {
                self.lifecycle[index] = JointLifecycle::Broken;
                self.broken_tick[index] = Some(tick);
                broken += 1;
            }
        }
        (degraded, broken)
    }

    pub fn make_inert_for_endpoint(&mut self, endpoint: CellIndex) -> u32 {
        let mut changed = 0_u32;
        for index in 0..self.lifecycle.len() {
            if self.lifecycle[index] == JointLifecycle::Active
                && self.endpoints[index].contains(endpoint)
            {
                self.lifecycle[index] = JointLifecycle::Inert;
                changed += 1;
            }
        }
        changed
    }

    pub fn break_for_endpoint(&mut self, endpoint: CellIndex, tick: Tick) -> u32 {
        let mut changed = 0_u32;
        for index in 0..self.lifecycle.len() {
            if self.lifecycle[index] != JointLifecycle::Broken
                && self.endpoints[index].contains(endpoint)
            {
                self.lifecycle[index] = JointLifecycle::Broken;
                self.broken_tick[index] = Some(tick);
                changed += 1;
            }
        }
        changed
    }
}
