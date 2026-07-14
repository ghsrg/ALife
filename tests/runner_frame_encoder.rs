use alife::runner::projections::{ProjectedCell, WorldFrameProjection};
use alife::viewer_server::frame_encoder::{MAGIC, VERSION, decode_frame, encode_world_frame};

fn make_projection(tick: u64, cell_count: usize) -> WorldFrameProjection {
    WorldFrameProjection {
        schema_version: 1,
        committed_tick: tick,
        heat: 1.5,
        waste: 0.25,
        cells: (0..cell_count)
            .map(|index| ProjectedCell {
                id: index as u32,
                x: index as f32 * 10.0,
                y: index as f32 * 5.0,
                radius: 4.0 + index as f32,
                energy: 50.0 + index as f32,
                lifecycle: index as u8 % 4,
            })
            .collect(),
    }
}

#[test]
fn encoded_frame_starts_with_magic_bytes() {
    let bytes = encode_world_frame(&make_projection(1, 0));
    assert_eq!(&bytes[0..4], MAGIC);
}

#[test]
fn encoded_frame_has_correct_version() {
    let bytes = encode_world_frame(&make_projection(1, 0));
    assert_eq!(bytes[4], VERSION);
}

#[test]
fn encoded_frame_encodes_tick_correctly() {
    let bytes = encode_world_frame(&make_projection(999, 0));
    let tick = u64::from_le_bytes(bytes[6..14].try_into().unwrap());
    assert_eq!(tick, 999);
}

#[test]
fn encoded_frame_encodes_heat_and_waste() {
    let bytes = encode_world_frame(&make_projection(1, 0));
    let heat = f32::from_le_bytes(bytes[14..18].try_into().unwrap());
    let waste = f32::from_le_bytes(bytes[18..22].try_into().unwrap());
    assert!((heat - 1.5).abs() < 1e-5);
    assert!((waste - 0.25).abs() < 1e-5);
}

#[test]
fn encoded_frame_with_zero_cells_has_26_bytes() {
    let bytes = encode_world_frame(&make_projection(1, 0));
    assert_eq!(bytes.len(), 26);
}

#[test]
fn encoded_frame_cell_count_and_size_match() {
    let bytes = encode_world_frame(&make_projection(1, 3));
    let count = u32::from_le_bytes(bytes[22..26].try_into().unwrap());
    assert_eq!(count, 3);
    assert_eq!(bytes.len(), 26 + 3 * 21);
}

#[test]
fn encode_decode_roundtrip_preserves_tick_and_cell_count() {
    let bytes = encode_world_frame(&make_projection(42, 2));
    let decoded = decode_frame(&bytes).expect("decode must succeed");
    assert_eq!(decoded.committed_tick, 42);
    assert_eq!(decoded.cells.len(), 2);
}

#[test]
fn encode_decode_roundtrip_preserves_cell_fields() {
    let bytes = encode_world_frame(&make_projection(10, 1));
    let decoded = decode_frame(&bytes).expect("decode must succeed");
    let cell = &decoded.cells[0];
    assert_eq!(cell.id, 0);
    assert!((cell.x - 0.0).abs() < 1e-4);
    assert!((cell.y - 0.0).abs() < 1e-4);
    assert!((cell.radius - 4.0).abs() < 1e-4);
    assert!((cell.energy - 50.0).abs() < 1e-4);
    assert_eq!(cell.lifecycle, 0);
}

#[test]
fn lifecycle_states_encode_correctly() {
    fn state_byte(lifecycle: u8) -> u8 {
        let mut frame = make_projection(0, 1);
        frame.cells[0].lifecycle = lifecycle;
        let bytes = encode_world_frame(&frame);
        bytes[26 + 20]
    }

    assert_eq!(state_byte(0), 0);
    assert_eq!(state_byte(1), 1);
    assert_eq!(state_byte(2), 2);
    assert_eq!(state_byte(3), 3);
}

#[test]
fn decode_returns_error_on_wrong_magic() {
    let mut bytes = encode_world_frame(&make_projection(1, 0));
    bytes[0] = 0xFF;
    assert!(decode_frame(&bytes).is_err());
}

#[test]
fn decode_returns_error_on_truncated_header() {
    assert!(decode_frame(&[0; 10]).is_err());
}
