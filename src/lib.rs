use structopt::StructOpt;
use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;


use png::Decoder;
use png::Encoder;

#[derive(StructOpt, Debug)]
pub struct Config {

    /// Output file
    #[structopt(
        short, 
        long, 
        parse(from_os_str))]
    output: PathBuf,

    /// Message to hide
    #[structopt(
        short, 
        long )]
    message: String,

    /// Input file to process
    #[structopt(
        name = "FILE", 
        parse(from_os_str))]
    input: PathBuf,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let bin_bytes = bin_bytes(config.message.as_bytes());


    let decoder = Decoder::new(File::open(&config.input).unwrap());
    
    let mut reader = decoder.read_info().unwrap();
    let mut data = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut data).unwrap();
    let info = reader.info();


    encode_lsb(&mut data, &bin_bytes);



    let output_file = File::create(PathBuf::from(&config.output)).unwrap();

    let mut encoder = Encoder::new(
        BufWriter::new(output_file),
        info.width,
        info.height
    );

    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap();

    Ok(())
}

fn bin_u8(num: &u8) -> Vec<u8>{
    let mut res: Vec<u8> = Vec::new();
    let mut num = *num;

    for _ in 0..8 {
        res.push(num % 2);
        num /= 2;
    }

    res.reverse();

    res
}

fn bin_bytes(bytes: &[u8]) -> Vec<u8>{
    let mut res: Vec<u8> = Vec::new();

    for byte in bytes {
        let bin = bin_u8(byte);
        
        for bit in bin {
            res.push(bit);
        }
    }

    res
}

fn encode_lsb(file_data: &mut Vec<u8>, message_bits: &Vec<u8>) {    
    for i in 0..message_bits.len(){
      file_data[i] >>= 1;
      file_data[i] <<= 1;

      file_data[i] |= message_bits[i];
    };
} 