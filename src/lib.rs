//! This crate provides [ReadBuffer] and [DynReadBuffer],
//! two wrappers to safely read into a buffer from a [`Read`].
//! 
//! # Motivation
//! 
//! With the default way of reading into a buffer using [Read::read][`Read_read`] like this:
//! ```
//! # fn main() -> Result<(), std::io::Error> {
//! use std::io::Read;
//! 
//! let mut reader = [1, 2, 3, 4].as_slice(); // Read is implemented for &[u8]
//! let mut buffer = [0; 16];
//! 
//! let length = reader.read(&mut buffer)?;
//! assert_eq!(buffer[..length], [1, 2, 3, 4]);
//! # Ok(())
//! # }
//! ```
//! there's nothing stopping you from accessing more data of the buffer than what was read
//! or even outright ignoring the [Result] of [Read::read][`Read_read`]:
//! ```
//! use std::io::Read;
//! 
//! let mut reader = [8, 8, 8, 8].as_slice();
//! let mut buffer = [0; 8];
//! 
//! // Ignoring the result of Read::read which might fail
//! # #[allow(unused)]
//! reader.read(&mut buffer);
//! 
//! // Reading too much data
//! assert_eq!(buffer, [8, 8, 8, 8, 0, 0, 0, 0]);
//! 
//! let mut reader = [1, 2, 3].as_slice();
//! 
//! # #[allow(unused)]
//! reader.read(&mut buffer);
//! 
//! // Reading garbage data from previous call to Read::read
//! assert_eq!(buffer[..4], [1, 2, 3, 8]);
//! ```
//! 
//! [ReadBuffer] and [DynReadBuffer] provide wrappers
//! that only let you access the data that was actually read,
//! and force you to check the [Result] before accessing the data.
//! 
//! # Examples
//! 
//! ```
//! # fn main() -> Result<(), std::io::Error> {
//! use read_buffer::ReadBuffer;
//! 
//! let mut reader = [8, 8, 8, 8].as_slice();
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
//! let mut reader = [1, 2, 3].as_slice();
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
//! 
//! [`Read`]: std::io::Read
//! [`Read_read`]: std::io::Read::read

#![deny(missing_docs)]

mod read_buffer;
mod dyn_read_buffer;

pub use self::read_buffer::ReadBuffer;
pub use self::dyn_read_buffer::DynReadBuffer;