pub struct Buffer<'a> {
    buffer: &'a [u8],
}

impl<'a> Buffer<'a> {
    pub fn new(bytes: &[u8]) -> Buffer {
        Buffer { buffer: bytes }
    }

    pub fn get_byte(self: &mut Buffer<'a>, value: &mut u8) -> Buffer {
        match &self.buffer.first() {
            Some(byte) => {
                *value = **byte;

                Buffer {
                    buffer: &self.buffer[1..],
                }
            }
            None => Buffer {
                buffer: self.buffer,
            },
        }
    }

    pub fn get_bool(self: &mut Buffer<'a>, value: &mut bool) -> Buffer {
        match &self.buffer.first() {
            Some(byte) => {
                if **byte == 0u8 {
                    *value = false;
                } else if **byte == 1u8 {
                    *value = true;
                }

                Buffer {
                    buffer: &self.buffer[1..],
                }
            }
            None => Buffer {
                buffer: self.buffer,
            },
        }
    }

    pub fn get_bytes(self: &mut Buffer<'a>, value: &mut [u8], size: usize) -> Buffer {
        match &self.buffer.get(0..size) {
            Some(bytes) => {
                value.clone_from_slice(bytes);

                Buffer {
                    buffer: &self.buffer[size..],
                }
            }
            None => Buffer {
                buffer: self.buffer,
            },
        }
    }

    pub fn get_bytes_flag(self: &mut Buffer<'a>, value: &mut [u8], size: usize) -> Buffer {
        match &self.buffer.first() {
            Some(flag) => {
                if **flag == 0u8 {
                    Buffer {
                        buffer: &self.buffer[1..],
                    }
                } else if **flag == 1u8 {
                    match &self.buffer.get(1..size + 1) {
                        Some(bytes) => {
                            value.clone_from_slice(bytes);

                            Buffer {
                                buffer: &self.buffer[size + 1..],
                            }
                        }
                        None => Buffer {
                            buffer: self.buffer,
                        },
                    }
                } else {
                    Buffer {
                        buffer: self.buffer,
                    }
                }
            }
            None => Buffer {
                buffer: self.buffer,
            },
        }
    }
}
