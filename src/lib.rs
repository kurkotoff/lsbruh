use std::error::Error;
use std::result::Result;
use std::path::PathBuf;
use image::GenericImageView;
use structopt::StructOpt;

use std::fs::File;
use std::io::{Read, Write};

use image::io::Reader as ImageReader;

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

// pub struct LSB {
//     config: Config
// }

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

fn encode_lsb(file_data: &mut Vec<u8>, bin_message: &Vec<u8>) {
    // dbg!(bin_message);

    for i in 0..bin_message.len() {
        file_data[i] >>= 1;
        file_data[i] <<= 1;

        file_data[i] |= bin_message[i];
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

pub fn write_data(options: &WriteOptions) -> Result<(), Box<dyn Error> >{
    let img = ImageReader::open(&options.secret)?.decode()?;
    let mut data = img.to_bytes();

    let mut message: Vec<u8> = Vec::new();
    File::open(&options.input_file)?.read_to_end(&mut message).unwrap();    
    let bin_bytes = bin_bytes(&message);

    check_size(&message, &data)?;
    encode_lsb(&mut data, &bin_bytes);

    image::save_buffer(
        &options.output,
        &data,
        img.width(),
        img.height(),
        img.color()
    )?;

    Ok(())
}


pub fn read_data(options: &ReadOptions) -> Result<(), Box<dyn Error> > {
    let img = ImageReader::open(&options.input_file)?.decode()?;
    let mut data = img.to_bytes();

    let mut file_lsb = read_lsb(&mut data);
    let mut output_file = File::create(&options.output)?;    
    output_file.write_all(&mut file_lsb)?;

    Ok(())
}

fn check_size(message: &Vec<u8>, file: &Vec<u8>) -> std::result::Result<(), &'static str> {
    if message.len() * 8 > file.len() {
        return Err("The input file is too big for the container");
    }

    Ok(())
}