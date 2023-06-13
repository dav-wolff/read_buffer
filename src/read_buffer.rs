use std::io;
use std::io::Read;

/// A buffer to read into from a [Read] and safely access the read data.
/// 
/// **ReadBuffer** provides a buffer to read into using [ReadBuffer::read_from],
/// but crucially doesn't allow **any** access to the data inside the buffer
/// outside of the slice returned from [ReadBuffer::read_from].
#[derive(Debug)]
pub struct ReadBuffer<const SIZE: usize> {
	buffer: [u8; SIZE],
}

impl<const SIZE: usize> ReadBuffer<SIZE> {
	/// Creates a new **ReadBuffer**
	pub fn new() -> Self {
		Self {
			buffer: [0u8; SIZE],
		}
	}
	
	/// Reads from the given [Read] into the internal buffer
	/// and returns a slice referencing the read data
	/// or an error if any occurred.
	/// 
	/// If the length of the returned slice is `0`,
	/// this indicates that the reader has reached its "end of file"
	/// as specified for [Read::read].  
	/// (Unless this method is called on a `ReadBuffer<0>`)
	/// 
	/// # Errors
	/// 
	/// Errors from [Read::read] are passed on to the caller.
	/// Besides those, this method does not return any errors.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # fn main() -> Result<(), std::io::Error> {
	/// use read_buffer::ReadBuffer;
	/// 
	/// let data = [1, 2, 3, 4];
	/// let mut reader = &data[..]; // Read is implemented for &[u8]
	/// let mut buffer: ReadBuffer<256> = ReadBuffer::new();
	/// 
	/// let read_data = buffer.read_from(&mut reader)?;
	/// 
	/// assert_eq!(read_data, [1, 2, 3, 4]);
	/// # Ok(())
	/// # }
	/// ```
	pub fn read_from(&mut self, source: &mut impl Read) -> Result<&[u8], io::Error> {
		let length = source.read(&mut self.buffer)?;
		Ok(&self.buffer[..length])
	}
	
	/// Continually calls [Read::read] on the given [Read] as long
	/// as predicate returns true, filling the internal buffer,
	/// and returns a slice referencing all the data read over all
	/// the calls made to [Read::read] or an error if any occurred.
	/// 
	/// This function takes a predicate that is called with each
	/// chunk of data read from [Read::read] and that decides
	/// whether to keep reading.
	/// 
	/// The predicate is **not** called with an empty slice if
	/// the call to [Read::read] returns a length of 0.
	/// 
	/// This function keeps calling [Read::read] on the given [Read]
	/// until one of the following occurs:
	/// 
	/// 1. The predicate returns `false`.
	/// 1. The internal buffer is full.
	/// 1. The call to [Read::read] returns a length of 0 indicating "end of file".
	/// 1. The call to [Read::read] returns an error.
	/// 
	/// # Errors
	/// 
	/// Errors from [Read::read] are passed on to the caller.
	/// Besides those, this method does not return any errors.
	pub fn read_while(&mut self, source: &mut impl Read, mut predicate: impl FnMut(&[u8]) -> bool) -> Result<&[u8], io::Error> {
		let mut remaining = &mut self.buffer[..];
		
		loop {
			let length = source.read(remaining)?;
			
			if length == 0 {
				break;
			}
			
			let chunk: &mut [u8];
			(chunk, remaining) = remaining.split_at_mut(length);
			
			if !predicate(chunk) || remaining.is_empty() {
				break;
			}
		}
		
		let read_bytes = SIZE - remaining.len();
		Ok(&self.buffer[..read_bytes])
	}
	
	/// Returns the capacity of the internal buffer
	/// which was set using the const generic.
	/// 
	/// This can be useful when checking whether a call to [Read::read]
	/// filled the buffer completely or stopped reading early.  
	/// Using `capacity` in this case avoids having to repeat the capacity
	/// and possibly forgetting to update it later on.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # fn main() -> Result<(), std::io::Error> {
	/// use read_buffer::ReadBuffer;
	/// 
	/// let data = [1, 2, 3, 4, 5, 6, 7];
	/// let mut reader = &data[..]; // Read is implemented for &[u8]
	/// let mut buffer: ReadBuffer<4> = ReadBuffer::new();
	/// 
	/// let read_data = buffer.read_from(&mut reader)?;
	/// 
	/// assert_eq!(read_data.len(), buffer.capacity());
	/// 
	/// let read_data = buffer.read_from(&mut reader)?;
	/// 
	/// assert_ne!(read_data.len(), buffer.capacity());
	/// # Ok(())
	/// # }
	/// ```
	pub const fn capacity(&self) -> usize {
		SIZE
	}
}

impl<const SIZE: usize> Default for ReadBuffer<SIZE> {
	fn default() -> Self {
		Self::new()
	}
}