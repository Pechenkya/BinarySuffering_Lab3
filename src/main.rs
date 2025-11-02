#![allow(dead_code, non_snake_case, unused_imports, unused_variables)]
mod BitStream;
mod Huffman;

fn main() {
    Huffman::HuffmanEncoder::encode("test_file.txt", "encoded_file.huff");
    Huffman::HuffmanEncoder::encode("test_pdf.pdf", "encoded_pdf.huff");
    Huffman::HuffmanEncoder::encode("test_pdf_2.pdf", "encoded_pdf_2.huff");
    Huffman::HuffmanDecoder::decode("encoded_file.huff", "decoded_file.txt");
    Huffman::HuffmanDecoder::decode("encoded_pdf_2.huff", "decoded_pdf_2.pdf");
}
