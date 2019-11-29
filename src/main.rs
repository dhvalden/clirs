use clirs::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //println!("Processing {}", config.infile);
    //println!("Output file {}", config.outfile);

    if let Err(e) = clirs::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
