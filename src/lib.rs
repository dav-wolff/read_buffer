//! This crate provides [ReadBuffer], a wrapper to safely read into a buffer from a [Read].
//! 
//! # Motivation
//! 
//! With the default way of reading into a buffer using [Read::read] like this:
//! ```
//! # fn main() -> Result<(), std::io::Error> {
//! use std::io::Read;
//! 
//! let data = [1, 2, 3, 4];
//! let mut reader = &data[..]; // Read is implemented for &[u8]
//! let mut buffer = [0; 16];
//! 
//! let length = reader.read(&mut buffer)?;
//! assert_eq!(buffer[..length], [1, 2, 3, 4]);
//! # Ok(())
//! # }
//! ```
//! there's nothing stopping you from accessing more data of the buffer than what was read
//! or even outright ignoring the [Result] of [Read::read]:
//! ```
//! use std::io::Read;
//! 
//! let data = [8, 8, 8, 8];
//! let mut reader = &data[..];
//! let mut buffer = [0; 8];
//! 
//! // Ignoring the result of Read::read which might fail
//! # #[allow(unused)]
//! reader.read(&mut buffer);
//! 
//! // Reading too much data
//! assert_eq!(buffer, [8, 8, 8, 8, 0, 0, 0, 0]);
//! 
//! let data = [1, 2, 3];
//! let mut reader = &data[..];
//! 
//! # #[allow(unused)]
//! reader.read(&mut buffer);
//! 
//! // Reading garbage data from previous call to Read::read
//! assert_eq!(buffer[..4], [1, 2, 3, 8]);
//! ```
//! 
//! [ReadBuffer] provides a wrapper that only lets you access the data that was actually read,
//! and forces you to check the [Result] before accessing the data.
//! 
//! # Examples
//! 
//! ```
//! # fn main() -> Result<(), std::io::Error> {
//! use read_buffer::ReadBuffer;
//! 
//! let data = [8, 8, 8, 8];
//! let mut reader = &data[..];
//! let mut buffer: ReadBuffer<8> = ReadBuffer::new();
//! 
//! // We are forced to check the Result of read_from to access the data we read
//! let read_data = buffer.read_from(&mut reader)?;
//! 
//! // read_data is a slice over only the data we actually read,
//! // trying to access the buffer past that point would panic
//! let eight = read_data[3];
//! // let zero = read_data[4]; would panic
//! 
//! assert_eq!(eight, 8);
//! assert_eq!(read_data, [8, 8, 8, 8]);
//! 
//! // We can reuse the same buffer for the next read, just as with Read::read
//! 
//! let data = [1, 2, 3];
//! let mut reader = &data[..];
//! 
//! let read_data = buffer.read_from(&mut reader)?;
//! 
//! // Again, we get a slice over only the data that was just read,
//! // trying to read garbage data from the previous call to read_from
//! // here would panic
//! let three = read_data[2];
//! // let eight = read_data[3]; would panic
//! 
//! assert_eq!(three, 3);
//! assert_eq!(read_data, [1, 2, 3]);
//! # Ok(())
//! # }
//! ```

#![deny(missing_docs)]

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
		ReadBuffer {
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