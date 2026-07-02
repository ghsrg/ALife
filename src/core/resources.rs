use crate::core::units::{GridCoord, Position, ResourceAmount, WorldSize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceLayerIndex(usize);

impl ResourceLayerIndex {
    pub const fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceGridError {
    EmptyInitialDistribution,
    InvalidGridSize,
    InvalidDecayRate,
    GridCoordOutOfBounds,
    LayerOutOfBounds,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceGrid {
    width: usize,
    height: usize,
    layer_count: usize,
    quantities: Vec<ResourceAmount>,
    optional_decay_rate: f32,
    spatial_grid_size: f32,
}

impl ResourceGrid {
    pub fn new(
        world_size: WorldSize,
        spatial_grid_size: f32,
        initial_distribution: Vec<ResourceAmount>,
        decay_rate: f32,
    ) -> Result<Self, ResourceGridError> {
        if initial_distribution.is_empty() {
            return Err(ResourceGridError::EmptyInitialDistribution);
        }
        if !spatial_grid_size.is_finite() || spatial_grid_size <= 0.0 {
            return Err(ResourceGridError::InvalidGridSize);
        }
        if !decay_rate.is_finite() || !(0.0..=1.0).contains(&decay_rate) {
            return Err(ResourceGridError::InvalidDecayRate);
        }

        let width = (world_size.width() / spatial_grid_size).ceil().max(1.0) as usize;
        let height = (world_size.height() / spatial_grid_size).ceil().max(1.0) as usize;
        let layer_count = initial_distribution.len();
        let cell_count = width * height;
        let mut quantities = Vec::with_capacity(layer_count * cell_count);

        for amount in initial_distribution {
            for _ in 0..cell_count {
                quantities.push(amount);
            }
        }

        Ok(Self {
            width,
            height,
            layer_count,
            quantities,
            optional_decay_rate: decay_rate,
            spatial_grid_size,
        })
    }

    pub fn coord_for_position(&self, position: Position) -> GridCoord {
        let max_x = self.width.saturating_sub(1);
        let max_y = self.height.saturating_sub(1);
        let x = (position.x() / self.spatial_grid_size).floor().max(0.0) as usize;
        let y = (position.y() / self.spatial_grid_size).floor().max(0.0) as usize;

        GridCoord::new(x.min(max_x), y.min(max_y))
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub const fn layer_count(&self) -> usize {
        self.layer_count
    }

    pub const fn cell_count(&self) -> usize {
        self.width * self.height
    }

    pub fn amount_at(
        &self,
        layer: ResourceLayerIndex,
        coord: GridCoord,
    ) -> Result<ResourceAmount, ResourceGridError> {
        let index = self.index(layer, coord)?;
        Ok(self.quantities[index])
    }

    pub fn set_amount_at(
        &mut self,
        layer: ResourceLayerIndex,
        coord: GridCoord,
        amount: ResourceAmount,
    ) -> Result<(), ResourceGridError> {
        let index = self.index(layer, coord)?;
        self.quantities[index] = amount;
        Ok(())
    }

    pub fn total_amount_for_layer(
        &self,
        layer: ResourceLayerIndex,
    ) -> Result<ResourceAmount, ResourceGridError> {
        if layer.raw() >= self.layer_count {
            return Err(ResourceGridError::LayerOutOfBounds);
        }
        let start = layer.raw() * self.cell_count();
        let end = start + self.cell_count();
        let total: f32 = self.quantities[start..end]
            .iter()
            .map(|amount| amount.raw())
            .sum();
        ResourceAmount::new(total).map_err(|_| ResourceGridError::InvalidGridSize)
    }

    pub fn decay_or_passive_update(&mut self) {
        for amount in &mut self.quantities {
            let next_value = (amount.raw() * (1.0 - self.optional_decay_rate)).max(0.0);
            *amount = ResourceAmount::new(next_value).unwrap_or_else(|_| ResourceAmount::zero());
        }
    }

    pub fn quantities(&self) -> &[ResourceAmount] {
        &self.quantities
    }

    fn index(
        &self,
        layer: ResourceLayerIndex,
        coord: GridCoord,
    ) -> Result<usize, ResourceGridError> {
        if layer.raw() >= self.layer_count {
            return Err(ResourceGridError::LayerOutOfBounds);
        }
        if coord.x() >= self.width || coord.y() >= self.height {
            return Err(ResourceGridError::GridCoordOutOfBounds);
        }

        Ok(layer.raw() * self.cell_count() + coord.y() * self.width + coord.x())
    }
}
