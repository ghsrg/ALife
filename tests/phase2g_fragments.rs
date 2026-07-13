use alife::core::fragments::{FragmentStore, MaterialFragment};
use alife::core::ids::MaterialTypeId;
use alife::core::units::{MaterialAmount, Position, Tick};

#[test]
fn material_fragment_preserves_identity_and_does_not_become_resource_implicitly() {
    let mut fragments = FragmentStore::default();
    let id = fragments.create(MaterialFragment::new(
        MaterialTypeId::from_raw(4),
        MaterialAmount::new(2.0).unwrap(),
        Position::new(3.0, 5.0),
        Tick::from_raw(7),
    ));

    let fragment = fragments.get(id).unwrap();
    assert_eq!(fragment.material_type_id(), MaterialTypeId::from_raw(4));
    assert_eq!(fragment.amount(), MaterialAmount::new(2.0).unwrap());
    assert_eq!(fragments.total_amount(), MaterialAmount::new(2.0).unwrap());
}
