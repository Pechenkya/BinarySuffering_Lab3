use std::cmp::min;
use std::io::{Read, Seek, Write};
use std::path::Path;
use std::fs::{File, OpenOptions};

// Max size of buffer in buffered read (4KB)
const BUFF_MAX_BYTE_SIZE: usize = 4096;

fn create_error(message: &str) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, message))
}

pub fn bin_string_LSBF(bytes: &[u8]) -> String {
    let result: Vec<String> = bytes
        .iter()
        .map(|byte| format!("{:08b}", byte.reverse_bits()))
        .collect();
    
    return result.join(" ");
}

pub struct BitStream {
    buff: Vec<u8>,
    bit_pointer: usize,
    read_dir: bool,
    file: File,
    byte_chunk_size: usize
}

impl BitStream {
    pub fn new(file_path: &str, read_dir: bool) -> Self {
        let buff: Vec<u8>;
        
        let file_stream: File;
        if read_dir {
            file_stream = File::open(file_path).unwrap();
            buff = vec![0u8; BUFF_MAX_BYTE_SIZE];
        }
        else {
            file_stream = OpenOptions::new().read(true)
                                            .create(true)
                                            .write(true)
                                            .truncate(false)
                                            .open(file_path).unwrap();
            buff = Vec::new();
        }

        BitStream {
            buff: buff,
            bit_pointer: 0,
            read_dir: read_dir,
            file: file_stream,
            byte_chunk_size: 0
        }
    }

    pub fn clear_output_file(&self) -> Result<(), std::io::Error> {
        if !self.read_dir {
            // Truncate file data
            self.file.set_len(0)?;
            Ok(())
        }
        else {
            create_error("Cannot clear file in read mode")
        }
    }

    pub fn write_bit_sequence(&mut self, in_buff: &[u8], bit_len: usize) -> Result<(), std::io::Error> {
        if self.read_dir {
            return create_error("This BitStream is in read mode");
        }

        let basic_shift = self.bit_pointer % 8;
        let full_bytes_to_write = bit_len / 8;
        let remaining_bits = bit_len % 8;
        
        if basic_shift == 0 {
            // Move full bytes to the stream buffer
            for i in 0..full_bytes_to_write {
                self.buff.push(in_buff[i]);
            }
            
            // Handle remaining bits
            if remaining_bits != 0 {
                self.buff.push((in_buff[full_bytes_to_write] << (8 - remaining_bits)) >> (8 - remaining_bits));
            }
        }
        else {
            let mut last_byte_id = self.buff.len() - 1;

            // Move full bytes to the stream buffer
            for i in 0..full_bytes_to_write {
                // Append low bits to the last byte
                self.buff[last_byte_id] |= in_buff[i] << basic_shift;

                // Push high bits as a new byte
                self.buff.push(in_buff[i] >> (8 - basic_shift));
                last_byte_id += 1;
            }

            // Handle remaining bits
            if remaining_bits != 0 {
                if remaining_bits + basic_shift > 8 {
                    self.buff[last_byte_id] |= in_buff[in_buff.len() - 1] << basic_shift;
                    self.buff.push((in_buff[in_buff.len() - 1] << (8 - remaining_bits)) >> (16 - remaining_bits - basic_shift));
                }
                else {
                    self.buff[last_byte_id] |= (in_buff[in_buff.len() - 1] << (8 - remaining_bits)) >> (8 - remaining_bits - basic_shift);
                }
            }
        }
        self.bit_pointer += bit_len;

        // println!("Buffer after write (LSB-F): {}", bin_string_LSBF(&self.buff));
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        if self.read_dir {
            return create_error("BitStream cannot be flushed in read mode");
        }

        // println!("Buffer on flush (LSB-F): {}", bin_string_LSBF(&self.buff));

        self.file.write_all(&self.buff)?;
        self.file.flush()?;

        self.buff.clear();
        self.bit_pointer = 0;

        Ok(())
    }

