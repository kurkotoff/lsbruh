use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use structopt::StructOpt;

use png::Decoder;
use png::Encoder;

#[derive(StructOpt, Debug)]
pub struct Config {
    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// Message to hide
    #[structopt(short, long)]
    message: String,

    /// Input file to process
    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,
}

/*
  Config is a structure that saves
  the command line arguments in their
  respective values
*/

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    // Reading message from config and converting it into bits
    let bin_bytes = bin_bytes(config.message.as_bytes());

    // Creating the decoder and reader objects 
    // to read input image data
    let decoder = Decoder::new(File::open(&config.input).unwrap());
    let mut reader = decoder.read_info().unwrap();

    // Creating the input image data buffer
    let mut data = vec![0; reader.output_buffer_size()];
    
    // Saving the image data as bytes (u8)
    reader.next_frame(&mut data).unwrap();

    // Pulling information about image to an object
    let info = reader.info();

    encode_lsb(&mut data, &bin_bytes);

    // Creating an encoder and writer objects
    // For generating an output pic
    let output_file = File::create(PathBuf::from(&config.output)).unwrap();
    let mut encoder = Encoder::new(BufWriter::new(output_file), info.width, info.height);

    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut writer = encoder.write_header().unwrap();

    // Writing the processed data to a new image
    writer.write_image_data(&data).unwrap();

    Ok(())
}

/*
  Converts a u8 number into an array
  of 0 and 1 with length 8

  EXAMPLE:
    123 => [0, 1, 1, 1, 1, 0, 1, 1]
    10  => [0, 0, 0, 0, 1, 0, 1, 0]
*/

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

/*
  Converts an array of bytes into an array
  of their binary representation

  EXAMPLE:
    [123, 10] => [0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0]
*/

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
