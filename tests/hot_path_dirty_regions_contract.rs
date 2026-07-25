use alife::core::cell_store::CellIndex;
use alife::core::dirty_regions::DirtyRegionTracker;
use alife::core::units::{Position, WorldSize};

#[test]
fn test_dirty_region_tracker_mapping_and_flags() {
    let world_size = WorldSize::new(1000.0, 1000.0).unwrap();
    let mut tracker = DirtyRegionTracker::new(world_size, 64.0);

    assert_eq!(tracker.tile_size(), 64.0);
    assert_eq!(tracker.grid_width(), 16);
    assert_eq!(tracker.grid_height(), 16);

    let pos1 = Position::new(10.0, 10.0);
    let pos2 = Position::new(500.0, 500.0);

    assert!(!tracker.is_position_dirty(pos1));
    assert!(!tracker.is_position_dirty(pos2));

    let cell1 = CellIndex::from_raw(0);
    tracker.mark_cell_dirty(cell1, pos1);

    assert!(tracker.is_position_dirty(pos1));
    assert!(!tracker.is_position_dirty(pos2));
    assert_eq!(tracker.dirty_tiles_count(), 1);
    assert_eq!(tracker.dirty_cells_count(), 1);

    tracker.clear_dirty_flags();
    assert!(!tracker.is_position_dirty(pos1));
    assert_eq!(tracker.dirty_tiles_count(), 0);
    assert_eq!(tracker.dirty_cells_count(), 0);
}
