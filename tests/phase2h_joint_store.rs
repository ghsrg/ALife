use alife::core::cell_store::CellIndex;
use alife::core::ids::JointId;
use alife::core::joints::{JointChannelConfig, JointEndpoints, JointStore};
use alife::core::units::{MaterialAmount, Tick};

#[test]
fn joint_store_allocates_stable_ids_and_orders_endpoints() {
    let mut joints = JointStore::with_capacity(4);
    let cfg = JointChannelConfig::mechanical_only(1.0);

    let first = joints.create(
        JointEndpoints::new(CellIndex::from_raw(3), CellIndex::from_raw(1)).unwrap(),
        MaterialAmount::new(2.0).unwrap(),
        cfg,
        Tick::from_raw(7),
    );
    let second = joints.create(
        JointEndpoints::new(CellIndex::from_raw(2), CellIndex::from_raw(1)).unwrap(),
        MaterialAmount::new(1.0).unwrap(),
        cfg,
        Tick::from_raw(8),
    );

    assert_eq!(first, JointId::from_raw(0));
    assert_eq!(second, JointId::from_raw(1));
    assert_eq!(joints.endpoints(first).unwrap().a, CellIndex::from_raw(1));
    assert_eq!(joints.endpoints(first).unwrap().b, CellIndex::from_raw(3));
    assert_eq!(joints.active_ids().collect::<Vec<_>>(), vec![first, second]);
}

#[test]
fn joint_store_rejects_self_endpoint_and_keeps_broken_joint_material() {
    assert!(JointEndpoints::new(CellIndex::from_raw(2), CellIndex::from_raw(2)).is_none());

    let mut joints = JointStore::with_capacity(1);
    let id = joints.create(
        JointEndpoints::new(CellIndex::from_raw(0), CellIndex::from_raw(1)).unwrap(),
        MaterialAmount::new(3.0).unwrap(),
        JointChannelConfig::mechanical_only(1.0),
        Tick::from_raw(0),
    );

    joints.break_joint(id, Tick::from_raw(3)).unwrap();

    assert!(!joints.is_active(id).unwrap());
    assert!(joints.is_broken(id).unwrap());
    assert_eq!(joints.material_amount(id).unwrap().raw(), 3.0);
}
