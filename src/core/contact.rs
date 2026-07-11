use crate::core::cell_store::{CellIndex, CellStore};
use crate::core::spatial::SpatialIndex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContactPair {
    pub a: CellIndex,
    pub b: CellIndex,
    pub overlap: f32,
    pub normal_x_from_b_to_a: f32,
    pub normal_y_from_b_to_a: f32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContactCache {
    pairs: Vec<ContactPair>,
    total_overlap: f32,
    max_overlap: f32,
}

impl ContactCache {
    pub fn rebuild(&mut self, cells: &CellStore, spatial_index: &SpatialIndex) {
        self.pairs.clear();
        self.total_overlap = 0.0;
        self.max_overlap = 0.0;

        let mut candidates = Vec::new();
        spatial_index.generate_candidate_pairs(cells, &mut candidates);

        for (a, b) in candidates {
            let pos_a = cells.position(a);
            let pos_b = cells.position(b);
            let dx = pos_a.x() - pos_b.x();
            let dy = pos_a.y() - pos_b.y();
            let dist_sq = dx * dx + dy * dy;
            let target_dist = cells.radius(a).raw() + cells.radius(b).raw();

            if dist_sq >= target_dist * target_dist {
                continue;
            }

            let dist = dist_sq.sqrt();
            let overlap = target_dist - dist;
            let (normal_x, normal_y) = if dist > 0.0 {
                (dx / dist, dy / dist)
            } else if a.raw() < b.raw() {
                (1.0, 0.0)
            } else {
                (-1.0, 0.0)
            };

            self.total_overlap += overlap;
            self.max_overlap = self.max_overlap.max(overlap);
            self.pairs.push(ContactPair {
                a,
                b,
                overlap,
                normal_x_from_b_to_a: normal_x,
                normal_y_from_b_to_a: normal_y,
            });
        }

        self.pairs
            .sort_unstable_by_key(|pair| (pair.a.raw(), pair.b.raw()));
    }

    pub fn pairs(&self) -> &[ContactPair] {
        &self.pairs
    }

    pub fn total_overlap(&self) -> f32 {
        self.total_overlap
    }

    pub fn max_overlap(&self) -> f32 {
        self.max_overlap
    }
}
