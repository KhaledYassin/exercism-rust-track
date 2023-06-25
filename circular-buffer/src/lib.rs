pub struct CircularBuffer<T> {
    capacity: usize,
    read_index: usize,
    write_index: usize,
    buffer: Vec<Option<T>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            read_index: 0,
            write_index: 0,
            buffer: vec![None; capacity],
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            Err(Error::FullBuffer)
        } else {
            self.buffer[self.write_index % self.capacity] = Some(element);
            self.write_index += 1;
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            let value = self.take(self.read_index % self.capacity);
            self.read_index += 1;

            if self.write_index >= self.capacity {
                while self.write_index % self.capacity != 0 {
                    self.write_index += 1;
                }
            }

            Ok(value.unwrap())
        }
    }

    fn take(&mut self, index: usize) -> Option<T> {
        std::mem::replace(&mut self.buffer[index], None)
    }

    pub fn clear(&mut self) {
        (0..self.capacity).into_iter().for_each(|i| {
            self.take(i);
        });
        self.read_index = 0;
        self.write_index = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.is_full() {
            self.replace_oldest_with_element(element);
            self.read_index += 1;
        } else {
            let result = self.write(element);
            match result {
                Ok(_) => {}
                Err(error) => panic!("Overwrite calling Write threw an error: {:?}", error),
            }
        }
    }

    fn is_full(&self) -> bool {
        self.write_index >= self.capacity && self.buffer[self.write_index % self.capacity].is_some()
    }

    fn is_empty(&self) -> bool {
        self.buffer[self.read_index % self.capacity].is_none()
    }

    fn replace_oldest_with_element(&mut self, element: T) -> Option<T> {
        std::mem::replace(
            &mut self.buffer[self.read_index % self.capacity],
            Some(element),
        )
    }
}
