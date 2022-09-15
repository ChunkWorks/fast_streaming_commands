use std::convert::TryInto;

pub struct CommandParser<T>
where
    T: FnMut(&[u8])
{
    callback: T,
    // We allocate a buffer which we clear after every command has been parsed successfully.
    // This keeps memory clean and free in case we don't need to receive larger packets such as LOGIN challenge signatures.
    pub buffer: Vec<u8>,
    length_buffer: [u8; 4],
    read_first_length_idx: u8,
    pub read_until_length: u32,
}

impl<T> CommandParser<T>
where
    T: FnMut(&[u8])
{
    pub fn new(callback: T) -> CommandParser<T> {
        CommandParser {
            callback,
            buffer: Vec::<u8>::new(),
            length_buffer: [0u8; 4],
            read_until_length: 0,
            read_first_length_idx: 0
        }
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.read_until_length = 0;
        self.read_first_length_idx = 0;
    }

    pub fn parse(&mut self, buf: &[u8]) {
        for byte in buf {
            if self.read_first_length_idx < 4 {
                self.length_buffer[self.read_first_length_idx as usize] = *byte;
                self.read_first_length_idx += 1;
                continue;
            }
            if self.read_until_length == 0 {
                self.read_until_length = u32::from_le_bytes(self.length_buffer);
            }
            let mut buf_len: u32 = self.buffer.len().try_into().unwrap();
            if self.read_until_length > buf_len {
                // Fill the buffer until we have filled it to the size requested
                self.buffer.push(*byte);
                buf_len += 1;
            }
            if self.read_until_length == buf_len {
                (self.callback)(&self.buffer);
                self.clear();
            }
        }
    }
}
