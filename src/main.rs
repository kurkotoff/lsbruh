use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::io::BufWriter;

use structopt::StructOpt;

use png::{Decoder, Encoder};

use lsbruh::Command;
use lsbruh::Config;


fn main() {
    let config = Config::from_args();
    
    match config.cmd {
        Command::Write(options) => {
                // Reading message from config and converting it into bits
                let mut input_file = File::open(&options.input_file)
                    .expect(format!("No file {:?} found", options.input_file).as_str());

                let mut message: Vec<u8> = Vec::new();
                
                input_file.read_to_end(&mut message).unwrap();
                
                let bin_bytes = lsbruh::bin_bytes(&message);
                
                // Creating the decoder and reader objects 
                // to read input image data
                let decoder = Decoder::new(File::open(&options.secret).unwrap());
                let mut reader = decoder.read_info().unwrap();

                // Creating the input image data buffer
                let mut data = vec![0; reader.output_buffer_size()];
                
                // Saving the image data as bytes (u8)
                reader.next_frame(&mut data).unwrap();

                // Pulling information about image to an object
                let info = reader.info();

                lsbruh::encode_lsb(&mut data, &bin_bytes);

                // Creating an encoder and writer objects
                // For generating an output pic
                let output_file = File::create(PathBuf::from(&options.output)).unwrap();
                let mut encoder = Encoder::new(BufWriter::new(output_file), info.width, info.height);

                encoder.set_color(info.color_type);
                encoder.set_depth(info.bit_depth);

                let mut writer = encoder.write_header().unwrap();

                // Writing the processed data to a new image
                writer.write_image_data(&data).unwrap();
        }
    }
}
