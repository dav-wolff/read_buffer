use std::io;
use std::str;
use std::fs::File;
use std::path::PathBuf;
use read_buffer::ReadBuffer;

fn main() -> Result<(), io::Error> {
	// Get the libraries root directory
	let mut path: PathBuf = env!("CARGO_MANIFEST_DIR").into();
	
	// Get the example text file
	path.push("examples");
	path.push("some_text.txt");
	
	// Open the file
	let mut file = File::open(path)?;
	
	// Create the buffer to read into
	let mut buffer: ReadBuffer<8> = ReadBuffer::new();
	
	loop {
		// Read up to 8 bytes from the file
		let read_data = buffer.read_from(&mut file)?;
		
		// A length of 0 indicates 'end of file'
		if read_data.len() == 0 {
			println!("Reached end of file");
			return Ok(());
		}
		
		// Is safe to use as the file contains only ASCII characters
		let string = str::from_utf8(read_data)
			.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
		
		// Print out the bytes read in hexadecimal representation
		// and interpreted as ASCII
		println!("Read some data: {:02x?}: {:?}", read_data, string);
	}
}