use alife::core::cell_store::{CellIndex, CellStore, EnergyBuffer, InitialCellState};
use alife::core::ids::ResourceTypeId;
use alife::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
};

fn initial_cell() -> InitialCellState {
    InitialCellState {
        position: Position::new(0.0, 0.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(EnergyAmount::zero(), EnergyAmount::new(1.0).unwrap()),
        resources: ResourceAmount::zero(),
        boundary_material: MaterialAmount::zero(),
        transport_material: MaterialAmount::zero(),
        metabolic_material: MaterialAmount::zero(),
        storage_material: MaterialAmount::zero(),
        synthesis_material: MaterialAmount::zero(),
        structural_material: MaterialAmount::zero(),
        repair_material: MaterialAmount::zero(),
        contractile_material: MaterialAmount::zero(),
        sensory_material: MaterialAmount::zero(),
        capacity_limit: CapacityAmount::new(10.0).unwrap(),
        temperature: Temperature::new(25.0),
    }
}

#[test]
fn typed_internal_resources_preserve_identity_and_contribute_to_capacity() {
    let mut cells = CellStore::with_capacity(1);
    cells
        .configure_typed_resource_types(vec![
            ResourceTypeId::from_raw(2),
            ResourceTypeId::from_raw(7),
        ])
        .unwrap();
    cells.insert_initial(initial_cell());
    let index = CellIndex::from_raw(0);

    cells
        .set_typed_resource_amount(
            index,
            ResourceTypeId::from_raw(2),
            ResourceAmount::new(3.0).unwrap(),
        )
        .unwrap();
    cells
        .set_typed_resource_amount(
            index,
            ResourceTypeId::from_raw(7),
            ResourceAmount::new(1.5).unwrap(),
        )
        .unwrap();

    assert_eq!(
        cells
            .typed_resource_amount(index, ResourceTypeId::from_raw(2))
            .unwrap(),
        ResourceAmount::new(3.0).unwrap()
    );
    assert_eq!(
        cells
            .typed_resource_amount(index, ResourceTypeId::from_raw(7))
            .unwrap(),
        ResourceAmount::new(1.5).unwrap()
    );
    assert_eq!(
        cells.resource_amount(index),
        ResourceAmount::new(4.5).unwrap()
    );
    assert_eq!(cells.used_capacity(index).raw(), 4.5);
}
