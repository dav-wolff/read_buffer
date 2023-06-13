pub mod utils;

use std::io::ErrorKind;
use read_buffer::ReadBuffer;
use crate::utils::ErrorReader;

#[test]
fn read() {
	let mut buffer: ReadBuffer<256> = ReadBuffer::new();
	let mut reader = [1, 1, 2, 3, 5, 8, 13, 21].as_slice();
	
	let result = buffer.read_from(&mut reader).unwrap();
	assert_eq!(result.len(), 8);
	assert_eq!(
		result,
		[1, 1, 2, 3, 5, 8, 13, 21]
	);
}

#[test]
fn read_partial() {
	let mut buffer: ReadBuffer<8> = ReadBuffer::new();
	let mut reader = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].as_slice();
	
	let result = buffer.read_from(&mut reader).unwrap();
	assert_eq!(result.len(), 8);
	assert_eq!(
		result,
		[1, 2, 3, 4, 5, 6, 7, 8]
	);
	
	let result = buffer.read_from(&mut reader).unwrap();
	assert_eq!(result.len(), 2);
	assert_eq!(
		result,
		[9, 10]
	);
}

#[test]
fn default_construction() {
	let buffer: Option<ReadBuffer<16>> = None;
	let mut buffer = buffer.unwrap_or_default();
	let mut reader = [4, 5, 45, 54].as_slice();
	
	let result = buffer.read_from(&mut reader).unwrap();
	assert_eq!(result.len(), 4);
	assert_eq!(
		result,
		[4, 5, 45, 54]
	);
}

#[test]
#[should_panic]
fn out_of_bounds_access() {
	let mut buffer: ReadBuffer<128> = ReadBuffer::new();
	let mut reader = [1, 2, 3, 4].as_slice();
	
	let Ok(result) = buffer.read_from(&mut reader) else {
		return; // don't panic so test will fail
	};
	result[4];
}

#[test]
#[should_panic]
fn out_of_bounds_with_empty_data() {
	let mut buffer: ReadBuffer<128> = ReadBuffer::new();
	let mut reader = [0; 0].as_slice();
	
	let Ok(result) = buffer.read_from(&mut reader) else {
		return; // don't panic so test will fail
	};
	result[0];
}

#[test]
fn error_result() {
	let mut buffer: ReadBuffer<64> = ReadBuffer::new();
	let mut reader = ErrorReader;
	
	let error = buffer.read_from(&mut reader).err().unwrap();
	assert_eq!(error.kind(), ErrorKind::NotFound);
}