#![allow(dead_code, non_snake_case, unused_imports, unused_variables)]
mod BitStream;
mod Huffman;

fn main() {
    // let mut encoder = Huffman::HuffmanEncoder::new();
    // let mut decoder = Huffman::HuffmanDecoder::new();

    Huffman::HuffmanEncoder::encode("test_file.txt", "encoded_file.huff");
    // Huffman::HuffmanEncoder::encode("test_xlsx.xlsx", "encoded_xlsx.huff");
    Huffman::HuffmanDecoder::decode("encoded_file.huff", "decoded_file.txt");
    // Huffman::HuffmanDecoder::decode("encoded_xlsx.huff", "decoded_xlsx.xlsx");
}
