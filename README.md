# read_buffer

This crate provides **ReadBuffer**, a wrapper to safely read into a buffer from a [Read].

## Motivation

With the default way of reading into a buffer using [Read::read] like this:
```rust
use std::io::Read;

let data = [1, 2, 3, 4];
let mut reader = &data[..]; // Read is implemented for &[u8]
let mut buffer = [0; 16];

let length = reader.read(&mut buffer)?;
assert_eq!(buffer[..length], [1, 2, 3, 4]);
```
there's nothing stopping you from accessing more data of the buffer than what was read
or even outright ignoring the [Result] of [Read::read]:
```rust
use std::io::Read;

let data = [8, 8, 8, 8];
let mut reader = &data[..];
let mut buffer = [0; 8];

// Ignoring the result of Read::read which might fail
# #[allow(unused)]
reader.read(&mut buffer);

// Reading too much data
assert_eq!(buffer, [8, 8, 8, 8, 0, 0, 0, 0]);

let data = [1, 2, 3];
let mut reader = &data[..];

# #[allow(unused)]
reader.read(&mut buffer);

// Reading garbage data from previous call to Read::read
assert_eq!(buffer[..4], [1, 2, 3, 8]);
```

**ReadBuffer** provides a wrapper that only lets you access the data that was actually read,
and forces you to check the [Result] before accessing the data.

## Examples

```rust
use read_buffer::ReadBuffer;

let data = [8, 8, 8, 8];
let mut reader = &data[..];
let mut buffer: ReadBuffer<8> = ReadBuffer::new();

// We are forced to check the Result of read_from to access the data we read
let read_data = buffer.read_from(&mut reader)?;

// read_data is a slice over only the data we actually read,
// trying to access the buffer past that point would panic
let eight = read_data[3];
// let zero = read_data[4]; would panic

assert_eq!(eight, 8);
assert_eq!(read_data, [8, 8, 8, 8]);

// We can reuse the same buffer for the next read, just as with Read::read

let data = [1, 2, 3];
let mut reader = &data[..];

let read_data = buffer.read_from(&mut reader)?;

// Again, we get a slice over only the data that was just read,
// trying to read garbage data from the previous call to read_from
// here would panic
let three = read_data[2];
// let eight = read_data[3]; would panic

assert_eq!(three, 3);
assert_eq!(read_data, [1, 2, 3]);
```

[Read]: https://doc.rust-lang.org/std/io/trait.Read.html
[Read::read]: https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read
[Result]: https://doc.rust-lang.org/core/result/enum.Result.html