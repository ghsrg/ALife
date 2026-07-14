use alife::runner::ring_buffer::RingBuffer;

#[test]
fn ring_buffer_keeps_only_newest_items() {
    let mut buffer = RingBuffer::new(3).unwrap();
    buffer.push(1);
    buffer.push(2);
    buffer.push(3);
    buffer.push(4);

    assert_eq!(buffer.len(), 3);
    assert_eq!(buffer.iter().copied().collect::<Vec<_>>(), vec![2, 3, 4]);
    assert_eq!(buffer.newest(), Some(&4));
}

#[test]
fn ring_buffer_rejects_zero_capacity() {
    assert!(RingBuffer::<u32>::new(0).is_err());
}
