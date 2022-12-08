const MAX_DATA_SIZE: usize = 650;

pub struct DataBuffer {
    buffer: [u8; MAX_DATA_SIZE],
    cursor: usize,
}

impl DataBuffer {
    pub fn new() -> Self {
        DataBuffer {
            buffer: [0u8; MAX_DATA_SIZE],
            cursor: 0,
        }
    }

    pub fn clean(&mut self) {
        self.buffer = [0u8; MAX_DATA_SIZE];
        self.cursor = 0;
    }

    pub fn push(&mut self, data: &[u8]) {
        let length = data.len();
        self.buffer[self.cursor..self.cursor + length].clone_from_slice(data);
        self.cursor += length;
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.cursor]
    }
}
