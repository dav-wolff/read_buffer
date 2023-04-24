use std::collections::VecDeque;
use std::io;
use std::io::{Read, Write};

pub struct ChunkedReader {
	chunks: VecDeque<Vec<u8>>,
}

impl ChunkedReader {
	pub fn new() -> Self {
		Self {
			chunks: VecDeque::new(),
		}
	}
	
	pub fn add_chunk(&mut self, chunk: Vec<u8>) {
		self.chunks.push_back(chunk);
	}
}

impl Read for ChunkedReader {
	fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
		let chunk = self.chunks.pop_front().unwrap_or_default();
		buf.write(chunk.as_slice())
	}
}