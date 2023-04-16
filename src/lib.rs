use std::io;
use std::io::Read;

#[derive(Debug)]
pub struct ReadBuffer<const SIZE: usize> {
	buffer: [u8; SIZE],
}

impl<const SIZE: usize> ReadBuffer<SIZE> {
	pub fn new() -> Self {
		ReadBuffer {
			buffer: [0u8; SIZE],
		}
	}
	
	pub fn read_from(&mut self, source: &mut impl Read) -> Result<&[u8], io::Error> {
		let length = source.read(&mut self.buffer)?;
		Ok(&self.buffer[..length])
	}
}

impl<const SIZE: usize> Default for ReadBuffer<SIZE> {
	fn default() -> Self {
		Self::new()
	}
}