use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Config {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Write(WriteOptions),
}

#[derive(Debug, StructOpt)]
pub struct WriteOptions {
    /// Input file to process
    #[structopt(name = "FILE", parse(from_os_str))]
    pub secret: PathBuf,

    /// Message to hide
    #[structopt(
        short = "-f", 
        long = "--input-file",
        parse(from_os_str))]
    pub input_file: PathBuf,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    pub output: PathBuf,
}


// /*
//   Converts a u8 number into an array
//   of 0 and 1 with length 8

//   EXAMPLE:
//     123 => [0, 1, 1, 1, 1, 0, 1, 1]
//     10  => [0, 0, 0, 0, 1, 0, 1, 0]
// */

pub fn bin_u8(num: &u8) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut num = *num;

    for _ in 0..8 {
        res.push(num % 2);
        num /= 2;
    }

    res.reverse();

    res
}

// /*
//   Converts an array of bytes into an array
//   of their binary representation

//   EXAMPLE:
//     [123, 10] => [0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0]
// */

pub fn bin_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for byte in bytes {
        let bin = bin_u8(byte);

        for bit in bin {
            res.push(bit);
        }
    }

    res
}

/*
  Replaces the last bits of an array of u8
  with respective bits of message bits

  EXAMPLE:
    data = [123, 10], message = [0, 1]
    123 => 0111101|1| => 0111101|0|
    10  => 0000101|0| => 0000101|1|
*/

pub fn encode_lsb(file_data: &mut Vec<u8>, message_bits: &Vec<u8>) {
    for i in 0..message_bits.len() {
        file_data[i] >>= 1;
        file_data[i] <<= 1;

        file_data[i] |= message_bits[i];
    }
}
