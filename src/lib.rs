use serde::Serialize;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

pub struct Config {
    pub infile: String,
    pub outfile: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let infile = args[1].clone();
        let outfile = args[2].clone();

        Ok(Config { infile, outfile })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let infile = File::open(config.infile)?;
    let reader = BufReader::new(infile);
    let mut outfile = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config.outfile)
        .unwrap();

    for line in reader.lines() {
        let v: Value = serde_json::from_str(&line.unwrap())?;
        let output = Output::from(v);
        let j = serde_json::to_string(&output)?;

        if let Err(e) = writeln!(outfile, "{}", &j) {
            eprintln!("Couldn't write to file: {}", e);
        }

        println!("{}", j);
    }

    Ok(())
}

#[derive(Serialize, Debug)]
pub struct Output {
    date_text: Option<String>,
    username: Option<String>,
    id: Option<String>,
}

impl From<Value> for Output {
    fn from(item: Value) -> Output {
        Output {
            date_text: item["created_at"].as_str().map(String::from),
            username: item["user"]["name"].as_str().map(String::from),
            id: item["id_str"].as_str().map(String::from),
        }
    }
}
