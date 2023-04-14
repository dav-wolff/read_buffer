#[derive(Debug, Eq, PartialEq)]
pub struct ReadBuffer<const SIZE: usize> {
	buffer: [u8; SIZE],
}

impl<const SIZE: usize> ReadBuffer<SIZE> {
	pub fn new() -> Self {
		ReadBuffer {
			buffer: [0u8; SIZE],
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn empty_buffer() {
		let buffer = ReadBuffer::new();
		assert_eq!(buffer, ReadBuffer {
			buffer: [0u8; 16],
		});
		
		let buffer: ReadBuffer<8> = ReadBuffer::new();
		assert_eq!(buffer, ReadBuffer {
			buffer: [0u8; 8],
		});
	}
}