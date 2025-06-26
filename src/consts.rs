pub(crate) const HELP_MSG: &str = r#"
Usage:
  huffman [MODE] [ARGS...]

Modes:
  encode    Compress input using Huffman encoding
  decode    Decompress previously encoded input
  version   Display version information
  help      Show this help message

Examples:
  huffman encode input.txt > output.huff
  huffman decode output.huff > decoded.txt

Notes:
  - All modes expect relevant arguments (e.g., input files).
  - For detailed help on each mode, use: huffman help [MODE]
"#;
pub(crate) const DECODE_MSG: &str = r#"
Oxidendron - Huffman Decoding Utility
Usage:
    oxidendron decode <ENCODED_FILE> [OPTIONS]

Description:
    Decompresses a file previously encoded by Oxidendron and reconstructs the original content.

Arguments:
    <ENCODED_FILE>      The path to the encoded file

Options:
    -o, --output <FILE>     Specify the output file (default: stdout)

Example:
    oxidendron decode data.huff -o restored.txt
"#;
pub(crate) const ENCODE_MSG: &str = r#"
Oxidendron - Huffman Encoding Utility
Usage:
    oxidendron encode <INPUT_FILE> [OPTIONS]

Description:
    Compresses the input file using Huffman encoding and writes the encoded output.

Arguments:
    <INPUT_FILE>        The path to the file to encode.

Options:
    -o, --output <FILE>     Specify the output file (default: stdout)

Example:
    oxidendron encode data.txt -o data.huff
"#;
