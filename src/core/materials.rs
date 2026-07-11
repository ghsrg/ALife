use crate::core::units::MaterialAmount;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaterialSlot {
    Boundary,
    Transport,
    Metabolic,
    Storage,
    Synthesis,
    Structural,
    Repair,
    Contractile,
    Sensory,
}

impl MaterialSlot {
    pub const ALL: [Self; 9] = [
        Self::Boundary,
        Self::Transport,
        Self::Metabolic,
        Self::Storage,
        Self::Synthesis,
        Self::Structural,
        Self::Repair,
        Self::Contractile,
        Self::Sensory,
    ];

    const fn index(self) -> usize {
        match self {
            Self::Boundary => 0,
            Self::Transport => 1,
            Self::Metabolic => 2,
            Self::Storage => 3,
            Self::Synthesis => 4,
            Self::Structural => 5,
            Self::Repair => 6,
            Self::Contractile => 7,
            Self::Sensory => 8,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialComposition {
    amounts: [MaterialAmount; 9],
}

impl MaterialComposition {
    pub fn from_slots(slots: [(MaterialSlot, MaterialAmount); 9]) -> Self {
        let mut amounts = [MaterialAmount::zero(); 9];
        for (slot, amount) in slots {
            amounts[slot.index()] = amount;
        }
        Self { amounts }
    }

    pub fn amount(self, slot: MaterialSlot) -> MaterialAmount {
        self.amounts[slot.index()]
    }

    pub fn total(self) -> MaterialAmount {
        let total = self.amounts.iter().map(|amount| amount.raw()).sum();
        MaterialAmount::new_unchecked(total)
    }

    pub fn fraction(self, slot: MaterialSlot) -> f32 {
        let total = self.total().raw();
        if total <= 0.0 {
            return 0.0;
        }
        self.amount(slot).raw() / total
    }
}
