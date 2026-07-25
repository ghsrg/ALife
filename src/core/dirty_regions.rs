use crate::core::cell_store::CellIndex;
use crate::core::units::{Position, WorldSize};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionId(pub usize);

#[derive(Clone, Debug, PartialEq)]
pub struct DirtyRegionTracker {
    tile_size: f32,
    grid_width: usize,
    grid_height: usize,
    dirty_tiles: HashSet<RegionId>,
    dirty_cells: Vec<CellIndex>,
}

impl DirtyRegionTracker {
    pub fn new(world_size: WorldSize, tile_size: f32) -> Self {
        let tile_size = if tile_size > 0.0 && tile_size.is_finite() {
            tile_size
        } else {
            64.0
        };

        let grid_width = ((world_size.width() / tile_size).ceil() as usize).max(1);
        let grid_height = ((world_size.height() / tile_size).ceil() as usize).max(1);

        Self {
            tile_size,
            grid_width,
            grid_height,
            dirty_tiles: HashSet::new(),
            dirty_cells: Vec::new(),
        }
    }

    pub fn tile_size(&self) -> f32 {
        self.tile_size
    }

    pub fn grid_width(&self) -> usize {
        self.grid_width
    }

    pub fn grid_height(&self) -> usize {
        self.grid_height
    }

    pub fn position_to_region(&self, pos: Position) -> RegionId {
        let tx =
            ((pos.x() / self.tile_size).floor() as usize).min(self.grid_width.saturating_sub(1));
        let ty =
            ((pos.y() / self.tile_size).floor() as usize).min(self.grid_height.saturating_sub(1));
        RegionId(ty * self.grid_width + tx)
    }

    pub fn mark_position_dirty(&mut self, pos: Position) {
        let region = self.position_to_region(pos);
        self.dirty_tiles.insert(region);
    }

    pub fn mark_cell_dirty(&mut self, cell_idx: CellIndex, pos: Position) {
        self.mark_position_dirty(pos);
        if !self.dirty_cells.contains(&cell_idx) {
            self.dirty_cells.push(cell_idx);
        }
    }

    pub fn is_position_dirty(&self, pos: Position) -> bool {
        let region = self.position_to_region(pos);
        self.dirty_tiles.contains(&region)
    }

    pub fn dirty_tiles_count(&self) -> usize {
        self.dirty_tiles.len()
    }

    pub fn dirty_cells_count(&self) -> usize {
        self.dirty_cells.len()
    }

    pub fn dirty_cells(&self) -> &[CellIndex] {
        &self.dirty_cells
    }

    pub fn clear_dirty_flags(&mut self) {
        self.dirty_tiles.clear();
        self.dirty_cells.clear();
    }
}
