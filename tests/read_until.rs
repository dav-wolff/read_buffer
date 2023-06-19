use std::io::ErrorKind;

use read_buffer::DynReadBuffer;
use utils::ChunkedReader;

pub mod utils;

#[test]
fn read_all() {
	let mut buffer = DynReadBuffer::new();
	let mut reader = [1, 2, 3, 4, 0].as_slice();
	
	let result = buffer.read_until(&mut reader, 0).unwrap();
	assert_eq!(
		result,
		[1, 2, 3, 4, 0]
	);
}

#[test]
fn unexpected_eof() {
	let mut buffer = DynReadBuffer::new();
	let mut reader = [1, 2, 3].as_slice();
	
	let error = buffer.read_until(&mut reader, 0).unwrap_err();
	assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
	
	let result = buffer.read_until(&mut reader, 3).unwrap();
	assert_eq!(
		result,
		[1, 2, 3]
	);
	
	let error = buffer.read_until(&mut reader, 3).unwrap_err();
	assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
}

#[test]
fn data_after_eof() {
	let mut buffer = DynReadBuffer::new();
	let mut reader = ChunkedReader::new();
	reader.add_chunk(vec![1, 2, 3]);
	
	let error = buffer.read_until(&mut reader, 0).unwrap_err();
	assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
	
	reader.add_chunk(vec![4, 0]);
	
	let result = buffer.read_until(&mut reader, 0).unwrap();
	assert_eq!(
		result,
		[1, 2, 3, 4, 0]
	);
}

#[test]
fn read_chunks() {
	let mut buffer = DynReadBuffer::new();
	let mut reader = ChunkedReader::new();
	reader.add_chunk(vec![1, 2, 4]);
	reader.add_chunk(vec![8, 16]);
	reader.add_chunk(vec![32]);
	
	let result = buffer.read_until(&mut reader, 32).unwrap();
	assert_eq!(
		result,
		[1, 2, 4, 8, 16, 32]
	);
}

fn generate_sequence(length: usize, start: usize)-> Vec<u8> {
	(start..start + length).into_iter()
		.map(|i| i % 255 + 1) // 1 <= x <= 255
		.map(|i| -> u8 {i.try_into().unwrap()})
		.collect()
}

#[test]
fn read_reallocating() {
	let sequence = generate_sequence(250, 0);
	let mut buffer = DynReadBuffer::new();
	let mut reader = ChunkedReader::new();
	reader.add_chunk(sequence.clone());
	reader.add_chunk(vec![0]);
	
	let result = buffer.read_until(&mut reader, 0).unwrap();
	assert_eq!(dbg!(result).len(), 251);
	assert_eq!(result[0..250], sequence);
	assert_eq!(result[250], 0);
}

#[test]
fn read_multiple_reallocating() {
	let sequence_a = generate_sequence(250, 0);
	let sequence_b = generate_sequence(200, 100);
	let mut buffer = DynReadBuffer::new();
	let mut reader = ChunkedReader::new();
	reader.add_chunk(sequence_a.clone());
	reader.add_chunk(vec![0]);
	reader.add_chunk(sequence_b.clone());
	reader.add_chunk(vec![0]);
	
	let result = buffer.read_until(&mut reader, 0).unwrap();
	assert_eq!(result.len(), 251);
	assert_eq!(result[0..250], sequence_a);
	assert_eq!(result[250], 0);
	
	let result = buffer.read_until(&mut reader, 0).unwrap();
	assert_eq!(result.len(), 201);
	assert_eq!(result[0..200], sequence_b);
	assert_eq!(result[200], 0);
}

#[test]
fn read_multiple_reallocating_continuous() {
	let sequence_a = generate_sequence(200, 0);
	let sequence_b = generate_sequence(50, 50);
	let sequence_c = generate_sequence(300, 200);
	let mut data = sequence_a.clone();
	data.push(0);
	data.extend_from_slice(&sequence_b);
	data.push(0);
	data.extend_from_slice(&sequence_c);
	data.push(0);
	let mut reader = data.as_slice();
	let mut buffer = DynReadBuffer::new();
	
	let result = buffer.read_until(&mut reader, 0).unwrap();
	assert_eq!(result.len(), 201);
	assert_eq!(result[0..200], sequence_a);
	assert_eq!(result[200], 0);
	
	let result = buffer.read_until(&mut reader, 0).unwrap();
	assert_eq!(result.len(), 51);
	assert_eq!(result[0..50], sequence_b);
	assert_eq!(result[50], 0);
	
	let result = buffer.read_until(&mut reader, 0).unwrap();
	assert_eq!(result.len(), 301);
	assert_eq!(result[0..300], sequence_c);
	assert_eq!(result[300], 0);
}

#[test]
fn data_after_error() {
	let mut buffer = DynReadBuffer::new();
	let mut reader = ChunkedReader::new();
	reader.add_chunk(vec![3, 26, 12]);
	reader.add_error(ErrorKind::NotFound.into());
	reader.add_chunk(vec![3, 31, 22]);
	
	let error = buffer.read_until(&mut reader, 22).unwrap_err();
	assert_eq!(error.kind(), ErrorKind::NotFound);
	
	let result = buffer.read_until(&mut reader, 22).unwrap();
	assert_eq!(
		result,
		[3, 26, 12, 3, 31, 22]
	);
}

#[test]
fn continues_after_interrupt() {
	let mut buffer = DynReadBuffer::new();
	let mut reader = ChunkedReader::new();
	reader.add_chunk(vec![14, 15, 1]);
	reader.add_error(ErrorKind::Interrupted.into());
	reader.add_chunk(vec![12, 15, 0]);
	
	let result = buffer.read_until(&mut reader, 0).unwrap();
	assert_eq!(
		result,
		[14, 15, 1, 12, 15, 0]
	);
}