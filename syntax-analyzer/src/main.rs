use clap::{arg, Command};
use std::fs;
use syntax_analyzer::tokenizer::JackTokenizer;

fn main() {
    let matches = Command::new("tokenize")
        .version("0.1.0")
        .about("Tokenize.")
        .arg(arg!(<FILE_NAME> "The filename").required(true).index(1))
        .long_flag("--filename")
        .get_matches();

    let file_name = matches.get_one::<String>("FILE_NAME").unwrap();
    let content = fs::read_to_string(file_name);

    let mut tokenizer = JackTokenizer::new(content.unwrap_or(String::new()));

    for i in 0..20 {
        tokenizer.advance();
        println!("{:?}", tokenizer.current_token);
    }
}
