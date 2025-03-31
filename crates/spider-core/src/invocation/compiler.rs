//! The compiler trait is responsible for taking source spider files and making it usable

use futures::{AsyncRead, AsyncReadExt, AsyncSeek, ready};
use pin_project::pin_project;
use std::io;
use std::io::{Read, SeekFrom};
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Reader: AsyncRead + AsyncSeek + Unpin + Send + Sync {
    fn read_to_end<'a>(&'a mut self, buf: &'a mut Vec<u8>) -> ReadToEnd<'a>;
}

#[pin_project]
pub struct ReadToEnd<'a> {
    reader: &'a mut dyn Reader,
    bytes: &'a mut Vec<u8>,
    bytes_read: usize,
}

impl Future for ReadToEnd<'_> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        let mut pinned = Pin::new(&mut **me.reader);
        loop {
            let mut buffer = vec![0; 1024];
            let ready = ready!(pinned.as_mut().poll_read(cx, &mut buffer));
            match ready {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    *me.bytes_read += n;
                    me.bytes.extend(&buffer[..n]);
                }
                Err(e) => {
                    return Poll::Ready(Err(e));
                }
            }
        }
        Poll::Ready(Ok(*me.bytes_read))
    }
}

impl Reader for Box<dyn Reader + '_> {
    fn read_to_end<'a>(&'a mut self, buf: &'a mut Vec<u8>) -> ReadToEnd<'a> {
        (**self).read_to_end(buf)
    }
}

/// As [`AsyncRead`] implementation that can read a vector
pub struct VecReader {
    buffer: Vec<u8>,
    read_bytes: usize,
}

impl VecReader {
    pub fn new(vec: Vec<u8>) -> Self {
        Self {
            buffer: vec,
            read_bytes: 0,
        }
    }
}

impl AsyncRead for VecReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        if self.read_bytes >= self.buffer.len() {
            Poll::Ready(Ok(0))
        } else {
            let n = ready!(Pin::new(&mut &self.buffer[self.read_bytes..]).poll_read(cx, buf))?;
            self.read_bytes += n;
            Poll::Ready(Ok(n))
        }
    }
}

impl AsyncSeek for VecReader {
    fn poll_seek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: SeekFrom,
    ) -> Poll<io::Result<u64>> {
        todo!()
    }
}

impl Reader for VecReader {
    fn read_to_end<'a>(&'a mut self, buf: &'a mut Vec<u8>) -> ReadToEnd<'a> {
        ReadToEnd {
            reader: self,
            bytes: buf,
            bytes_read: 0,
        }
    }
}

pub trait File {}

/// responsible for loading files
pub trait FileLoader: Send + Sync + 'static {
    type File: File;

    fn load(&self, reader: &dyn Reader) -> impl Future<Output = Self::File>;

    /// File type extensions to look for
    fn extensions(&self) -> &[&'static str];
}

pub trait Compiler {}

#[cfg(test)]
mod tests {
    use crate::invocation::compiler::{Reader, VecReader};

    #[tokio::test]
    async fn test_read_to_end() {
        let mut vec_reader = VecReader::new(Vec::from(b"Hello World!"));
        let mut buffer = vec![];
        vec_reader.read_to_end(&mut buffer).await.unwrap();
        assert_eq!(buffer, b"Hello World!");
    }
}
