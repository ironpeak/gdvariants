use std::{
    fmt::Arguments,
    io::{IoSlice, Result, Write},
};

use crate::vec::Vec;

/// Write is implemented for `Vec<u8>` by appending to the vector.
/// The vector will grow as needed.
impl Write for Vec<u8> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.base.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize> {
        self.base.write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.base.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<()> {
        self.base.write_fmt(fmt)
    }

    fn flush(&mut self) -> Result<()> {
        self.base.flush()
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }
}
