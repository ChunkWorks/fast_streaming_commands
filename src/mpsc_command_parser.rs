use std::convert::TryInto;
use std::future::Future;
use tokio::sync::{mpsc};

pub struct MPSCCommandParser
{
    callback: mpsc::Sender<Vec<u8>>,
    // We allocate a buffer which we clear after every command has been parsed successfully.
    // This keeps memory clean and free in case we don't need to receive larger packets such as LOGIN challenge signatures.
    pub buffer: Vec<u8>,
    length_buffer: [u8; 4],
    read_first_length_idx: u8,
    pub read_until_length: u32,
}

impl MPSCCommandParser
{
    pub fn new(callback: mpsc::Sender<Vec<u8>>) -> MPSCCommandParser {
        MPSCCommandParser {
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

    pub async fn parse(&mut self, buf: &[u8]) -> Result<(), &str> {
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
                match self.callback.send(self.buffer.clone()).await {
                    Err(_) => {
                      return Err("Callback channel died");
                    }
                    Ok(_) => {

                    }
                }
                self.clear();
            }
        }
        return Ok(());
    }
}
