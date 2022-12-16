const MAX_DATA_SIZE: usize = 650;

pub struct Context {
    pub signing_context: SigningContext,
}

impl Context {
    pub fn new() -> Self {
        Context {
            signing_context: SigningContext::new(),
        }
    }
}

pub struct SigningContext {
    pub buffer: DataBuffer,
    pub bip32: [u32; 5],
    pub amount_decimals: u8,
    pub fee_decimals: u8,
}

impl SigningContext {
    pub fn new() -> Self {
        SigningContext {
            buffer: DataBuffer::new(),
            bip32: [0u32; 5],
            amount_decimals: 0,
            fee_decimals: 0,
        }
    }
}

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

    pub fn length(&self) -> usize {
        self.cursor
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.cursor]
    }

    pub fn push(&mut self, data: &[u8]) {
        let length = data.len();
        self.buffer[self.cursor..self.cursor + length].clone_from_slice(data);
        self.cursor += length;
    }

    pub fn clean(&mut self) {
        self.buffer = [0u8; MAX_DATA_SIZE];
        self.cursor = 0;
    }
}
