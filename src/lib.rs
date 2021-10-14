use std::{path::PathBuf};
use structopt::StructOpt;

use std::fs::File;
use std::io::{Read, Write};
use std::io::BufWriter;

use png::{Encoder, Decoder};


#[derive(StructOpt, Debug)]
pub struct Config {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Write(WriteOptions),
    Read(ReadOptions),
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

#[derive(Debug, StructOpt)]
pub struct ReadOptions {
    /// Input file to process
    #[structopt(name = "FILE", parse(from_os_str))]
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

fn bin_u8(num: &u8) -> Vec<u8> {
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

fn bin_bytes(bytes: &[u8]) -> Vec<u8> {
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

fn encode_lsb(file_data: &mut Vec<u8>, message_bits: &Vec<u8>) {
    for i in 0..message_bits.len() {
        file_data[i] >>= 1;
        file_data[i] <<= 1;

        file_data[i] |= message_bits[i];
    }
}

fn read_lsb(file: &mut Vec<u8>) -> Vec<u8> {

    let mut message = Vec::new();
    for byte in file {
        let last_bit = *byte  % 2;
        message.push(last_bit);
    }

    let read_size = message.len() / 8 * 8;
    let mut buf:Vec<u8> = vec![0;8];
    let mut res = Vec::new();

    for i in 0..read_size {
        if i % 8 == 0 && i != 0 {
            let mut val: u8 = 0;
        
            for j in 0..8 {
                let index = j as u32;
                let power = 7 - index % 8;
                val += buf[j] *2u8.pow(power);
            }
            
            res.push(val);
            buf.resize(8, 0);
        }

        buf[i % 8] = message[i];
    }

    res
}


pub fn write_data(options: &WriteOptions){
    let mut input_file = File::open(&options.input_file)    // Reading message from config and converting it into bits
        .expect(format!("No file {:?} found", options.input_file).as_str());

    let mut message: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut message).unwrap();
    let bin_bytes = bin_bytes(&message);

    let decoder = Decoder::new(File::open(&options.secret).unwrap());  // Creating the decoder and reader objects 
    let mut reader = decoder.read_info().unwrap();                           // to read input image data
    let mut data = vec![0; reader.output_buffer_size()];                         // Creating the input image data buffer

    
    reader.next_frame(&mut data).unwrap();                                          // Saving the image data as bytes (u8)
    let info = reader.info();                                                     // Pulling information about image to an object

    encode_lsb(&mut data, &bin_bytes);
 
    let output_file = File::create(PathBuf::from(&options.output)).unwrap();  // Creating an encoder and writer objects for generating an output pic

    let mut encoder = Encoder::new(
        BufWriter::new(output_file), 
        info.width, 
        info.height);
    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();                                            // Writing the processed data to a new image
}

pub fn read_data(options: &ReadOptions) {
    let decoder = Decoder::new(File::open(&options.input_file).unwrap());  // Creating the decoder and reader objects 
    let mut reader = decoder.read_info().unwrap();                               // to read input image data
    let mut data = vec![0; reader.output_buffer_size()];                             // Creating the input image data buffer
    reader.next_frame(&mut data).unwrap();

    let mut file_lsb = read_lsb(&mut data);

    let mut output_file = File::create(&options.output).unwrap();

    output_file.write_all(&mut file_lsb).unwrap();
}