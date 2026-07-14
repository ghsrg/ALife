use crate::bootstrap::seed_domains::SplitMix64;
use crate::core::units::{Position, WorldSize};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct PlacedCell {
    pub ordinal: usize,
    pub position: Position,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellPlacementError {
    code: &'static str,
}

impl CellPlacementError {
    pub const fn code(&self) -> &'static str {
        self.code
    }
}

impl fmt::Display for CellPlacementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code)
    }
}

impl std::error::Error for CellPlacementError {}

pub fn explicit_positions(
    world_size: WorldSize,
    positions: Vec<Position>,
) -> Result<Vec<PlacedCell>, CellPlacementError> {
    if positions.iter().any(|position| {
        position.x() < 0.0
            || position.y() < 0.0
            || position.x() > world_size.width()
            || position.y() > world_size.height()
    }) {
        return Err(CellPlacementError {
            code: "BOOTSTRAP_CELL_OUT_OF_BOUNDS",
        });
    }
    Ok(positions
        .into_iter()
        .enumerate()
        .map(|(ordinal, position)| PlacedCell { ordinal, position })
        .collect())
}

pub fn grid_positions(
    world_size: WorldSize,
    count: usize,
    minimum_spacing: f32,
) -> Result<Vec<PlacedCell>, CellPlacementError> {
    if count == 0 || !minimum_spacing.is_finite() || minimum_spacing <= 0.0 {
        return Err(CellPlacementError {
            code: "BOOTSTRAP_CELL_PLACEMENT_IMPOSSIBLE",
        });
    }
    let columns = (count as f32).sqrt().ceil() as usize;
    let rows = count.div_ceil(columns);
    let usable_width = world_size.width();
    let usable_height = world_size.height();
    if columns > 1 && usable_width / (columns as f32 - 1.0) < minimum_spacing {
        return Err(CellPlacementError {
            code: "BOOTSTRAP_CELL_PLACEMENT_IMPOSSIBLE",
        });
    }
    if rows > 1 && usable_height / (rows as f32 - 1.0) < minimum_spacing {
        return Err(CellPlacementError {
            code: "BOOTSTRAP_CELL_PLACEMENT_IMPOSSIBLE",
        });
    }

    let mut placed = Vec::with_capacity(count);
    for ordinal in 0..count {
        let column = ordinal % columns;
        let row = ordinal / columns;
        let x = if columns == 1 {
            usable_width * 0.5
        } else {
            (usable_width / (columns as f32 - 1.0)) * column as f32
        };
        let y = if rows == 1 {
            usable_height * 0.5
        } else {
            (usable_height / (rows as f32 - 1.0)) * row as f32
        };
        placed.push(PlacedCell {
            ordinal,
            position: Position::new(x, y),
        });
    }
    Ok(placed)
}

pub fn near_resource_positions(
    world_size: WorldSize,
    anchor: Position,
    count: usize,
    radius: f32,
    rng: &mut SplitMix64,
) -> Result<Vec<PlacedCell>, CellPlacementError> {
    if count == 0 || !radius.is_finite() || radius < 0.0 {
        return Err(CellPlacementError {
            code: "BOOTSTRAP_CELL_PLACEMENT_IMPOSSIBLE",
        });
    }
    let mut placed = Vec::with_capacity(count);
    for ordinal in 0..count {
        let angle = rng.next_f32() * std::f32::consts::TAU;
        let distance = rng.next_f32() * radius;
        let x = (anchor.x() + angle.cos() * distance).clamp(0.0, world_size.width());
        let y = (anchor.y() + angle.sin() * distance).clamp(0.0, world_size.height());
        placed.push(PlacedCell {
            ordinal,
            position: Position::new(x, y),
        });
    }
    Ok(placed)
}
