# LSBRUH, an LSB steganography tool written in Rust

If you want to try hiding a message inside a picture - this tool is the right tool for you. Here's the functions of this tool:

- [x] Hide messages in image files (.png)
- [ ] TODO: Extract messages hidden in other files
- [ ] TODO: Can try to bruteforce the bits that contain the hidden message if you do no know, which ones are they
- [ ] TODO: Support more file formats

## Usage:
```
USAGE:
    lsbruh <FILE> --message <message> --output <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --message <message>    Message to hide
    -o, --output <output>      Output file

ARGS:
    <FILE>    Input file to process
```

## Example:
```
lsbruh hello.png -m "Hello There!" -o general_kenobi.png
```
