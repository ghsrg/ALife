use crate::core::ids::FieldTypeId;
use crate::core::units::{FieldValue, GridCoord, Position, WorldSize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldLayerIndex(usize);

impl FieldLayerIndex {
    pub const fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldGridError {
    EmptyInitialDistribution,
    InvalidGridSize,
    InvalidBounds,
    GridCoordOutOfBounds,
    LayerOutOfBounds,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldKind {
    Scalar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldEffectProfile {
    Temperature,
    Light,
    Pressure,
    Radiation,
    ChemicalGradient,
    Flow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldConservedBehavior {
    Conserved,
    Dissipated,
    Clamped,
    Derived,
    Abstracted,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldRuntimeConfig {
    pub id: String,
    pub type_id: FieldTypeId,
    pub kind: FieldKind,
    pub initial_value: FieldValue,
    pub diffusion_rate: f32,
    pub decay_rate: f32,
    pub min_value: FieldValue,
    pub max_value: FieldValue,
    pub effect_profile: FieldEffectProfile,
    pub conserved_behavior: FieldConservedBehavior,
}

impl FieldRuntimeConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        type_id: FieldTypeId,
        kind: FieldKind,
        initial_value: FieldValue,
        diffusion_rate: f32,
        decay_rate: f32,
        min_value: FieldValue,
        max_value: FieldValue,
        effect_profile: FieldEffectProfile,
        conserved_behavior: FieldConservedBehavior,
    ) -> Result<Self, FieldGridError> {
        if min_value.raw() > max_value.raw()
            || initial_value.raw() < min_value.raw()
            || initial_value.raw() > max_value.raw()
            || !diffusion_rate.is_finite()
            || !decay_rate.is_finite()
            || diffusion_rate < 0.0
            || decay_rate < 0.0
        {
            return Err(FieldGridError::InvalidBounds);
        }
        Ok(Self {
            id,
            type_id,
            kind,
            initial_value,
            diffusion_rate,
            decay_rate,
            min_value,
            max_value,
            effect_profile,
            conserved_behavior,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldGrid {
    width: usize,
    height: usize,
    layer_count: usize,
    values: Vec<FieldValue>,
    type_ids: Vec<FieldTypeId>,
    min_values: Vec<f32>,
    max_values: Vec<f32>,
    diffusion_rates: Vec<f32>,
    decay_rates: Vec<f32>,
    spatial_grid_size: f32,
}

impl FieldGrid {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        world_size: WorldSize,
        spatial_grid_size: f32,
        initial_distribution: Vec<(FieldTypeId, FieldValue)>,
        min_value: f32,
        max_value: f32,
        diffusion_rate: f32,
        decay_rate: f32,
    ) -> Result<Self, FieldGridError> {
        let layer_count = initial_distribution.len();
        Self::new_with_layer_configs(
            world_size,
            spatial_grid_size,
            initial_distribution,
            vec![min_value; layer_count],
            vec![max_value; layer_count],
            vec![diffusion_rate; layer_count],
            vec![decay_rate; layer_count],
        )
    }

    fn new_with_layer_configs(
        world_size: WorldSize,
        spatial_grid_size: f32,
        initial_distribution: Vec<(FieldTypeId, FieldValue)>,
        min_values: Vec<f32>,
        max_values: Vec<f32>,
        diffusion_rates: Vec<f32>,
        decay_rates: Vec<f32>,
    ) -> Result<Self, FieldGridError> {
        let layer_count = initial_distribution.len();
        let width = grid_axis(world_size.width(), spatial_grid_size)?;
        let height = grid_axis(world_size.height(), spatial_grid_size)?;
        let cell_count = width * height;
        let initial_layers = initial_distribution
            .into_iter()
            .map(|(type_id, value)| (type_id, vec![value; cell_count]))
            .collect::<Vec<_>>();
        Self::new_with_layer_values(
            width,
            height,
            spatial_grid_size,
            initial_layers,
            min_values,
            max_values,
            diffusion_rates,
            decay_rates,
        )
        .and_then(|grid| {
            if grid.layer_count == layer_count {
                Ok(grid)
            } else {
                Err(FieldGridError::InvalidBounds)
            }
        })
    }

    fn new_with_layer_values(
        width: usize,
        height: usize,
        spatial_grid_size: f32,
        initial_layers: Vec<(FieldTypeId, Vec<FieldValue>)>,
        min_values: Vec<f32>,
        max_values: Vec<f32>,
        diffusion_rates: Vec<f32>,
        decay_rates: Vec<f32>,
    ) -> Result<Self, FieldGridError> {
        if initial_layers.is_empty() {
            return Err(FieldGridError::EmptyInitialDistribution);
        }
        if !spatial_grid_size.is_finite() || spatial_grid_size <= 0.0 {
            return Err(FieldGridError::InvalidGridSize);
        }
        let layer_count = initial_layers.len();
        if min_values.len() != layer_count
            || max_values.len() != layer_count
            || diffusion_rates.len() != layer_count
            || decay_rates.len() != layer_count
        {
            return Err(FieldGridError::InvalidBounds);
        }

        let cell_count = width * height;
        let mut type_ids = Vec::with_capacity(layer_count);
        let mut values = Vec::with_capacity(layer_count * cell_count);

        for (layer, (type_id, layer_values)) in initial_layers.into_iter().enumerate() {
            if layer_values.len() != cell_count {
                return Err(FieldGridError::InvalidBounds);
            }
            let min_value = min_values[layer];
            let max_value = max_values[layer];
            let diffusion_rate = diffusion_rates[layer];
            let decay_rate = decay_rates[layer];
            if !min_value.is_finite()
                || !max_value.is_finite()
                || min_value > max_value
                || !diffusion_rate.is_finite()
                || !decay_rate.is_finite()
                || diffusion_rate < 0.0
                || decay_rate < 0.0
            {
                return Err(FieldGridError::InvalidBounds);
            }
            type_ids.push(type_id);
            for value in layer_values {
                let clamped = FieldValue::new(value.raw().clamp(min_value, max_value))
                    .map_err(|_| FieldGridError::InvalidBounds)?;
                values.push(clamped);
            }
        }

        Ok(Self {
            width,
            height,
            layer_count,
            values,
            type_ids,
            min_values,
            max_values,
            diffusion_rates,
            decay_rates,
            spatial_grid_size,
        })
    }

    pub fn from_configs(
        world_size: WorldSize,
        spatial_grid_size: f32,
        configs: &[FieldRuntimeConfig],
    ) -> Result<Option<Self>, FieldGridError> {
        if configs.is_empty() {
            return Ok(None);
        }
        Self::new_with_layer_configs(
            world_size,
            spatial_grid_size,
            configs
                .iter()
                .map(|config| (config.type_id, config.initial_value))
                .collect(),
            configs
                .iter()
                .map(|config| config.min_value.raw())
                .collect(),
            configs
                .iter()
                .map(|config| config.max_value.raw())
                .collect(),
            configs.iter().map(|config| config.diffusion_rate).collect(),
            configs.iter().map(|config| config.decay_rate).collect(),
        )
        .map(Some)
    }

    pub fn from_configs_with_layers(
        world_size: WorldSize,
        spatial_grid_size: f32,
        configs: &[FieldRuntimeConfig],
        prepared_layers: Option<Vec<Vec<FieldValue>>>,
    ) -> Result<Option<Self>, FieldGridError> {
        let Some(prepared_layers) = prepared_layers else {
            return Self::from_configs(world_size, spatial_grid_size, configs);
        };
        if configs.is_empty() {
            return Ok(None);
        }
        if prepared_layers.len() != configs.len() {
            return Err(FieldGridError::InvalidBounds);
        }
        Self::new_with_layer_values(
            grid_axis(world_size.width(), spatial_grid_size)?,
            grid_axis(world_size.height(), spatial_grid_size)?,
            spatial_grid_size,
            configs
                .iter()
                .zip(prepared_layers)
                .map(|(config, values)| (config.type_id, values))
                .collect(),
            configs
                .iter()
                .map(|config| config.min_value.raw())
                .collect(),
            configs
                .iter()
                .map(|config| config.max_value.raw())
                .collect(),
            configs.iter().map(|config| config.diffusion_rate).collect(),
            configs.iter().map(|config| config.decay_rate).collect(),
        )
        .map(Some)
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

    pub fn values(&self) -> &[FieldValue] {
        &self.values
    }

    pub fn coord_for_position(&self, position: Position) -> GridCoord {
        let max_x = self.width.saturating_sub(1);
        let max_y = self.height.saturating_sub(1);
        let x = (position.x() / self.spatial_grid_size).floor().max(0.0) as usize;
        let y = (position.y() / self.spatial_grid_size).floor().max(0.0) as usize;

        GridCoord::new(x.min(max_x), y.min(max_y))
    }

    pub fn sample_at_position(
        &self,
        field_type: FieldTypeId,
        position: Position,
    ) -> Result<FieldValue, FieldGridError> {
        let layer = self.layer_for_type(field_type)?;
        self.value_at(layer, self.coord_for_position(position))
    }

    pub fn value_at(
        &self,
        layer: FieldLayerIndex,
        coord: GridCoord,
    ) -> Result<FieldValue, FieldGridError> {
        let index = self.index(layer, coord)?;
        Ok(self.values[index])
    }

    pub fn set_value_at(
        &mut self,
        layer: FieldLayerIndex,
        coord: GridCoord,
        value: FieldValue,
    ) -> Result<(), FieldGridError> {
        let index = self.index(layer, coord)?;
        let min_value = self.min_values[layer.raw()];
        let max_value = self.max_values[layer.raw()];
        self.values[index] = FieldValue::new(value.raw().clamp(min_value, max_value))
            .map_err(|_| FieldGridError::InvalidBounds)?;
        Ok(())
    }

    pub fn decay_elapsed(&mut self, elapsed_ticks: u64) {
        let cell_count = self.width * self.height;
        for layer in 0..self.layer_count {
            let decay_factor =
                (1.0 - self.decay_rates[layer].clamp(0.0, 1.0)).powi(elapsed_ticks.max(1) as i32);
            let start = layer * cell_count;
            let min_value = self.min_values[layer];
            let max_value = self.max_values[layer];
            for value in &mut self.values[start..start + cell_count] {
                let next = (value.raw() * decay_factor).clamp(min_value, max_value);
                *value = FieldValue::new(next).unwrap_or_else(|_| FieldValue::zero());
            }
        }
    }

    pub fn diffuse_all(&mut self) -> Result<(), FieldGridError> {
        for layer in 0..self.layer_count {
            self.diffuse_layer(FieldLayerIndex::from_raw(layer))?;
        }
        Ok(())
    }

    pub fn diffuse_layer(&mut self, layer: FieldLayerIndex) -> Result<(), FieldGridError> {
        if layer.raw() >= self.layer_count {
            return Err(FieldGridError::LayerOutOfBounds);
        }
        let rate = self.diffusion_rates[layer.raw()].clamp(0.0, 1.0);
        if rate <= 0.0 {
            return Ok(());
        }

        let cell_count = self.width * self.height;
        let start = layer.raw() * cell_count;
        let min_value = self.min_values[layer.raw()];
        let max_value = self.max_values[layer.raw()];
        let current = self.values[start..start + cell_count]
            .iter()
            .map(|value| value.raw())
            .collect::<Vec<_>>();
        let mut delta = vec![0.0_f32; cell_count];

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if x + 1 < self.width {
                    diffuse_pair(&current, &mut delta, idx, y * self.width + x + 1, rate);
                }
                if y + 1 < self.height {
                    diffuse_pair(&current, &mut delta, idx, (y + 1) * self.width + x, rate);
                }
            }
        }

        for (offset, change) in delta.into_iter().enumerate() {
            let next = (current[offset] + change).clamp(min_value, max_value);
            self.values[start + offset] =
                FieldValue::new(next).map_err(|_| FieldGridError::InvalidBounds)?;
        }
        Ok(())
    }

    fn layer_for_type(&self, field_type: FieldTypeId) -> Result<FieldLayerIndex, FieldGridError> {
        self.type_ids
            .iter()
            .position(|id| *id == field_type)
            .map(FieldLayerIndex::from_raw)
            .ok_or(FieldGridError::LayerOutOfBounds)
    }

    fn index(&self, layer: FieldLayerIndex, coord: GridCoord) -> Result<usize, FieldGridError> {
        if layer.raw() >= self.layer_count {
            return Err(FieldGridError::LayerOutOfBounds);
        }
        if coord.x() >= self.width || coord.y() >= self.height {
            return Err(FieldGridError::GridCoordOutOfBounds);
        }
        Ok(layer.raw() * self.width * self.height + coord.y() * self.width + coord.x())
    }
}

fn diffuse_pair(current: &[f32], delta: &mut [f32], a: usize, b: usize, rate: f32) {
    let gradient = current[a] - current[b];
    if gradient.abs() <= f32::EPSILON {
        return;
    }
    let moved = gradient.abs() * rate * 0.5;
    if gradient > 0.0 {
        delta[a] -= moved;
        delta[b] += moved;
    } else {
        delta[a] += moved;
        delta[b] -= moved;
    }
}

fn grid_axis(size: f32, spatial_grid_size: f32) -> Result<usize, FieldGridError> {
    if !spatial_grid_size.is_finite() || spatial_grid_size <= 0.0 {
        return Err(FieldGridError::InvalidGridSize);
    }
    Ok((size / spatial_grid_size).ceil().max(1.0) as usize)
}
