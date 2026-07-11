use alife::core::{
    cell_store::{CellIndex, CellStore, EnergyBuffer, InitialCellState},
    contact::ContactCache,
    spatial::SpatialIndex,
    units::{
        CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, WorldSize,
    },
};

fn cell_at(x: f32, y: f32, radius: f32) -> InitialCellState {
    InitialCellState {
        position: Position::new(x, y),
        radius: Radius::new(radius).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(10.0).unwrap(),
            EnergyAmount::new(20.0).unwrap(),
        ),
        resources: ResourceAmount::zero(),
        boundary_material: MaterialAmount::new(1.0).unwrap(),
        transport_material: MaterialAmount::new(1.0).unwrap(),
        metabolic_material: MaterialAmount::zero(),
        storage_material: MaterialAmount::zero(),
        synthesis_material: MaterialAmount::zero(),
        structural_material: MaterialAmount::new(1.0).unwrap(),
        repair_material: MaterialAmount::zero(),
        contractile_material: MaterialAmount::zero(),
        sensory_material: MaterialAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        temperature: alife::core::units::Temperature::new(25.0),
    }
}

#[test]
fn contact_cache_records_only_overlapping_pairs_in_stable_order() {
    let mut cells = CellStore::with_capacity(4);
    cells.insert_initial(cell_at(10.0, 10.0, 2.0));
    cells.insert_initial(cell_at(13.0, 10.0, 2.0));
    cells.insert_initial(cell_at(16.0, 10.0, 2.0));
    cells.insert_initial(cell_at(40.0, 40.0, 1.0));

    let mut spatial = SpatialIndex::new();
    spatial.rebuild(&cells, WorldSize::new(64.0, 64.0).unwrap(), 8.0);

    let mut cache = ContactCache::default();
    cache.rebuild(&cells, &spatial);

    let pairs: Vec<_> = cache
        .pairs()
        .iter()
        .map(|pair| (pair.a.raw(), pair.b.raw(), pair.overlap))
        .collect();

    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].0, CellIndex::from_raw(0).raw());
    assert_eq!(pairs[0].1, CellIndex::from_raw(1).raw());
    assert_eq!(pairs[1].0, CellIndex::from_raw(1).raw());
    assert_eq!(pairs[1].1, CellIndex::from_raw(2).raw());
    assert!(pairs[0].2 > 0.9 && pairs[0].2 < 1.1);
    assert!(cache.total_overlap() > 1.9 && cache.total_overlap() < 2.1);
    assert!(cache.max_overlap() > 0.9 && cache.max_overlap() < 1.1);
}
