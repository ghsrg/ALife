use alife::bootstrap::cell_placement::{
    explicit_positions, grid_positions, near_resource_positions,
};
use alife::bootstrap::seed_domains::SplitMix64;
use alife::core::units::{Position, WorldSize};

#[test]
fn explicit_placement_preserves_positions_and_ordinals() {
    let world = WorldSize::new(10.0, 10.0).unwrap();
    let placed = explicit_positions(
        world,
        vec![Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
    )
    .unwrap();

    assert_eq!(placed[0].ordinal, 0);
    assert_eq!(placed[0].position, Position::new(1.0, 2.0));
    assert_eq!(placed[1].ordinal, 1);
    assert_eq!(placed[1].position, Position::new(3.0, 4.0));
}

#[test]
fn grid_placement_respects_minimum_spacing() {
    let world = WorldSize::new(10.0, 10.0).unwrap();
    let placed = grid_positions(world, 4, 2.0).unwrap();

    assert_eq!(placed.len(), 4);
    for a in 0..placed.len() {
        for b in (a + 1)..placed.len() {
            let dx = placed[a].position.x() - placed[b].position.x();
            let dy = placed[a].position.y() - placed[b].position.y();
            assert!((dx * dx + dy * dy).sqrt() >= 2.0);
        }
    }
}

#[test]
fn near_resource_placement_is_deterministic() {
    let world = WorldSize::new(10.0, 10.0).unwrap();
    let mut a_rng = SplitMix64::new(777);
    let mut b_rng = SplitMix64::new(777);

    let a = near_resource_positions(world, Position::new(5.0, 5.0), 3, 1.0, &mut a_rng).unwrap();
    let b = near_resource_positions(world, Position::new(5.0, 5.0), 3, 1.0, &mut b_rng).unwrap();

    assert_eq!(a, b);
}

#[test]
fn impossible_grid_placement_returns_stable_error() {
    let world = WorldSize::new(2.0, 2.0).unwrap();
    let err = grid_positions(world, 4, 3.0).unwrap_err();

    assert_eq!(err.code(), "BOOTSTRAP_CELL_PLACEMENT_IMPOSSIBLE");
}
