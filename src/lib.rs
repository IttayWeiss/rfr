use clap::{Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    search_phrase: String,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("zbref")
        .version("0.1.0")
        .author("Ittay Weiss <weittay@gmail.com")
        .about("Bibtex reference snatcher for zbMATH")
        .arg(Arg::new("search_phrase")
            .value_name("TEXT")
            .help("input search phrase")
            .required(true)
            .num_args(1..)
        )
        .get_matches();

    Ok(Config {
        search_phrase : matches.get_many("search_phrase")
            .expect("Needs to be a string") 
            .cloned()
            .collect::<Vec<String>>()
            .join(" ")
            .to_lowercase(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{}", config.search_phrase);

    Ok(())
}