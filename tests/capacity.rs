use read_buffer::ReadBuffer;

#[test]
fn capacity() {
	let buffer_1: ReadBuffer<1> = ReadBuffer::new();
	let buffer_30: ReadBuffer<30> = ReadBuffer::new();
	let buffer_128: ReadBuffer<128> = ReadBuffer::new();
	
	assert_eq!(buffer_1.capacity(), 1);
	assert_eq!(buffer_30.capacity(), 30);
	assert_eq!(buffer_128.capacity(), 128);
}