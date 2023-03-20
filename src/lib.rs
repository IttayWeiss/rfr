use clap::{value_parser, Arg, ArgAction, Command};
use std::error::Error;
use std::path::PathBuf;
use std::{fs, io};

type MyResult<T> = Result<T, Box<dyn Error>>;

const ZBMATH_URL: &str = "https://zbmath.org";

// Maximal number of articles to display for choosing interactively.
const MAX_TO_DISPLAY: usize = 20;

// A struct to hold the information about an article.
struct Record {
    // Presentation data:
    title: String,

    // bib retreiving data:
    bib_url: String,
}

pub struct Config {
    search_phrase: String,
    read_from_file: bool,
    input_file: Option<PathBuf>,
    exact_title: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("rfr")
        .version("0.1.0")
        .author("Ittay Weiss <weittay@gmail.com")
        .about("Bibtex reference snatcher")
        .arg(
            Arg::new("search_phrase")
                .value_name("TEXT")
                .help("input search phrase")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("exact_title")
                .help("Match title exactly")
                .short('e')
                .long("exact")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("from_file")
                .value_name("path")
                .help("Load a local html file of zbMATH query results")
                .short('l')
                .long("locally")
                .action(ArgAction::Set)
                .num_args(1)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    Ok(Config {
        search_phrase: matches
            .get_many("search_phrase")
            .expect("Needs to be a string")
            .cloned()
            .collect::<Vec<String>>()
            .join(" ")
            .to_lowercase(),
        read_from_file: matches.contains_id("from_file"),
        input_file: matches.get_one::<PathBuf>("from_file").map(|x| x.clone()),
        exact_title: matches.get_flag("exact_title"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let response = if config.read_from_file {
        response_from_file(
            &config
                .input_file
                .expect("input file must be valid since set to read from file"),
        )?
    } else {
        response_from_zbmath(&config.search_phrase)?
    };

    let articles = scrape_zbmath(&response);
    if articles.is_empty() {
        println!("No articles found.");
        return Ok(());
    }

    let the_index = if config.exact_title {
        if let Some(i) = articles
            .iter()
            .position(|x| x.title.to_lowercase() == config.search_phrase)
        {
            i
        } else {
            println!("no exact match found!");
            return Ok(());
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
    // Performs a GET request from zbMATH.
    // ti%3A indicates for zbMATH to perform a title search.
    let url = format!("{}{}{}", ZBMATH_URL, "/?q=ti%3A", query);
    let response = reqwest::blocking::get(url)?.text()?;
    Ok(response)
}

fn response_from_file(file_name: &PathBuf) -> MyResult<String> {
    // Reads response from a locally saved file.
    Ok(fs::read_to_string(file_name)?)
}

fn scrape_zbmath(response: &str) -> Vec<Record> {
    // Scrapes the zbMATH response into a vector of article records.
    let document = scraper::Html::parse_document(response);
    let articles_selector = scraper::Selector::parse(".content-result .list").unwrap();
    document
        .select(&articles_selector)
        .map(extract_from_zbmath)
        .collect()
}

fn extract_from_zbmath(article: scraper::ElementRef) -> Record {
    // Converts a single zbMATH scraper element of an article into a record.
    let title_selector = scraper::Selector::parse(".title strong").unwrap();
    let bib_selector = scraper::Selector::parse(".bib").unwrap();

    let mut title = article.select(&title_selector).next().unwrap().inner_html();
    title.pop();
    let bib_url = article
        .select(&bib_selector)
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap()
        .to_string();

    Record {
        bib_url: format!("{}/{}", ZBMATH_URL, &bib_url),
        title,
    }
}

fn display(articles: &Vec<Record>) -> usize {
    // Displays the articles and returns total number displayed.
    if articles.len() > MAX_TO_DISPLAY {
        println!("Displaying only the first {} articles.", MAX_TO_DISPLAY);
    }
    for (i, a) in articles.iter().take(MAX_TO_DISPLAY).enumerate() {
        println!("{})\t{}", i + 1, a.title);
    }
    articles.len().min(MAX_TO_DISPLAY)
}

fn choose_interactively(articles: &Vec<Record>) -> MyResult<usize> {
    // Display the articles and allows user to choose one.
    let num_articles = display(articles);
    loop {
        println!("Make a choice (1-{}): ", num_articles);
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        match choice.trim().parse::<usize>() {
            Ok(n) if 1 <= n && n <= num_articles => {
                break Ok(n - 1);
            }
            _ => {
                println!("Invalid choice. Try again.");
            }
        }
    }
}

fn extract_bibtex(article: &Record) -> String {
    // Performs a GET request from zbMATH to obtain bibtex data.
    let response = reqwest::blocking::get(&article.bib_url)
        .unwrap()
        .text()
        .unwrap();

    response
}
