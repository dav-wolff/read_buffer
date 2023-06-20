pub mod utils;

use std::io::ErrorKind;

use read_buffer::DynReadBuffer;
use crate::utils::{ErrorReader, ChunkedReader};

#[test]
fn read() {
	let reader = [1, 2, 3, 4, 5, 6, 7, 8].as_slice();
	let mut buffer = DynReadBuffer::new(reader);
	
	let result = buffer.read_bytes(8).unwrap();
	assert_eq!(result.len(), 8);
	assert_eq!(
		result,
		[1, 2, 3, 4, 5, 6, 7, 8]
	);
}

#[test]
fn read_partial() {
	let reader = [5, 4, 3, 2, 1].as_slice();
	let mut buffer = DynReadBuffer::new(reader);
	
	let result = buffer.read_bytes(3).unwrap();
	assert_eq!(result.len(), 3);
	assert_eq!(
		result,
		[5, 4, 3]
	);
	
	let result = buffer.read_bytes(2).unwrap();
	assert_eq!(result.len(), 2);
	assert_eq!(
		result,
		[2, 1]
	);
}

#[test]
fn read_nothing() {
	let reader = [1, 2, 3].as_slice();
	let mut buffer = DynReadBuffer::new(reader);
	
	let result = buffer.read_bytes(0).unwrap();
	assert!(result.is_empty());
	
	let result = buffer.read_bytes(1).unwrap();
	assert_eq!(
		result,
		[1]
	);
	
	let result = buffer.read_bytes(0).unwrap();
	assert!(result.is_empty());
	
	let result = buffer.read_bytes(2).unwrap();
	assert_eq!(
		result,
		[2, 3]
	);
	
	let result = buffer.read_bytes(0).unwrap();
	assert!(result.is_empty());
}

#[test]
fn with_capacity() {
	let mut reader = ChunkedReader::new();
	reader.add_chunk(vec![1]);
	reader.add_chunk(vec![5; 2048]);
	let mut buffer = DynReadBuffer::with_capacity(reader, 2048);
	
	let result = buffer.read_bytes(1).unwrap();
	assert_eq!(
		result,
		[1]
	);
	
	let original_address = &result[0] as *const u8 as usize;
	
	let result = buffer.read_bytes(2048).unwrap();
	assert_eq!(
		result,
		[5; 2048]
	);
	
	let new_address = &result[0] as *const u8 as usize;
	
	assert!(original_address <= new_address && new_address < original_address + 2048);
}

#[test]
fn unexpected_eof() {
	let reader = [1, 2, 4, 8, 16].as_slice();
	let mut buffer = DynReadBuffer::new(reader);
	
	let error = buffer.read_bytes(8).unwrap_err();
	assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
	
	let result = buffer.read_bytes(5).unwrap();
	assert_eq!(
		result,
		[1, 2, 4, 8, 16]
	);
	
	let error = buffer.read_bytes(1).unwrap_err();
	assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
}

#[test]
fn error_result() {
	let reader = ErrorReader;
	let mut buffer = DynReadBuffer::new(reader);
	
	let error = buffer.read_bytes(1).unwrap_err();
	assert_eq!(error.kind(), ErrorKind::NotFound);
}