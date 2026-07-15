use crate::runner::projections::{ProjectedCell, WorldFrameProjection};

pub const MAGIC: &[u8; 4] = b"ALIF";
pub const VERSION: u8 = 2;

const HEADER_SIZE: usize = 50;
const CELL_SIZE: usize = 21;

pub fn encode_world_frame(frame: &WorldFrameProjection) -> Vec<u8> {
    let cell_count = frame.cells.len();
    let mut bytes = Vec::with_capacity(HEADER_SIZE + cell_count * CELL_SIZE);

    bytes.extend_from_slice(MAGIC);
    bytes.push(VERSION);
    bytes.push(0);
    bytes.extend_from_slice(&frame.committed_tick.to_le_bytes());
    bytes.extend_from_slice(&frame.projection_sequence.to_le_bytes());
    bytes.extend_from_slice(&frame.wall_clock_generated_at_ms.to_le_bytes());
    bytes.extend_from_slice(&frame.previous_committed_tick.unwrap_or(u64::MAX).to_le_bytes());
    bytes.extend_from_slice(&frame.heat.to_le_bytes());
    bytes.extend_from_slice(&frame.waste.to_le_bytes());
    bytes.extend_from_slice(&(cell_count as u32).to_le_bytes());

    for cell in &frame.cells {
        bytes.extend_from_slice(&cell.id.to_le_bytes());
        bytes.extend_from_slice(&cell.x.to_le_bytes());
        bytes.extend_from_slice(&cell.y.to_le_bytes());
        bytes.extend_from_slice(&cell.radius.to_le_bytes());
        bytes.extend_from_slice(&cell.energy.to_le_bytes());
        bytes.push(cell.lifecycle);
    }

    bytes
}

pub fn decode_frame(bytes: &[u8]) -> Result<WorldFrameProjection, String> {
    if bytes.len() < HEADER_SIZE {
        return Err(format!(
            "Frame too short: {} bytes, expected at least {}",
            bytes.len(),
            HEADER_SIZE
        ));
    }
    if &bytes[0..4] != MAGIC {
        return Err("Invalid ALIF magic".to_string());
    }
    if bytes[4] != VERSION {
        return Err(format!("Unsupported ALIF version: {}", bytes[4]));
    }

    let committed_tick = u64::from_le_bytes(bytes[6..14].try_into().unwrap());
    let projection_sequence = u64::from_le_bytes(bytes[14..22].try_into().unwrap());
    let wall_clock_generated_at_ms = u64::from_le_bytes(bytes[22..30].try_into().unwrap());
    let previous_raw = u64::from_le_bytes(bytes[30..38].try_into().unwrap());
    let previous_committed_tick = if previous_raw == u64::MAX {
        None
    } else {
        Some(previous_raw)
    };
    let heat = f32::from_le_bytes(bytes[38..42].try_into().unwrap());
    let waste = f32::from_le_bytes(bytes[42..46].try_into().unwrap());
    let cell_count = u32::from_le_bytes(bytes[46..50].try_into().unwrap()) as usize;
    let expected_len = HEADER_SIZE + cell_count * CELL_SIZE;
    if bytes.len() < expected_len {
        return Err(format!(
            "Frame truncated: {} bytes, expected {} for {} cells",
            bytes.len(),
            expected_len,
            cell_count
        ));
    }

    let cells = (0..cell_count)
        .map(|index| {
            let offset = HEADER_SIZE + index * CELL_SIZE;
            ProjectedCell {
                id: u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()),
                x: f32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().unwrap()),
                y: f32::from_le_bytes(bytes[offset + 8..offset + 12].try_into().unwrap()),
                radius: f32::from_le_bytes(bytes[offset + 12..offset + 16].try_into().unwrap()),
                energy: f32::from_le_bytes(bytes[offset + 16..offset + 20].try_into().unwrap()),
                lifecycle: bytes[offset + 20],
            }
        })
        .collect();

    Ok(WorldFrameProjection {
        schema_version: VERSION,
        committed_tick,
        projection_sequence,
        wall_clock_generated_at_ms,
        previous_committed_tick,
        heat,
        waste,
        cells,
    })
}
