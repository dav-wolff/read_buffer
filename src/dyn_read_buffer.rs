use std::io::{Read, self, ErrorKind};

/// A dynamically sized buffer to read into from a [Read] and safely access the read data.
/// 
/// **DynReadBuffer** provides a heap-allocated buffer to read into using
/// [`read_bytes`] or [`read_until`],
/// but crucially doesn't allow **any** access to the data inside the buffer
/// outside of the slices returned from [`read_bytes`] and [`read_until`].
/// 
/// This type is preferrable over [`ReadBuffer`] when the maximum expected size of a single read
/// is not known at compile time.
/// 
/// [`read_bytes`]: DynReadBuffer::read_bytes
/// [`read_until`]: DynReadBuffer::read_until
/// [`ReadBuffer`]: crate::ReadBuffer
pub struct DynReadBuffer<R: Read> {
	buffer: Vec<u8>,
	reader: R,
	filled_buffer_start: usize,
	filled_buffer_length: usize,
}

impl<R: Read> DynReadBuffer<R> {
	/// Creates a new **DynReadBuffer** to read from the given [Read].
	pub fn new(reader: R) -> Self {
		Self {
			buffer: Vec::new(),
			reader,
			filled_buffer_start: 0,
			filled_buffer_length: 0,
		}
	}
	
	/// Creates a new **DynReadBuffer** to read from the given [Read]
	/// with an internal buffer of at least the specified capacity.
	pub fn with_capacity(reader: R, capacity: usize) -> Self {
		Self {
			buffer: vec![0; capacity],
			reader,
			filled_buffer_start: 0,
			filled_buffer_length: 0,
		}
	}
	
	/// Reads the specified amount of bytes
	/// from the given [Read] into the internal buffer
	/// and returns a slice referencing the read data.
	/// 
	/// # Errors
	/// 
	/// If the given [Read] reaches its "end of file" before
	/// the requested amount of bytes could be read,
	/// an error of the kind [ErrorKind::UnexpectedEof][`UnexpectedEof`]
	/// is returned following the behavior of [Read::read_exact].
	/// 
	/// If an error of the kind [ErrorKind::Interrupted][`Interrupted`]
	/// is encountered, it is ignored.
	/// 
	/// All other errors from [Read::read_exact] are passed on to the caller.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # fn main() -> Result<(), std::io::Error> {
	/// use read_buffer::DynReadBuffer;
	/// 
	/// let mut reader = [1, 2, 3, 4].as_slice(); // Read is implemented for &[u8]
	/// let mut buffer = DynReadBuffer::new(reader);
	/// 
	/// let read_data = buffer.read_bytes(3)?;
	/// 
	/// assert_eq!(read_data, [1, 2, 3]);
	/// # Ok(())
	/// # }
	/// ```
	/// 
	/// [`UnexpectedEof`]: std::io::ErrorKind::UnexpectedEof
	/// [`Interrupted`]: std::io::ErrorKind::Interrupted
	pub fn read_bytes(&mut self, amount: usize) -> Result<&[u8], io::Error> {
		if amount > self.filled_buffer_length {
			let amount_to_fill = amount - self.filled_buffer_length;
			self.reserve(amount_to_fill);
			
			let start = self.filled_buffer_end();
			let end = start + amount_to_fill;
			let buffer_to_fill = &mut self.buffer[start..end];
			self.reader.read_exact(buffer_to_fill)?;
			
			self.filled_buffer_length += amount_to_fill;
		}
		
		let start = self.filled_buffer_start;
		let end = start + amount;
		let result = &self.buffer[start..end];
		
		self.filled_buffer_start += amount;
		self.filled_buffer_length -= amount;
		
		Ok(result)
	}
	
	/// Reads from the given [Read] until the specified delimiter is encountered
	/// and returns a slice referencing the data up to and including the delimiter.
	/// 
	/// # Errors
	/// 
	/// If any error occurs, the data read so far is preserved in the internal buffer
	/// for future reads.
	/// 
	/// If the given [Read] reaches its "end of file" before
	/// the delimiter was encountered, an error of the kind
	/// [ErrorKind::UnexpectedEof][`UnexpectedEof`] is returned.
	/// 
	/// If an error of the kind [ErrorKind::Interrupted][`Interrupted`]
	/// is encountered, it is ignored.
	/// 
	/// All other errors from [Read::read] are passed on to the caller.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # fn main() -> Result<(), std::io::Error> {
	/// use read_buffer::DynReadBuffer;
	/// 
	/// let mut reader = [1, 2, 3, 0, 4].as_slice();
	/// let mut buffer = DynReadBuffer::new(reader);
	/// 
	/// let read_data = buffer.read_until(0)?;
	/// 
	/// assert_eq!(read_data, [1, 2, 3, 0]);
	/// assert_eq!(buffer.read_bytes(1)?, [4]);
	/// # Ok(())
	/// # }
	/// ```
	/// 
	/// [`UnexpectedEof`]: std::io::ErrorKind::UnexpectedEof
	/// [`Interrupted`]: std::io::ErrorKind::Interrupted
	pub fn read_until(&mut self, delimiter: u8) -> Result<&[u8], io::Error> {
		if self.filled_buffer_length > 0 {
			let filled_buffer = &self.buffer[
				self.filled_buffer_start..self.filled_buffer_end()
			];
			let delimiter_position = filled_buffer.iter()
				.position(|byte| *byte == delimiter);
			
			if let Some(relative_position) = delimiter_position {
				let absolute_position = self.filled_buffer_start
					+ relative_position;
				let result = &self.buffer[self.filled_buffer_start..=absolute_position];
				self.filled_buffer_start = absolute_position + 1;
				self.filled_buffer_length -= result.len();
				return Ok(result);
			}
		}
		
		loop {
			self.reserve(32);
			
			let filled_buffer_end = self.filled_buffer_end();
			let available_buffer = &mut self.buffer[filled_buffer_end..];
			let amount_read = match self.reader.read(available_buffer) {
				Ok(n) => n,
				Err(err) if err.kind() == ErrorKind::Interrupted => continue,
				Err(err) => return Err(err),
			};
			
			if amount_read == 0 {
				return Err(ErrorKind::UnexpectedEof.into());
			}
			
			self.filled_buffer_length += amount_read;
			
			let read_data = &available_buffer[..amount_read];
			let delimiter_position = read_data.iter()
				.position(|byte| *byte == delimiter);
			
			if let Some(relative_position) = delimiter_position {
				let absolute_position = self.filled_buffer_end()
					- amount_read
					+ relative_position;
				let result = &self.buffer[self.filled_buffer_start..=absolute_position];
				self.filled_buffer_start = absolute_position + 1;
				self.filled_buffer_length -= result.len();
				return Ok(result);
			}
		}
	}
	
	fn reserve(&mut self, amount: usize) {
		let filled_buffer_end = self.filled_buffer_start + self.filled_buffer_length;
		
		if self.buffer.len() >= filled_buffer_end + amount {
			return;
		}
		
		if self.filled_buffer_start >= amount {
			self.buffer.rotate_left(self.filled_buffer_start);
			self.filled_buffer_start = 0;
			return;
		}
		
		self.buffer.resize(self.filled_buffer_end() + amount, 0);
	}
	
	fn filled_buffer_end(&self) -> usize {
		self.filled_buffer_start + self.filled_buffer_length
	}
}