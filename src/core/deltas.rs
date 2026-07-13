use std::collections::BTreeMap;

use crate::core::ids::{CellId, MaterialTypeId, ResourceTypeId};
use crate::core::reactions::{ReactionDelta, ReactionId, ReactionMode, ReactionTerm};
use crate::core::units::{EnergyAmount, GridCoord, HeatAmount, MaterialAmount, ResourceAmount};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CommitSummary {
    pub ticks_committed: u64,
    pub events_emitted: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReactionLocation {
    Grid(GridCoord),
}

impl ReactionLocation {
    fn key(self) -> (usize, usize) {
        match self {
            Self::Grid(c) => (c.x(), c.y()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReactionSource {
    Environment,
    Cell(CellId),
}

impl ReactionSource {
    fn key(self) -> (u8, u32) {
        match self {
            Self::Environment => (0, 0),
            Self::Cell(id) => (1, id.raw()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReactionDestination {
    Product(ReactionTerm),
    Retained(ReactionTerm),
    Residual(ReactionTerm),
    Sink(ReactionTerm),
}

impl ReactionDestination {
    fn term(self) -> ReactionTerm {
        match self {
            Self::Product(t) | Self::Retained(t) | Self::Residual(t) | Self::Sink(t) => t,
        }
    }
    fn produces(self) -> bool {
        !matches!(self, Self::Sink(_))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccountingRejection {
    UnaccountedInput,
    ProductsRequireInputs,
    InsufficientInput,
    PassiveEnergyCreditForbidden,
    ControlledEnergyNotAllowed,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReactionInventory {
    resources: BTreeMap<(usize, usize, u32), ResourceAmount>,
    materials: BTreeMap<(usize, usize, u32), MaterialAmount>,
}

impl ReactionInventory {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_resource(
        &mut self,
        at: ReactionLocation,
        id: ResourceTypeId,
        amount: ResourceAmount,
    ) {
        let (x, y) = at.key();
        self.resources.insert((x, y, id.raw()), amount);
    }
    pub fn set_material(
        &mut self,
        at: ReactionLocation,
        id: MaterialTypeId,
        amount: MaterialAmount,
    ) {
        let (x, y) = at.key();
        self.materials.insert((x, y, id.raw()), amount);
    }
    pub fn resource(&self, at: ReactionLocation, id: ResourceTypeId) -> ResourceAmount {
        let (x, y) = at.key();
        self.resources
            .get(&(x, y, id.raw()))
            .copied()
            .unwrap_or_else(ResourceAmount::zero)
    }
    pub fn material(&self, at: ReactionLocation, id: MaterialTypeId) -> MaterialAmount {
        let (x, y) = at.key();
        self.materials
            .get(&(x, y, id.raw()))
            .copied()
            .unwrap_or_else(MaterialAmount::zero)
    }
    fn available(&self, at: ReactionLocation, term: ReactionTerm) -> f32 {
        match term {
            ReactionTerm::Resource { id, .. } => self.resource(at, id).raw(),
            ReactionTerm::Material { id, .. } => self.material(at, id).raw(),
        }
    }
    fn add(&mut self, at: ReactionLocation, term: ReactionTerm, sign: f32) {
        match term {
            ReactionTerm::Resource { id, amount } => {
                let n = (self.resource(at, id).raw() + sign * amount.raw()).max(0.0);
                self.set_resource(at, id, ResourceAmount::new(n).unwrap());
            }
            ReactionTerm::Material { id, amount } => {
                let n = (self.material(at, id).raw() + sign * amount.raw()).max(0.0);
                self.set_material(at, id, MaterialAmount::new(n).unwrap());
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountingReport {
    inventory: ReactionInventory,
    accepted: Vec<ReactionId>,
    rejected: BTreeMap<ReactionId, AccountingRejection>,
    heat: BTreeMap<(usize, usize), HeatAmount>,
    energy: BTreeMap<(usize, usize), EnergyAmount>,
}

impl AccountingReport {
    pub fn validate_and_commit(
        mut inventory: ReactionInventory,
        mut deltas: Vec<ReactionDelta>,
    ) -> Self {
        deltas.sort_by_key(|d| {
            (
                d.location.key(),
                d.source.key(),
                d.reaction_id.raw(),
                d.inputs
                    .first()
                    .map(|t| t.sort_key())
                    .unwrap_or((u8::MAX, u32::MAX)),
            )
        });
        let mut accepted = Vec::new();
        let mut rejected = BTreeMap::new();
        let mut heat = BTreeMap::new();
        let mut energy = BTreeMap::new();
        for delta in deltas {
            let input_total: f32 = delta.inputs.iter().map(|t| t.amount()).sum();
            let destination_total: f32 = delta.destinations.iter().map(|d| d.term().amount()).sum();
            let reason = if delta.energy_output.raw() > 0.0 && delta.mode == ReactionMode::Passive {
                Some(AccountingRejection::PassiveEnergyCreditForbidden)
            } else if delta.energy_output.raw() > 0.0
                && delta.mode == ReactionMode::Controlled
                && delta.controlled_energy_token.is_none()
            {
                Some(AccountingRejection::ControlledEnergyNotAllowed)
            } else if delta.inputs.is_empty() && delta.destinations.iter().any(|d| d.produces()) {
                Some(AccountingRejection::ProductsRequireInputs)
            } else if (input_total - destination_total).abs() > 1e-5 {
                Some(AccountingRejection::UnaccountedInput)
            } else if delta
                .inputs
                .iter()
                .any(|t| inventory.available(delta.location, *t) + 1e-6 < t.amount())
            {
                Some(AccountingRejection::InsufficientInput)
            } else {
                None
            };
            if let Some(reason) = reason {
                rejected.insert(delta.reaction_id, reason);
                continue;
            }
            for term in &delta.inputs {
                inventory.add(delta.location, *term, -1.0);
            }
            for dest in &delta.destinations {
                if dest.produces() {
                    inventory.add(delta.location, dest.term(), 1.0);
                }
            }
            rejected.remove(&delta.reaction_id);
            let key = delta.location.key();
            let current_heat = heat.get(&key).copied().unwrap_or_else(HeatAmount::zero);
            heat.insert(key, current_heat.saturating_add(delta.heat_output));
            let current_energy = energy.get(&key).copied().unwrap_or_else(EnergyAmount::zero);
            energy.insert(key, current_energy.saturating_add(delta.energy_output));
            accepted.push(delta.reaction_id);
        }
        Self {
            inventory,
            accepted,
            rejected,
            heat,
            energy,
        }
    }
    pub fn inventory(&self) -> &ReactionInventory {
        &self.inventory
    }
    pub fn accepted_reaction_ids(&self) -> &[ReactionId] {
        &self.accepted
    }
    pub fn rejection(&self, id: ReactionId) -> Option<&AccountingRejection> {
        self.rejected.get(&id)
    }
    pub fn heat_at(&self, location: ReactionLocation) -> HeatAmount {
        self.heat
            .get(&location.key())
            .copied()
            .unwrap_or_else(HeatAmount::zero)
    }
    pub fn energy_at(&self, location: ReactionLocation) -> EnergyAmount {
        self.energy
            .get(&location.key())
            .copied()
            .unwrap_or_else(EnergyAmount::zero)
    }
}
