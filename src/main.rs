pub(crate) use structopt::StructOpt;

use lsbruh::Command;
use lsbruh::Config;


fn main() {
    let config = Config::from_args();
    
    match config.cmd {
        Command::Write(options) => {
            lsbruh::write_data(&options);
        }

        Command::Read(options) => {
            lsbruh::read_data(&options);
        }
    }
}
