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

#[cfg(test)]
mod tests {
    use std::io::{IoSlice, Write};

    use crate::vec::Vec;

    #[test]
    fn test_write() {
        let mut stdvec: std::vec::Vec<u8> = vec![2, 1, 3];
        let mut cratevec: Vec<u8> = Vec::from(vec![2, 1, 3]);

        assert_eq!(
            stdvec.write(b"hello, world!").unwrap(),
            cratevec.write(b"hello, world!").unwrap()
        );

        assert_eq!(stdvec, cratevec);
    }

    #[test]
    fn test_write_vectored() {
        let mut stdvec: std::vec::Vec<u8> = vec![2, 1, 3];
        let mut cratevec: Vec<u8> = Vec::from(vec![2, 1, 3]);

        assert_eq!(
            stdvec.write_vectored(&[IoSlice::new(&[2, 1, 3])]).unwrap(),
            cratevec
                .write_vectored(&[IoSlice::new(&[2, 1, 3])])
                .unwrap()
        );

        assert_eq!(stdvec, cratevec);
    }

    #[test]
    fn test_write_all() {
        let mut stdvec: std::vec::Vec<u8> = vec![2, 1, 3];
        let mut cratevec: Vec<u8> = Vec::from(vec![2, 1, 3]);

        assert_eq!(
            stdvec.write_all(b"hello, world!").unwrap(),
            cratevec.write_all(b"hello, world!").unwrap()
        );

        assert_eq!(stdvec, cratevec);
    }
}
