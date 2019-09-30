pub struct Reader {
    bytes: Vec<u8>,
    index: usize,
}

impl Reader {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            index: 0,
        }
    }

    pub fn eof(&self) -> bool {
        self.index >= self.bytes.len()
    }

    pub fn peek_byte(&self) -> u8 {
        if self.eof() {
            return 255;
        }
        self.bytes[self.index]
    }

    pub fn next_byte(&mut self) -> u8 {
        let next_byte = self.bytes[self.index];
        self.index = self.index + 1;
        next_byte
    }

    pub fn next_u16(&mut self) -> u16 {
        ((self.next_byte() as u16) << 8 | self.next_byte() as u16) as u16
    }

    pub fn next_i32(&mut self) -> i32 {
        ((self.next_u16() as i32) << 16 | self.next_u16() as i32) as i32
    }

    pub fn next_bool(&mut self) -> bool {
        self.next_byte() != 0
    }

    pub fn offset(&self) -> usize {
        self.index
    }
}
