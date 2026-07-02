#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SpatialIndex {
    rebuild_count: u64,
}

impl SpatialIndex {
    pub fn new() -> Self {
        Self { rebuild_count: 0 }
    }

    pub fn rebuild(&mut self) {
        self.rebuild_count += 1;
    }

    pub fn rebuild_count(&self) -> u64 {
        self.rebuild_count
    }
}
