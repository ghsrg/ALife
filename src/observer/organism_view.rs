use crate::core::cell_store::{CellIndex, LifecycleState};
use crate::core::world::WorldState;
use crate::observer::payloads::{
    OrganismViewPayload, OrganismViewProjection, ProjectionSourceMetricRef,
};
use crate::observer::projection_envelope::ProjectionCompleteness;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn build_organism_view_projection(world: &WorldState) -> OrganismViewProjection {
    let cells = world.cells();
    let joints = world.joints();

    let active_cell_indices: Vec<CellIndex> = (0..cells.len())
        .map(CellIndex::from_raw)
        .filter(|&idx| cells.lifecycle_state(idx) != LifecycleState::Dead)
        .collect();

    let mut adjacency: HashMap<CellIndex, Vec<CellIndex>> = HashMap::new();
    for &idx in &active_cell_indices {
        adjacency.insert(idx, Vec::new());
    }

    for joint_id in joints.active_ids() {
        if let Some(endpoints) = joints.endpoints(joint_id)
            && cells.lifecycle_state(endpoints.a) != LifecycleState::Dead
            && cells.lifecycle_state(endpoints.b) != LifecycleState::Dead
        {
            adjacency.entry(endpoints.a).or_default().push(endpoints.b);
            adjacency.entry(endpoints.b).or_default().push(endpoints.a);
        }
    }

    let mut visited: HashSet<CellIndex> = HashSet::new();
    let mut organisms: Vec<OrganismViewPayload> = Vec::new();
    let mut unattached_cells_count = 0;
    let mut organism_counter = 1;

    for &start_idx in &active_cell_indices {
        if visited.contains(&start_idx) {
            continue;
        }

        let mut component_cells: Vec<CellIndex> = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(start_idx);
        visited.insert(start_idx);

        while let Some(current) = queue.pop_front() {
            component_cells.push(current);
            if let Some(neighbors) = adjacency.get(&current) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        component_cells.sort_by_key(|c| c.raw());

        let total_cells_count = component_cells.len();
        if total_cells_count == 1 {
            unattached_cells_count += 1;
        }

        let primary_cell_id = component_cells[0].raw() as u32;

        let mut total_mass = 0.0;
        let mut total_energy = 0.0;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;

        for &idx in &component_cells {
            let pos = cells.position(idx);
            sum_x += pos.x();
            sum_y += pos.y();
            total_energy += cells.energy(idx).current().raw();

            let boundary = cells.boundary_material(idx).raw();
            let transport = cells.transport_material(idx).raw();
            let metabolic = cells.metabolic_material(idx).raw();
            let storage = cells.storage_material(idx).raw();
            let synthesis = cells.synthesis_material(idx).raw();
            let structural = cells.structural_material(idx).raw();
            let repair = cells.repair_material(idx).raw();
            let contractile = cells.contractile_material(idx).raw();
            let sensory = cells.sensory_material(idx).raw();

            let cell_mass = boundary
                + transport
                + metabolic
                + storage
                + synthesis
                + structural
                + repair
                + contractile
                + sensory;
            total_mass += cell_mass;
        }

        let centroid_x = if total_cells_count > 0 {
            sum_x / total_cells_count as f32
        } else {
            0.0
        };
        let centroid_y = if total_cells_count > 0 {
            sum_y / total_cells_count as f32
        } else {
            0.0
        };

        let cell_set: HashSet<CellIndex> = component_cells.iter().copied().collect();
        let mut organism_joints_count = 0;
        for joint_id in joints.active_ids() {
            if let Some(endpoints) = joints.endpoints(joint_id)
                && cell_set.contains(&endpoints.a)
                && cell_set.contains(&endpoints.b)
            {
                organism_joints_count += 1;
            }
        }

        organisms.push(OrganismViewPayload {
            organism_id: organism_counter,
            cell_ids: component_cells.iter().map(|c| c.raw() as u32).collect(),
            primary_cell_id,
            total_cells_count,
            total_mass,
            total_energy,
            total_joints_count: organism_joints_count,
            centroid_x,
            centroid_y,
            confidence: 1.0,
            completeness: ProjectionCompleteness::full(),
        });

        organism_counter += 1;
    }

    let total_organisms_count = organisms.len();

    OrganismViewProjection {
        tick: world.tick().raw(),
        organisms,
        total_organisms_count,
        unattached_cells_count,
        completeness: ProjectionCompleteness::full(),
        source_metrics: vec![
            ProjectionSourceMetricRef::new("cell.alive", "CellStore", "cells"),
            ProjectionSourceMetricRef::new("joint.active", "JointStore", "joints"),
        ],
    }
}
