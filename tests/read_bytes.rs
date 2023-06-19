pub mod utils;

use std::io::ErrorKind;

use read_buffer::DynReadBuffer;
use crate::utils::ErrorReader;

#[test]
fn read() {
	let mut buffer = DynReadBuffer::new();
	let mut reader = [1, 2, 3, 4, 5, 6, 7, 8].as_slice();
	
	let result = buffer.read_bytes(&mut reader, 8).unwrap();
	assert_eq!(result.len(), 8);
	assert_eq!(
		result,
		[1, 2, 3, 4, 5, 6, 7, 8]
	);
}

#[test]
fn read_partial() {
	let mut buffer = DynReadBuffer::new();
	let mut reader = [5, 4, 3, 2, 1].as_slice();
	
	let result = buffer.read_bytes(&mut reader, 3).unwrap();
	assert_eq!(result.len(), 3);
	assert_eq!(
		result,
		[5, 4, 3]
	);
	
	let result = buffer.read_bytes(&mut reader, 2).unwrap();
	assert_eq!(result.len(), 2);
	assert_eq!(
		result,
		[2, 1]
	);
}

#[test]
fn read_nothing() {
	let mut buffer = DynReadBuffer::new();
	let mut reader = [1, 2, 3].as_slice();
	
	let result = buffer.read_bytes(&mut reader, 0).unwrap();
	assert!(result.is_empty());
	
	let result = buffer.read_bytes(&mut reader, 1).unwrap();
	assert_eq!(
		result,
		[1]
	);
	
	let result = buffer.read_bytes(&mut reader, 0).unwrap();
	assert!(result.is_empty());
	
	let result = buffer.read_bytes(&mut reader, 2).unwrap();
	assert_eq!(
		result,
		[2, 3]
	);
	
	let result = buffer.read_bytes(&mut reader, 0).unwrap();
	assert!(result.is_empty());
}

#[test]
fn default_construction() {
	let mut buffer: DynReadBuffer = Default::default();
	let mut reader = [1, 2, 3, 4].as_slice();
	
	let result = buffer.read_bytes(&mut reader, 4).unwrap();
	assert_eq!(
		result,
		[1, 2, 3, 4]
	);
}

#[test]
fn with_capacity() {
	let mut buffer = DynReadBuffer::with_capacity(2048);
	let mut reader = [1].as_slice();
	
	let result = buffer.read_bytes(&mut reader, 1).unwrap();
	assert_eq!(
		result,
		[1]
	);
	
	let original_address = &result[0] as *const u8 as usize;
	
	let mut reader = [5; 2048].as_slice();
	let result = buffer.read_bytes(&mut reader, 2048).unwrap();
	assert_eq!(
		result,
		[5; 2048]
	);
	
	let new_address = &result[0] as *const u8 as usize;
	
	assert!(original_address <= new_address && new_address < original_address + 2048);
}

#[test]
fn unexpected_eof() {
	let mut buffer = DynReadBuffer::new();
	let data = [1, 2, 4, 8, 16];
	let mut reader = data.as_slice();
	
	let error = buffer.read_bytes(&mut reader, 8).unwrap_err();
	assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
	
	let result = buffer.read_bytes(&mut reader, 5).unwrap();
	assert_eq!(
		result,
		[1, 2, 4, 8, 16]
	);
	
	let error = buffer.read_bytes(&mut reader, 1).unwrap_err();
	assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
}

#[test]
fn error_result() {
	let mut buffer = DynReadBuffer::new();
	let mut reader = ErrorReader;
	
	let error = buffer.read_bytes(&mut reader, 1).unwrap_err();
	assert_eq!(error.kind(), ErrorKind::NotFound);
}