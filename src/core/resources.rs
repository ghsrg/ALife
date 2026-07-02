use crate::core::units::ResourceAmount;

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceGrid {
    layers: Vec<ResourceAmount>,
    optional_decay_rate: f32,
}

impl ResourceGrid {
    pub fn new(initial: ResourceAmount, decay_rate: f32) -> Self {
        Self {
            layers: vec![initial],
            optional_decay_rate: decay_rate,
        }
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn decay_or_passive_update(&mut self) {
        for layer in &mut self.layers {
            let next_val = (layer.raw() * (1.0 - self.optional_decay_rate)).max(0.0);
            *layer = ResourceAmount::new(next_val).unwrap_or_else(|_| ResourceAmount::zero());
        }
    }

    pub fn layers(&self) -> &[ResourceAmount] {
        &self.layers
    }
}
