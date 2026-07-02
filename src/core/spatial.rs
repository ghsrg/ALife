use crate::core::cell_store::{CellIndex, CellStore, LifecycleState};
use crate::core::units::WorldSize;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpatialIndex {
    rebuild_count: u64,
    sorted_cells: Vec<CellIndex>,
    grid_offsets: Vec<usize>,
    cols: usize,
    rows: usize,
    grid_size: f32,
}

impl SpatialIndex {
    pub fn new() -> Self {
        Self {
            rebuild_count: 0,
            sorted_cells: Vec::new(),
            grid_offsets: Vec::new(),
            cols: 0,
            rows: 0,
            grid_size: 1.0,
        }
    }

    pub fn rebuild_count(&self) -> u64 {
        self.rebuild_count
    }

    pub fn rebuild(&mut self, cells: &CellStore, world_size: WorldSize, grid_size: f32) {
        self.rebuild_count += 1;
        let cell_count = cells.len();
        self.sorted_cells.clear();
        self.sorted_cells.resize(cell_count, CellIndex::from_raw(0));

        let cols = (world_size.width() / grid_size).ceil() as usize;
        let rows = (world_size.height() / grid_size).ceil() as usize;
        let total_cells = cols * rows;
        self.cols = cols;
        self.rows = rows;
        self.grid_size = grid_size;

        self.grid_offsets.clear();
        self.grid_offsets.resize(total_cells + 1, 0);

        // 1. Count cells per grid cell
        let mut active_count = 0;
        for i in 0..cell_count {
            let idx = CellIndex::from_raw(i);
            if cells.lifecycle_state(idx) == LifecycleState::Dead {
                continue;
            }
            let pos = cells.position(idx);
            let cx = ((pos.x() / grid_size).floor() as usize).min(cols - 1);
            let cy = ((pos.y() / grid_size).floor() as usize).min(rows - 1);
            let grid_idx = cy * cols + cx;
            self.grid_offsets[grid_idx] += 1;
            active_count += 1;
        }

        self.sorted_cells.truncate(active_count);

        // 2. Prefix sum (offsets)
        let mut sum = 0;
        for i in 0..total_cells {
            let count = self.grid_offsets[i];
            self.grid_offsets[i] = sum;
            sum += count;
        }
        self.grid_offsets[total_cells] = sum;

        // 3. Populate sorted_cells
        let mut insertion_offsets = self.grid_offsets.clone();
        for i in 0..cell_count {
            let idx = CellIndex::from_raw(i);
            if cells.lifecycle_state(idx) == LifecycleState::Dead {
                continue;
            }
            let pos = cells.position(idx);
            let cx = ((pos.x() / grid_size).floor() as usize).min(cols - 1);
            let cy = ((pos.y() / grid_size).floor() as usize).min(rows - 1);
            let grid_idx = cy * cols + cx;
            let dest = insertion_offsets[grid_idx];
            self.sorted_cells[dest] = idx;
            insertion_offsets[grid_idx] += 1;
        }
    }

    pub fn generate_candidate_pairs(
        &self,
        _cells: &CellStore,
        pairs: &mut Vec<(CellIndex, CellIndex)>,
    ) {
        pairs.clear();
        if self.cols == 0 || self.rows == 0 {
            return;
        }

        let neighbors: [(isize, isize); 4] = [
            (1, 0),  // right
            (-1, 1), // bottom-left
            (0, 1),  // bottom
            (1, 1),  // bottom-right
        ];

        for cy in 0..self.rows {
            for cx in 0..self.cols {
                let grid_idx = cy * self.cols + cx;
                let start = self.grid_offsets[grid_idx];
                let end = self.grid_offsets[grid_idx + 1];

                // CellIndex in same grid cell
                for i in start..end {
                    let idx_i = self.sorted_cells[i];
                    for j in (i + 1)..end {
                        let idx_j = self.sorted_cells[j];
                        if idx_i.raw() < idx_j.raw() {
                            pairs.push((idx_i, idx_j));
                        } else {
                            pairs.push((idx_j, idx_i));
                        }
                    }

                    // Cells in adjacent grid cells
                    for &(dx, dy) in &neighbors {
                        let nx = cx as isize + dx;
                        let ny = cy as isize + dy;
                        if nx >= 0 && nx < self.cols as isize && ny >= 0 && ny < self.rows as isize
                        {
                            let neighbor_idx = (ny as usize) * self.cols + (nx as usize);
                            let n_start = self.grid_offsets[neighbor_idx];
                            let n_end = self.grid_offsets[neighbor_idx + 1];
                            for k in n_start..n_end {
                                let idx_k = self.sorted_cells[k];
                                if idx_i.raw() < idx_k.raw() {
                                    pairs.push((idx_i, idx_k));
                                } else {
                                    pairs.push((idx_k, idx_i));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Sort pairs to ensure stable, deterministic iteration order
        pairs.sort_unstable_by_key(|&(i, j)| (i.raw(), j.raw()));
    }
}
