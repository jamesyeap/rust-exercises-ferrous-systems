use std::{env, io};
use url::Url;

const INCORRECT_FILE_PATH: &'static str = "src/lib/content.txt";
const CORRECT_FILE_PATH: &'static str = "/Users/jamesyeap/Developer/rust/rust-exercises-ferrous-systems/crates/urls-match-result/src/data/content.txt";

fn main() -> Result<(), io::Error> {
    // let file_contents = std::fs::read_to_string(INCORRECT_FILE_PATH)?;
    let file_contents = std::fs::read_to_string(CORRECT_FILE_PATH)?;

    let num_lines = file_contents.lines().count();
    println!("number of lines in fine: {num_lines}");

    for non_empty_line in file_contents.lines().filter(|line| !line.is_empty()) {
        println!("{non_empty_line}");
    }

    for url_optional in file_contents.lines().map(|line| parse_url(line)) {
       match url_optional {
           Some(url) => println!("Is a URL: {url}"),
           None => println!("Not a URL"),
       }
    }

    Ok(())
}

fn parse_url(input: &str) -> Option<Url> {
   match Url::parse(input) {
       Ok(url) => Some(url),
       Err(error) => None,
   }
}

#[test]
fn correct_url() {
    assert!(parse_url("https://example.com").is_some())
}

#[test]
fn no_url() {
    assert!(parse_url("abcdf").is_none())
}