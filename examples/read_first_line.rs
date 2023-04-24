// In this example we're trying to read just the first line
// from a reader (the file some_text.txt). To be efficient,
// we don't want any calls to Read::read to occur after we
// encounter the first '\n', and we want all of the bytes to
// be read into one large buffer that we allocate in the
// beginning.

use std::{cmp, io, str};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use read_buffer::ReadBuffer;

// We create an adapter over a Read to emulate the behavior
// that a call to read may read less bytes than the length
// of the given buffer. This is explicitly allowed in the
// documentation for Read::read.
// (https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read)
struct ChunkedReadAdapter<T: Read> (T);

impl<T: Read> Read for ChunkedReadAdapter<T> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		let bytes_to_read = cmp::min(7, buf.len());
		self.0.read(&mut buf[..bytes_to_read])
	}
}

fn main() -> Result<(), io::Error> {
	// Get the libraries root directory
	let mut path: PathBuf = env!("CARGO_MANIFEST_DIR").into();
	
	// Get the example text file
	path.push("examples");
	path.push("some_text.txt");
	
	// Open the file
	let file = File::open(path)?;
	
	// Create the adapter emulating a reader that
	// returns 5 bytes at a time
	let mut reader = ChunkedReadAdapter(file);
	
	// Create the buffer to read into with a size large enough
	// to hold the entire line
	let mut buffer: ReadBuffer<512> = ReadBuffer::new();
	
	// Read into the buffer while the condition is true
	// (or until the buffer is full, "end of file" is
	// encountered, or an error occurs)
	let read_data = buffer.read_while(&mut reader, |chunk| {
		// Keep reading until we encounter a '\n'
		!chunk.contains(&b'\n')
	})?;
	
	// Is safe to use as the file contains only ASCII characters
	let string = str::from_utf8(read_data)
		.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
	
	// We read some extra data as part of the chunk which contained '\n'
	// so we'll still have to cut the string off after the first line
	let first_line = string.lines().next()
		.ok_or(io::Error::from(io::ErrorKind::InvalidData))?;
	
	// Print out the first line
	println!("First line: {:?}", first_line);
	
	// Print out all data that was read to show that it didn't
	// stop precisely at the '\n' but it didn't read too much
	// data after the end of the line
	println!("All read data: {:?}", string);
	
	Ok(())
}