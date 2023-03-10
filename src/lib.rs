use clap::{Arg, ArgAction, Command};
use std::io::Write;
use std::error::Error;
use std::fs;
//use std::fs::File;
//use std::io::prelude::*;

type MyResult<T> = Result<T, Box<dyn Error>>;

const ZBMATH_URL: &str = "https://zbmath.org";
const MAX_TO_DISPLAY: usize = 20;

struct Record {
    bib_url: String,
    title: String,
}

#[derive(Debug)]
pub struct Config {
    search_phrase: String,
    read_from_file: bool, 
    input_file: Option<String>,
    exact_title: bool,
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
        .arg(Arg::new("exact_title")
            .help("Match title exactly")
            .short('e')
            .long("exact")
            .action(ArgAction::SetTrue)
        )
        .arg(Arg::new("from_file")
            .value_name("path")
            .help("load a local html file of zbMATH query results")
            .short('l')
            .long("locally") 
            .action(ArgAction::Set)
            .num_args(1)
            .value_parser(clap::builder::NonEmptyStringValueParser::new()) // improve parser type
        )
        .get_matches();

    Ok(Config {
        search_phrase : matches.get_many("search_phrase")
            .expect("Needs to be a string")
            .cloned()
            .collect::<Vec<String>>()
            .join(" ")
            .to_lowercase(),
        read_from_file : matches.contains_id("from_file"),
        input_file : matches.get_one::<String>("from_file").map(|x| x.to_string()),
        exact_title : matches.get_flag("exact_title"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let response = if config.read_from_file {
        response_from_file(&config.input_file.unwrap())?
    } else {
        response_from_zbmath(&config.search_phrase)?
    };

    //{
    //    let mut file = fs::File::create("empty.html")?;
    //    file.write_all(response.as_bytes())?;
    //}
    
    let articles = scrape_zbmath(&response);
    if articles.len() == 0 {
        println!("No articles found.");
        return Ok(())
    }

    let the_index = if config.exact_title {
        if let Some(i) = articles.iter().position(|x| x.title.to_lowercase() == config.search_phrase) {
            i
        } else {
            println!("no exact match found!");
            return Ok(())
        }
            
    } else {
        choose_interactively(&articles)?
    };

    let the_article = &articles[the_index];
    let bibtex = extract_bibtex(the_article);
    println!("{}", &bibtex);
    
    Ok(())
}

fn response_from_zbmath(query: &str) -> MyResult<String> {
    // ti%3A searches in title
    let url = format!("{}{}{}", ZBMATH_URL, "/?q=ti%3A", query);   
    let response = reqwest::blocking::get(url)?.text()?;
    Ok(response)
}

fn response_from_file(file_name: &str) -> MyResult<String> {
    Ok(fs::read_to_string(file_name)?)
}

fn scrape_zbmath(response: &str) -> Vec<Record>{
    let document = scraper::Html::parse_document(response);
    let articles_selector = scraper::Selector::parse(".content-result .list").unwrap();
    document
        .select(&articles_selector)
        .map(extract_from_zbmath)
        .collect()
}

fn extract_from_zbmath(article: scraper::ElementRef) -> Record {
    let title_selector = scraper::Selector::parse(".title strong").unwrap();
    let bib_selector = scraper::Selector::parse(".bib").unwrap();
    
    let mut title = article.select(&title_selector).next().unwrap().inner_html();
    title.pop();
    let bib_url = article.select(&bib_selector).next().unwrap().value().attr("href").unwrap().to_string();

    let res = Record {
        bib_url: format!("{}/{}", ZBMATH_URL, &bib_url),
        title : title, 
    };
    return res;
}

fn display(articles: &Vec<Record>) -> usize {
    if articles.len() > MAX_TO_DISPLAY {
        println!("Number of articles exceeds {}. Displaying the first {}.", MAX_TO_DISPLAY, MAX_TO_DISPLAY);
    }
    for (i, a) in articles.iter().take(MAX_TO_DISPLAY).enumerate() {
        println!("{})\t{}", i + 1, a.title);
    }
    articles.len().min(MAX_TO_DISPLAY)
}

fn choose_interactively(articles: &Vec<Record>) -> MyResult<usize> {
    return Ok(1)
}

fn extract_bibtex(article: &Record) -> String {
    let response = reqwest::blocking::get(&article.bib_url).unwrap().text().unwrap();

    response
}