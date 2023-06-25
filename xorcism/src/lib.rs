use std::{
    borrow::Borrow,
    io::{Read, Write},
    iter::Cycle,
    slice::Iter,
};

#[derive(Clone)]
/// A munger which XORs a key with some data
pub struct Xorcism<'a> {
    // This field is just to suppress compiler complaints;
    // feel free to delete it at any point.
    key: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        Xorcism {
            key: key.as_ref().iter().cycle(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            .for_each(|byte| *byte ^= self.key.next().unwrap())
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<T: Borrow<u8>>(
        &mut self,
        data: impl IntoIterator<Item = T> + 'static,
    ) -> impl Iterator<Item = u8> + 'a {
        let mut munged = vec![];

        for b in data.into_iter() {
            munged.push(b.borrow() ^ self.key.next().unwrap())
        }

        munged.into_iter()
    }

    pub fn reader(self, buffer: impl Read + 'a) -> impl Read + 'a {
        XorReader {
            munger: self,
            buffer,
        }
    }

    pub fn writer(self, buffer: impl Write + 'a) -> impl Write + 'a {
        XorWriter {
            munger: self,
            buffer,
        }
    }
}

struct XorReader<'a, R> {
    munger: Xorcism<'a>,
    buffer: R,
}

impl<'a, R> Read for XorReader<'a, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.buffer.read(buf)?;

        self.munger.munge_in_place(buf);

        Ok(size)
    }
}

struct XorWriter<'a, W> {
    munger: Xorcism<'a>,
    buffer: W,
}

impl<'a, W> Write for XorWriter<'a, W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let vector = buf.to_vec();

        let stream = self.munger.munge(vector).collect::<Vec<_>>();

        self.buffer.write(&stream)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}
