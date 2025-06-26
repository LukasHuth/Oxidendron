# Oxidendron

This is just a simple CLI tool for huffman encoding/decoding

## Usage

### Encoding
```sh
oxidendron encode file_to_encode.txt > output.txt
```
or
```sh
oxidendron encode file_to_encode.txt -o output.txt
```
or
```sh
oxidendron encode file_to_encode.txt --output output.txt
```

### Decoding
```sh
oxidendron decode file_to_decode.txt > output.txt
```
or
```sh
oxidendron decode file_to_decode.txt -o output.txt
```
or
```sh
oxidendron decode file_to_decode.txt --output output.txt
```

### Version
```sh
oxidendron version
```

### Help page
```sh
oxidendron help
```

## Installation

### With Repository
```sh
git clone git@github.com:LukasHuth/Oxidendron.git
pushd Oxidendron
cargo install --path .
popd
```
