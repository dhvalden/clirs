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
pub struct Entities {
    hashtags: Option<Vec<Hashtag>>,
    urls: Option<Vec<Url>>,
    user_mentions: Option<Vec<Mention>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hashtag {
    indices: Option<Vec<i32>>,
    text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    indices: Option<Vec<i32>>,
    url: Option<String>,
    display_url: Option<String>,
    expanded_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mention {
    name: Option<String>,
    indices: Option<Vec<i32>>,
    screen_name: Option<String>,
    id_str: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ExtendedTweet {
    full_text: Option<String>,
}

impl ExtendedTweet {
    fn full_text(self) -> Option<String> {
        match self.full_text {
            None => None,
            Some(x) => Some(x.clone()),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Status {
    text: Option<String>,
    extended_tweet: Option<ExtendedTweet>,
}

//impl Status {
//    fn full_text(self) -> Option<String> {
//        self.extended_tweet.map(ExtendedTweet::full_text).unwrap_or(self.text.clone())
//    }
//}

#[derive(Deserialize, Debug)]
pub struct Tweet {
    id_str: Option<String>,
    created_at: Option<String>,
    user: User,
    entities: Entities,
    quote_count: Option<i32>,
    reply_count: Option<i32>,
    retweet_count: Option<i32>,
    favorite_count: Option<i32>,

    extended_tweet: Option<ExtendedTweet>,
    
    //retweeted_status: Option<Status>,
    //quoted_status: Option<Status>,
    text: Option<String>,
    source: Option<String>,
    lang: Option<String>,
}

//impl Tweet {
//    fn full_text(&self) -> Option<String> {
//        self.retweeted_status.map(Status::full_text).or_else(|| {
//            self.quoted_status.map(Status::full_text)
//        }).or_else(|| {
//            self.extended_tweet.map(ExtendedTweet::full_text)
//        }).unwrap_or(Some(self.text.clone()))
//    }
//}

#[derive(Serialize, Debug)]
pub struct Digest {
    id: Option<String>,
    created_at: Option<String>,
    screen_name: Option<String>,
    location: Option<String>,
    description: Option<String>,
    user_id: Option<String>,
    verified: Option<bool>,
    followers_count: Option<i32>,
    friends_count: Option<i32>,
    listed_count: Option<i32>,
    favourites_count: Option<i32>,
    statuses_count: Option<i32>,
    user_created_at: Option<String>,
    quote_count: Option<i32>,
    reply_count: Option<i32>,
    retweet_count: Option<i32>,
    favorite_count: Option<i32>,
    hashtags: Option<Vec<Hashtag>>,
    urls: Option<Vec<Url>>,
    user_mentions: Option<Vec<Mention>>,
    //full_text: Option<String>,
    text: Option<String>,
    extended_text: Option<String>,
    source: Option<String>,
    lang: Option<String>,
}

impl From<Tweet> for Digest {
    fn from(item: Tweet) -> Digest {
        Digest {
            //full_text: item.full_text(),

            id: item.id_str,
            created_at: item.created_at,
            screen_name: item.user.screen_name,
            location: item.user.location,
            description: item.user.description,
            user_id: item.user.id_str,
            verified: item.user.verified,
            followers_count: item.user.followers_count,
            friends_count: item.user.friends_count,
            listed_count: item.user.listed_count,
            favourites_count: item.user.favourites_count,
            statuses_count: item.user.statuses_count,
            user_created_at: item.user.created_at,
            quote_count: item.quote_count,
            reply_count: item.reply_count,
            retweet_count: item.retweet_count,
            favorite_count: item.favorite_count,
            hashtags: item.entities.hashtags,
            urls: item.entities.urls,
            user_mentions: item.entities.user_mentions,
            text: item.text,
            extended_text: item.extended_tweet.map(ExtendedTweet::full_text).unwrap_or(None),
            source: item.source,
            lang: item.lang,
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
        let v: Tweet = serde_json::from_str(&line.unwrap())?;
        let output = Digest::from(v);
        let j = serde_json::to_string(&output)?;

        if let Err(e) = writeln!(outfile, "{}", &j) {
            eprintln!("Couldn't write to file: {}", e);
        }

        //println!("{}", j);
    }

    Ok(())
}
