use std::collections::VecDeque;
use std::io;
use std::io::Read;

pub struct ChunkedReader {
	chunks: VecDeque<Result<Vec<u8>, io::Error>>,
}

impl ChunkedReader {
	pub fn new() -> Self {
		Self {
			chunks: VecDeque::new(),
		}
	}
	
	pub fn add_chunk(&mut self, chunk: Vec<u8>) {
		self.chunks.push_back(Ok(chunk));
	}
	
	pub fn add_eof(&mut self) {
		self.chunks.push_back(Ok(Vec::new()));
	}
	
	pub fn add_error(&mut self, error: io::Error) {
		self.chunks.push_back(Err(error));
	}
}

impl Read for ChunkedReader {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		let chunk = self.chunks.pop_front()
			.unwrap_or(Ok(Vec::new()))?;
		let mut slice = chunk.as_slice();
		let amount_read = slice.read(buf)?;
		
		if !slice.is_empty() {
			self.chunks.push_front(Ok(slice.to_vec()));
		}
		
		Ok(amount_read)
	}
}