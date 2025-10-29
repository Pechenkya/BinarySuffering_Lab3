use crate::BitStream::BitStream;

struct Node {
    weight: u8,
    byte_value: Option<u8>,
    parent: Option<Box<Node>>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct HuffmanEncoder {
    freq_t: [u32; 256],
    root: Option<Box<Node>>,
    input_stream: BitStream,
    output_stream: BitStream,
}

pub struct HuffmanDecoder {
    freq_t: [u32; 256],
    root: Option<Box<Node>>,
    input_stream: BitStream,
    output_stream: BitStream,
}

impl HuffmanEncoder {
    fn calc_frequences(&mut self) {
        while let Ok(byte_arr) = self.input_stream.read_bit_sequence(8) {
            self.freq_t[byte_arr[0] as usize] += 1;
        }

        self.input_stream.rewind_read_stream().unwrap();
    }

    fn build_tree_and_get_codes(&mut self) {
        let mut queue: Vec<Box<Node>> = Vec::new();
        for (i, &freq) in self.freq_t.iter().enumerate() {
            queue.push(Box::new(Node {
                weight: freq as u8,
                byte_value: Some(i as u8),
                parent: None,
                left: None,
                right: None,
            }));
        }

        // Build Huffman tree
        while queue.len() > 1 {
            queue.sort_by_key(|node| node.weight);
            let left = queue.remove(0);
            let right = queue.remove(0);

            let parent = Box::new(Node {
                weight: left.weight + right.weight,
                byte_value: None,
                parent: None,
                left: Some(left),
                right: Some(right),
            });

            queue.push(parent);
        }

        self.root = Some(queue.remove(0));

        // Traverse tree to get codes
        let mut codes: [(u8, u8); 256] = [(0, 0); 256];
        let mut stack: Vec<&Node> = Vec::new();
        stack.push(self.root.as_ref().unwrap());
        let mut code: u8 = 0;
        let mut code_length: u8 = 0;


        let mut ptr = self.root.as_ref();
        

        while ptr.is_some() {
            let node = ptr.unwrap();
            if let Some(byte_value) = node.byte_value {
                codes[byte_value as usize] = (code, code_length);
            }

            ptr = ptr.unwrap().left.as_ref();
            while Some() {
                
            }

        }
    }

    pub fn encode(mut self, input: &str, output: &str) {
        self.input_stream = BitStream::new(input, true);
        self.output_stream = BitStream::new(output, false);
        self.calc_frequences();
        self.build_tree_and_get_codes();
        
        // Write frequency table to output
        unsafe {
            let (_, aligned, _) = self.freq_t.align_to::<u8>();
            self.output_stream.write_bit_sequence(&aligned, 1024).unwrap();
        }
        
        // Encode all bytes
        while let Ok(byte_arr) = self.input_stream.read_bit_sequence(8) {

        }


    }
}