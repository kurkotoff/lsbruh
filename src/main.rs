use lsbruh::Config;
use structopt::StructOpt;


fn main() {
    let config = Config::from_args();
    
    if let Err(e) = lsbruh::run(config) {
        eprintln!("Application error: {}", e);

        std::process::exit(1);
    }
}