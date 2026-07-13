use crate::core::ids::{MaterialFragmentId, MaterialTypeId};
use crate::core::units::{MaterialAmount, Position, Tick};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialFragment {
    material_type_id: MaterialTypeId,
    amount: MaterialAmount,
    position: Position,
    created_tick: Tick,
}

impl MaterialFragment {
    pub const fn new(
        material_type_id: MaterialTypeId,
        amount: MaterialAmount,
        position: Position,
        created_tick: Tick,
    ) -> Self {
        Self {
            material_type_id,
            amount,
            position,
            created_tick,
        }
    }
    pub const fn material_type_id(self) -> MaterialTypeId {
        self.material_type_id
    }
    pub const fn amount(self) -> MaterialAmount {
        self.amount
    }
    pub const fn position(self) -> Position {
        self.position
    }
    pub const fn created_tick(self) -> Tick {
        self.created_tick
    }
}

#[derive(Clone, Debug, Default)]
pub struct FragmentStore {
    fragments: Vec<MaterialFragment>,
}

impl FragmentStore {
    pub fn create(&mut self, fragment: MaterialFragment) -> MaterialFragmentId {
        let id = MaterialFragmentId::from_raw(self.fragments.len() as u32);
        self.fragments.push(fragment);
        id
    }
    pub fn get(&self, id: MaterialFragmentId) -> Option<&MaterialFragment> {
        self.fragments.get(id.raw() as usize)
    }
    pub fn iter(&self) -> impl Iterator<Item = (MaterialFragmentId, &MaterialFragment)> {
        self.fragments
            .iter()
            .enumerate()
            .map(|(index, fragment)| (MaterialFragmentId::from_raw(index as u32), fragment))
    }
    pub fn total_amount(&self) -> MaterialAmount {
        MaterialAmount::new(
            self.fragments
                .iter()
                .map(|fragment| fragment.amount.raw())
                .sum(),
        )
        .expect("fragment amounts are validated")
    }
}
