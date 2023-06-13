use std::{iter, io::{Read, self}};

/// A dynamically sized buffer to read into from a [Read] and safely access the read data.
/// 
/// **DynReadBuffer** provides a heap-allocated buffer to read into using [`read_bytes`],
/// but crucially doesn't allow **any** access to the data inside the buffer
/// outside of the slice returned from [`read_bytes`].
/// 
/// This type is preferrable over [`ReadBuffer`] when the maximum expected size of a single read
/// is not known at compile time.
/// 
/// [`read_bytes`]: DynReadBuffer::read_bytes
/// [`ReadBuffer`]: crate::ReadBuffer
pub struct DynReadBuffer {
	buffer: Vec<u8>,
}

impl DynReadBuffer {
	/// Creates a new **DynReadBuffer**.
	pub fn new() -> Self {
		Self {
			buffer: Vec::new(),
		}
	}
	
	/// Creates a new **DynReadBuffer** with an internal buffer of at least the specified capacity.
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			buffer: Vec::with_capacity(capacity),
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
	/// let mut buffer = DynReadBuffer::new();
	/// 
	/// let read_data = buffer.read_bytes(&mut reader, 3)?;
	/// 
	/// assert_eq!(read_data, [1, 2, 3]);
	/// # Ok(())
	/// # }
	/// ```
	/// 
	/// [`UnexpectedEof`]: std::io::ErrorKind::UnexpectedEof
	/// [`Interrupted`]: std::io::ErrorKind::Interrupted
	pub fn read_bytes(&mut self, mut reader: impl Read, amount: usize) -> Result<&[u8], io::Error> {
		if self.buffer.len() < amount {
			self.buffer.extend(
				iter::repeat(0)
					.take(amount - self.buffer.len())
			);
		}
		
		let buffer = &mut self.buffer[..amount];
		reader.read_exact(buffer)?;
		
		Ok(buffer)
	}
}

impl Default for DynReadBuffer {
	fn default() -> Self {
		Self::new()
	}
}