    pub fn read_bit_sequence(&mut self, size: usize) -> Result<Vec<u8>, std::io::Error> {
        if !self.read_dir {
            return Err(create_error("This BitStream is in write mode").err().unwrap());
        }

        let mut result: Vec<u8> = Vec::new();
        let mut bits_read: usize = 0;

        while bits_read != size {
            // If chunk is empty -> read next
            if (self.byte_chunk_size * 8 - self.bit_pointer) == 0 {
                let bytes_read = self.file.read(&mut self.buff)?;
                if bytes_read == 0 {
                    // println!("Warning! Reached EOF for stream in read operation!");
                    return Ok(result);
                }
                self.byte_chunk_size = bytes_read;
                self.bit_pointer = 0;
            }

            let bit_chunk_size = self.byte_chunk_size * 8;
            let basic_shift = self.bit_pointer % 8;
            
            if basic_shift == 0 {
                // Easy case, move full bytes and fill bit remainder
                let bits_to_move = min(bit_chunk_size - self.bit_pointer, size - bits_read);

                // Move bytes
                let start_id = self.bit_pointer / 8;
                let end_id = (self.bit_pointer + bits_to_move) / 8;
                result.append(&mut self.buff[start_id..end_id].to_vec());
                
                // Move remainder bits
                let rem_bits = bits_to_move % 8;
                if rem_bits != 0 {
                    result.push((self.buff[end_id] << (8 - rem_bits)) >> (8 - rem_bits));
                }

                // Move pointers/counters
                bits_read += bits_to_move;
                self.bit_pointer += bits_to_move;
            }
            else {
                let mut byte_id = self.bit_pointer / 8;

                // Corner case of empty return vector
                if result.is_empty() {
                    result.push(0);
                    // result.push(self.buff[byte_id] >> basic_shift);
                    // byte_id += 1;
                    // bits_read += 8 - basic_shift;
                    // self.bit_pointer += 8 - basic_shift;
                }

                let mut bits_left = min(size - bits_read, bit_chunk_size - self.bit_pointer);
                let mut last_id = result.len() - 1;

                while bits_left > 8 {
                    // Move to high lower bits of the byte
                    result[last_id] |= self.buff[byte_id] << (8 - basic_shift);

                    // Move to low high bits of the byte
                    result.push(self.buff[byte_id] >> basic_shift);

                    // Next bit
                    bits_left -= 8;
                    byte_id += 1;
                    last_id += 1;

                    bits_read += 8;
                    self.bit_pointer += 8;
                }

                // Process tail bits
                if bits_left != 0
                {
                    if bits_left > 8 - basic_shift {
                        result[last_id] |= self.buff[byte_id] << (8 - basic_shift);
                        // Remainder has size b_l - b_s, to get it we need to shift left on 8 - b_l (clear high bits)
                        // And then to set it to low bits we need to move it on 8 - b_l + b_s (mattth)
                        result.push((self.buff[byte_id] << (8 - bits_left)) >> 8 - bits_left + basic_shift);
                    }
                    else 
                    {
                        // Set the last bits to the end of the buffer (clear high bits, then move to low on b_s - b_l (mathhh))
                        // result[last_id] |= ((self.buff[byte_id] << (8 - bits_left)) >> (8 - bits_left)) << basic_shift;
                        let _debug_byte_val = self.buff[byte_id];
                        result[last_id] |= ((self.buff[byte_id] >> basic_shift) << (8 - bits_left)) >> (8 - bits_left);
                    }
                    bits_read += bits_left;
                    self.bit_pointer += bits_left;
                }
            }
        }

        Ok(result)
    }

    pub fn rewind_read_stream(&mut self) -> Result<(), std::io::Error> {
        if !self.read_dir {
            return create_error("Cannot reset stream in write mode");
        }

        self.file.rewind()?;
        self.buff.clear();
        self.buff.resize(BUFF_MAX_BYTE_SIZE, 0u8);
        
        self.bit_pointer = 0;
        self.byte_chunk_size = 0;

        Ok(())
    }
}