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

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn empty_buffer() {
		let buffer = ReadBuffer::new();
		assert_eq!(
			buffer.buffer,
			[0u8; 256]
		);
		
		let buffer: ReadBuffer<128> = ReadBuffer::new();
		assert_eq!(
			buffer.buffer,
			[0u8; 128]
		);
	}
	
	#[test]
	fn read() {
		let mut buffer: ReadBuffer<256> = ReadBuffer::new();
		let data = [1, 1, 2, 3, 5, 8, 13, 21];
		let mut data = &data[..];
		
		let result = buffer.read_from(&mut data).unwrap();
		assert_eq!(result.len(), 8);
		assert_eq!(
			result,
			[1, 1, 2, 3, 5, 8, 13, 21]
		);
	}
	
	#[test]
	fn read_partial() {
		let mut buffer: ReadBuffer<8> = ReadBuffer::new();
		let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let mut data = &data[..];
		
		let result = buffer.read_from(&mut data).unwrap();
		assert_eq!(result.len(), 8);
		assert_eq!(
			result,
			[1, 2, 3, 4, 5, 6, 7, 8]
		);
		
		let result = buffer.read_from(&mut data).unwrap();
		assert_eq!(result.len(), 2);
		assert_eq!(
			result,
			[9, 10]
		);
	}
}