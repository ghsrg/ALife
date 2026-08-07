use crate::core::cell_store::LifecycleState;
use crate::core::ids::CellId;
use crate::core::resources::ResourceLayerIndex;
use crate::core::units::{EnergyAmount, Position, Radius, ResourceAmount, Tick};
use crate::core::world::WorldState;

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialSnapshot {
    pub material_type_id: u32,
    pub amount: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceAmountSnapshot {
    pub resource_type_id: u32,
    pub amount: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CellSnapshot {
    pub id: CellId,
    pub position: Position,
    pub radius: Radius,
    pub energy: EnergyAmount,
    pub energy_capacity: EnergyAmount,
    pub lifecycle_state: LifecycleState,
    pub materials: [f32; 9],
    pub internal_resources: Vec<ResourceAmount>,
    pub local_external_resources: Vec<ResourceAmount>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceLayerCellSnapshot {
    pub x: u32,
    pub y: u32,
    pub amount: ResourceAmount,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceLayerSnapshot {
    pub layer_index: u32,
    pub resource_type_id: u32,
    pub resource_id: String,
    pub width: u32,
    pub height: u32,
    pub total_amount: ResourceAmount,
    pub cells: Vec<ResourceLayerCellSnapshot>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JointSnapshot {
    pub id: u32,
    pub cell1_id: CellId,
    pub cell2_id: CellId,
    pub rest_length: f32,
    pub pulse_intensity: f32,
    pub signal_speed: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrganismSnapshot {
    pub id: u32,
    pub cell_ids: Vec<CellId>,
    pub hull_color_hue: u16,
    pub organic_membrane_tension: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommittedSnapshot {
    pub tick: Tick,
    pub cells: Vec<CellSnapshot>,
    pub joints: Vec<JointSnapshot>,
    pub organisms: Vec<OrganismSnapshot>,
    pub heat: f32,
    pub waste: f32,
    pub resource_layer_totals: Vec<ResourceAmount>,
    pub resource_layers: Vec<ResourceLayerSnapshot>,
}

impl CommittedSnapshot {
    pub fn from_world(world: &WorldState) -> Self {
        let cells: Vec<CellSnapshot> = world
            .cells()
            .iter_indices()
            .map(|index| {
                let coord = world
                    .resources()
                    .coord_for_position(world.cells().position(index));
                let local_external_resources = (0..world.resources().layer_count())
                    .map(|layer| {
                        world
                            .resources()
                            .amount_at(ResourceLayerIndex::from_raw(layer), coord)
                            .unwrap_or_else(|_| ResourceAmount::zero())
                    })
                    .collect();
                CellSnapshot {
                    id: world.cells().id_at(index),
                    position: world.cells().position(index),
                    radius: world.cells().radius(index),
                    energy: world.cells().energy(index).current(),
                    energy_capacity: world.cells().energy(index).capacity(),
                    lifecycle_state: world.cells().lifecycle_state(index),
                    materials: [
                        world.cells().boundary_material(index).raw(),
                        world.cells().transport_material(index).raw(),
                        world.cells().metabolic_material(index).raw(),
                        world.cells().storage_material(index).raw(),
                        world.cells().synthesis_material(index).raw(),
                        world.cells().structural_material(index).raw(),
                        world.cells().repair_material(index).raw(),
                        world.cells().contractile_material(index).raw(),
                        world.cells().sensory_material(index).raw(),
                    ],
                    internal_resources: vec![world.cells().resource_amount(index)],
                    local_external_resources,
                }
            })
            .collect();

        let resource_layer_totals = (0..world.resources().layer_count())
            .map(|layer| {
                world
                    .resources()
                    .total_amount_for_layer(ResourceLayerIndex::from_raw(layer))
                    .expect("layer range is derived from layer_count")
            })
            .collect();
        let resource_layers = (0..world.resources().layer_count())
            .map(|layer| {
                let layer_index = ResourceLayerIndex::from_raw(layer);
                let mut cells = Vec::with_capacity(world.resources().cell_count());
                for y in 0..world.resources().height() {
                    for x in 0..world.resources().width() {
                        let coord = crate::core::units::GridCoord::new(x, y);
                        cells.push(ResourceLayerCellSnapshot {
                            x: x as u32,
                            y: y as u32,
                            amount: world
                                .resources()
                                .amount_at(layer_index, coord)
                                .expect("coord range is derived from resource grid"),
                        });
                    }
                }
                ResourceLayerSnapshot {
                    layer_index: layer as u32,
                    resource_type_id: layer as u32,
                    resource_id: world
                        .config()
                        .chemistry
                        .resources
                        .get(layer)
                        .map(|resource| resource.id.clone())
                        .unwrap_or_else(|| format!("resource_{layer}")),
                    width: world.resources().width() as u32,
                    height: world.resources().height() as u32,
                    total_amount: world
                        .resources()
                        .total_amount_for_layer(layer_index)
                        .expect("layer range is derived from layer_count"),
                    cells,
                }
            })
            .collect();

        let joints: Vec<JointSnapshot> = world
            .joints()
            .active_ids()
            .filter_map(|joint_id| {
                let endpoints = world.joints().endpoints(joint_id)?;
                let c1_id = world.cells().id_at(endpoints.a);
                let c2_id = world.cells().id_at(endpoints.b);
                let c1_pos = world.cells().position(endpoints.a);
                let c2_pos = world.cells().position(endpoints.b);
                let dx = c1_pos.x() - c2_pos.x();
                let dy = c1_pos.y() - c2_pos.y();
                let dist = (dx * dx + dy * dy).sqrt();
                let pulse_intensity = world
                    .joints()
                    .readable_signal(joint_id, world.tick())
                    .unwrap_or(0.0);
                Some(JointSnapshot {
                    id: joint_id.raw(),
                    cell1_id: c1_id,
                    cell2_id: c2_id,
                    rest_length: dist,
                    pulse_intensity: 0.5 + pulse_intensity * 0.5,
                    signal_speed: 1.0,
                })
            })
            .collect();

        let mut visited = std::collections::HashSet::new();
        let mut organisms = Vec::new();
        let mut organism_id_counter = 1u32;

        for cell in &cells {
            if visited.contains(&cell.id.raw()) {
                continue;
            }
            let mut component = Vec::new();
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(cell.id);
            visited.insert(cell.id.raw());

            while let Some(current_id) = queue.pop_front() {
                component.push(current_id);
                for joint in &joints {
                    let neighbor_id = if joint.cell1_id == current_id {
                        Some(joint.cell2_id)
                    } else if joint.cell2_id == current_id {
                        Some(joint.cell1_id)
                    } else {
                        None
                    };
                    if let Some(nid) = neighbor_id {
                        if !visited.contains(&nid.raw()) {
                            visited.insert(nid.raw());
                            queue.push_back(nid);
                        }
                    }
                }
            }

            if component.len() >= 2 {
                let hull_hue =
                    ((component.first().map(|c| c.raw()).unwrap_or(1) * 149) % 360) as u16;
                organisms.push(OrganismSnapshot {
                    id: organism_id_counter,
                    cell_ids: component,
                    hull_color_hue: hull_hue,
                    organic_membrane_tension: 0.75,
                });
                organism_id_counter += 1;
            }
        }

        Self {
            tick: world.tick(),
            cells,
            joints,
            organisms,
            heat: world.environment().heat().raw(),
            waste: world.environment().waste().raw(),
            resource_layer_totals,
            resource_layers,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewerCell {
    pub id: CellId,
    pub position: Position,
    pub radius: Radius,
    pub lifecycle_state: LifecycleState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ViewerFrame {
    pub tick: Tick,
    pub cells: Vec<ViewerCell>,
    pub heat: f32,
    pub waste: f32,
}

impl ViewerFrame {
    pub fn from_snapshot(snapshot: &CommittedSnapshot) -> Self {
        Self {
            tick: snapshot.tick,
            cells: snapshot
                .cells
                .iter()
                .map(|cell| ViewerCell {
                    id: cell.id,
                    position: cell.position,
                    radius: cell.radius,
                    lifecycle_state: cell.lifecycle_state,
                })
                .collect(),
            heat: snapshot.heat,
            waste: snapshot.waste,
        }
    }
}
