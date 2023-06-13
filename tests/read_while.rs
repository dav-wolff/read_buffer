pub mod utils;

use std::io::ErrorKind;
use read_buffer::ReadBuffer;
use crate::utils::{ChunkedReader, ErrorReader};

#[test]
fn read() {
	let mut buffer: ReadBuffer<256> = ReadBuffer::new();
	let mut reader = [1, 1, 2, 3, 5, 8, 13, 21].as_slice();
	
	let result = buffer.read_while(&mut reader, |_chunk| true).unwrap();
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
	
	let result = buffer.read_while(&mut reader, |_chunk| true).unwrap();
	assert_eq!(result.len(), 8);
	assert_eq!(
		result,
		[1, 2, 3, 4, 5, 6, 7, 8]
	);
	
	let result = buffer.read_while(&mut reader, |_chunk| true).unwrap();
	assert_eq!(result.len(), 2);
	assert_eq!(
		result,
		[9, 10]
	);
}

#[test]
fn read_chunks() {
	let mut buffer: ReadBuffer<64> = ReadBuffer::new();
	let mut reader = ChunkedReader::new();
	reader.add_chunk(vec![1]);
	reader.add_chunk(vec![2, 3]);
	reader.add_chunk(vec![3, 2, 1]);
	
	let mut received_chunks: Vec<Vec<u8>> = Vec::new();
	
	let result = buffer.read_while(&mut reader, |chunk| {
		received_chunks.push(chunk.into());
		true
	}).unwrap();
	
	assert_eq!(result.len(), 6);
	assert_eq!(
		result,
		[1, 2, 3, 3, 2, 1]
	);
	assert_eq!(
		received_chunks,
		vec![
			vec![1],
			vec![2, 3],
			vec![3, 2, 1]
		]
	);
}

#[test]
fn read_chunks_conditional() {
	let mut buffer: ReadBuffer<64> = ReadBuffer::new();
	let mut reader = ChunkedReader::new();
	reader.add_chunk(vec![1, 2, 3]);
	reader.add_chunk(vec![4, 3, 2, 1]);
	reader.add_chunk(vec![1, 0, 1]);
	reader.add_chunk(vec![2, 3, 4, 5]);
	
	let mut received_chunks: Vec<Vec<u8>> = Vec::new();
	
	let result = buffer.read_while(&mut reader, |chunk| {
		received_chunks.push(chunk.into());
		!chunk.contains(&0)
	}).unwrap();
	
	assert_eq!(result.len(), 10);
	assert_eq!(
		result,
		[1, 2, 3, 4, 3, 2, 1, 1, 0, 1]
	);
	assert_eq!(
		received_chunks,
		vec![
			vec![1, 2, 3],
			vec![4, 3, 2, 1],
			vec![1, 0, 1]
		]
	);
}

#[test]
fn read_chunks_partial() {
	let mut buffer: ReadBuffer<8> = ReadBuffer::new();
	let mut reader = ChunkedReader::new();
	reader.add_chunk(vec![1, 2, 3]);
	reader.add_chunk(vec![4, 5, 6, 7, 8]);
	reader.add_chunk(vec![9, 10, 11, 12]);
	
	let mut received_chunks: Vec<Vec<u8>> = Vec::new();
	
	let result = buffer.read_while(&mut reader, |chunk| {
		received_chunks.push(chunk.into());
		true
	}).unwrap();
	
	assert_eq!(result.len(), 8);
	assert_eq!(
		result,
		[1, 2, 3, 4, 5, 6, 7, 8]
	);
	assert_eq!(
		received_chunks,
		vec![
			vec![1, 2, 3],
			vec![4, 5, 6, 7, 8]
		]
	);
	
	received_chunks.clear();
	
	let result = buffer.read_while(&mut reader, |chunk| {
		received_chunks.push(chunk.into());
		true
	}).unwrap();
	
	assert_eq!(result.len(), 4);
	assert_eq!(
		result,
		[9, 10, 11, 12]
	);
	assert_eq!(
		received_chunks,
		vec![
			vec![9, 10, 11, 12]
		]
	);
}

#[test]
fn error_result() {
	let mut buffer: ReadBuffer<64> = ReadBuffer::new();
	let mut reader = ErrorReader;
	
	let error = buffer.read_while(&mut reader, |_chunk| true).err().unwrap();
	assert_eq!(error.kind(), ErrorKind::NotFound);
}