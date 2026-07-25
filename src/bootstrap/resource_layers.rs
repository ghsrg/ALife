use crate::bootstrap::manifest::ResourceLayerSummary;
use crate::bootstrap::seed_domains::SplitMix64;
use crate::core::units::ResourceAmount;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct PreparedResourceLayer {
    pub quantities: Vec<ResourceAmount>,
    pub summary: ResourceLayerSummary,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceLayerError {
    code: &'static str,
}

impl ResourceLayerError {
    pub const fn new(code: &'static str) -> Self {
        Self { code }
    }

    pub const fn code(&self) -> &'static str {
        self.code
    }
}

impl fmt::Display for ResourceLayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code)
    }
}

impl std::error::Error for ResourceLayerError {}

pub fn uniform_resource_layer(
    layer_index: usize,
    cells: usize,
    amount: f32,
) -> Result<ResourceLayerSummary, ResourceLayerError> {
    if !amount.is_finite() || amount < 0.0 {
        return Err(ResourceLayerError::new("BOOTSTRAP_INVALID_RESOURCE_LAYER"));
    }
    Ok(ResourceLayerSummary {
        layer_index,
        total: amount * cells as f32,
        min: amount,
        max: amount,
    })
}

pub fn patches_resource_layer(
    layer_index: usize,
    patches: usize,
    min_amount: f32,
    max_amount: f32,
    rng: &mut SplitMix64,
) -> Result<ResourceLayerSummary, ResourceLayerError> {
    if patches == 0
        || !min_amount.is_finite()
        || !max_amount.is_finite()
        || min_amount < 0.0
        || max_amount < min_amount
    {
        return Err(ResourceLayerError::new("BOOTSTRAP_INVALID_RESOURCE_LAYER"));
    }
    let mut total = 0.0;
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    for _ in 0..patches {
        let amount = min_amount + (max_amount - min_amount) * rng.next_f32();
        total += amount;
        min = min.min(amount);
        max = max.max(amount);
    }
    Ok(ResourceLayerSummary {
        layer_index,
        total,
        min,
        max,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn generate_patch_resource_layer(
    layer_index: usize,
    width: usize,
    height: usize,
    patches: usize,
    min_amount: f32,
    max_amount: f32,
    falloff: f32,
    rng: &mut SplitMix64,
) -> Result<PreparedResourceLayer, ResourceLayerError> {
    if width == 0
        || height == 0
        || patches == 0
        || !min_amount.is_finite()
        || !max_amount.is_finite()
        || !falloff.is_finite()
        || min_amount < 0.0
        || max_amount < min_amount
        || !(0.0..=1.0).contains(&falloff)
    {
        return Err(ResourceLayerError::new("BOOTSTRAP_INVALID_RESOURCE_LAYER"));
    }

    let cell_count = width * height;
    let mut quantities = vec![min_amount; cell_count];
    for _ in 0..patches {
        let center_x = (rng.next_u64() as usize) % width;
        let center_y = (rng.next_u64() as usize) % height;
        let peak = min_amount + (max_amount - min_amount) * rng.next_f32();
        let radius = ((width.max(height) as f32) * falloff.max(0.125)).max(1.0);
        for y in 0..height {
            for x in 0..width {
                let dx = x.abs_diff(center_x) as f32;
                let dy = y.abs_diff(center_y) as f32;
                let distance = (dx * dx + dy * dy).sqrt();
                let influence = (1.0 - distance / radius).clamp(0.0, 1.0);
                let amount = min_amount + (peak - min_amount) * influence;
                let index = y * width + x;
                quantities[index] = quantities[index].max(amount).min(max_amount);
            }
        }
    }
    prepared_layer(layer_index, quantities)
}

pub fn generate_gradient_resource_layer(
    layer_index: usize,
    width: usize,
    height: usize,
    min_amount: f32,
    max_amount: f32,
) -> Result<PreparedResourceLayer, ResourceLayerError> {
    if width == 0
        || height == 0
        || !min_amount.is_finite()
        || !max_amount.is_finite()
        || min_amount < 0.0
        || max_amount < min_amount
    {
        return Err(ResourceLayerError::new("BOOTSTRAP_INVALID_RESOURCE_LAYER"));
    }
    let denominator = width.saturating_sub(1).max(1) as f32;
    let mut quantities = Vec::with_capacity(width * height);
    for _y in 0..height {
        for x in 0..width {
            let t = x as f32 / denominator;
            quantities.push(min_amount + (max_amount - min_amount) * t);
        }
    }
    prepared_layer(layer_index, quantities)
}

pub fn uniform_prepared_resource_layer(
    layer_index: usize,
    cells: usize,
    amount: f32,
) -> Result<PreparedResourceLayer, ResourceLayerError> {
    if cells == 0 || !amount.is_finite() || amount < 0.0 {
        return Err(ResourceLayerError::new("BOOTSTRAP_INVALID_RESOURCE_LAYER"));
    }
    prepared_layer(layer_index, vec![amount; cells])
}

fn prepared_layer(
    layer_index: usize,
    raw_quantities: Vec<f32>,
) -> Result<PreparedResourceLayer, ResourceLayerError> {
    let mut total = 0.0;
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    let quantities = raw_quantities
        .into_iter()
        .map(|amount| {
            total += amount;
            min = min.min(amount);
            max = max.max(amount);
            ResourceAmount::new(amount)
                .map_err(|_| ResourceLayerError::new("BOOTSTRAP_INVALID_RESOURCE_LAYER"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PreparedResourceLayer {
        quantities,
        summary: ResourceLayerSummary {
            layer_index,
            total,
            min,
            max,
        },
    })
}
