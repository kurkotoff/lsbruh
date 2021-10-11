use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::env;

use png::Decoder;
use png::Encoder;

pub struct Config {
    mode: String,
    message: String,
    input_file: String,
    output_file: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        args.next();

        let mode = match args.next() {
            Some(arg) => arg,
            None => return Err("Operation mode not found")
        };

        let message = if mode == "w" {
            match args.next() {
                Some (arg) => arg,
                None => return Err("Message not found")
            };
            String::new()
        } else {
            String::new()
        };

        let input_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Input filename not found")
        };
            
        let output_file = if mode == "w" {
            match args.next() {
                Some (arg) => arg,
                None => return Err("Message not found")
            };
            String::new()
        } else {
            String::new()
        };
        
        Ok(Config {
            mode,
            message,
            input_file,
            output_file,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let message = String::from("hello");
    let message_bytes = message.as_bytes();

    let decoder = Decoder::new(File::open("hello.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
        
    let mut data = vec![0; reader.output_buffer_size()];

    if data.len() < message_bytes.len(){

    }

    reader.next_frame(&mut data).unwrap();
    let info = reader.info();


    for val in &mut data {
        if *val % 2 != 1 {
            *val += 1;
        }
    }

    let output_file = File::create(Path::new("square_hidden.png")).unwrap();

    let ref mut w = BufWriter::new(output_file);
    let mut encoder = Encoder::new(w, info.width, info.height);

    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut writer = encoder.write_header().unwrap();
    
    writer.write_image_data(&data).unwrap();

    Ok(())
}
