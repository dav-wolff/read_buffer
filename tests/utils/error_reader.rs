use std::io;
use std::io::{ErrorKind, Read};

pub struct ErrorReader;

impl Read for ErrorReader {
	fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
		Err(ErrorKind::NotFound.into())
	}
}