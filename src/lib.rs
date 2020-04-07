use serde::Serialize;
use serde::Deserialize;
//use serde_json::Value;
//use serde_json::Result;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

pub struct Config {
    pub infile: String,
    pub outfile: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    screen_name: Option<String>,
    location: Option<String>,
    description: Option<String>,
    id_str: Option<String>,
    verified: Option<bool>,
    followers_count: Option<i32>,
    friends_count: Option<i32>,
    listed_count: Option<i32>,
    favourites_count: Option<i32>,
    statuses_count: Option<i32>,
    created_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Input {
    text: Option<String>,
    user: User
}

#[derive(Serialize, Debug)]
pub struct Output {
    text: Option<String>,
    username: Option<String>,
    location: Option<String>,
    verified: Option<bool>,
    followers_count: Option<i32>
}

impl From<Input> for Output {
    fn from(item: Input) -> Output {
        Output {
            text: item.text,
            username: item.user.screen_name,
            location: item.user.location,
            verified: item.user.verified,
            followers_count: item.user.followers_count
        }
    }
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
        let v: Input = serde_json::from_str(&line.unwrap())?;
        let output = Output::from(v);
        let j = serde_json::to_string(&output)?;

        if let Err(e) = writeln!(outfile, "{}", &j) {
            eprintln!("Couldn't write to file: {}", e);
        }

        //println!("{}", j);
    }

    Ok(())
}
