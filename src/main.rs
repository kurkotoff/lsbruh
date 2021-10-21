pub(crate) use structopt::StructOpt;

use lsbruh::Command;
use lsbruh::Config;


fn main() {
    let config = Config::from_args();
    
    match config.cmd {
        Command::Write(options) => {
            if let Err(e) = lsbruh::write_data(&options) {
                eprintln!("Application error: {}", e);
            }
        }

        Command::Read(options) => {
            if let Err(e) = lsbruh::read_data(&options) {
                eprintln!("Application error: {}", e);
            }        
        }
    }
}
