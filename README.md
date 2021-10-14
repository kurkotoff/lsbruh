# LSBRUH, an LSB steganography tool written in Rust

If you want to try hiding a message inside a picture - this tool is the right tool for you. Here's the functions of this tool:

- [x] Hide messages in image files (.png)
- [x] TODO: Extract messages hidden in other files
- [ ] TODO: Can try to bruteforce the bits that contain the hidden message if you do no know, which ones are they
- [ ] TODO: Supports more file formats

## Usage:

### Write a message into a file
```
lsbruh write container.png -f message.txt -o stego.png
```

### Read LSB from a stego file
```
lsbruh read stego.png -o output.txt
```
