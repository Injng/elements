pub mod interpreter;
pub mod lang;
pub mod lexer;
pub mod renderer;
pub mod utils;

use interpreter::evaluate;
use lang::types::Value;
use lexer::{tokenize, Token};
use renderer::render;

use std::fs;

const TOLERANCE: f64 = 1e-10;

fn main() {
    // get args and check for at least 2
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    // check if label and debug is enabled
    let mut is_label = false;
    let mut is_debug = false;
    if args.len() > 2 && args[2] == "--label" {
        is_label = true;
    } else if args.len() > 2 && args[2] == "--debug" {
        is_debug = true;
    }
    if args.len() > 3 && args[3] == "--label" {
        is_label = true;
    } else if args.len() > 3 && args[3] == "--debug" {
        is_debug = true;
    }

    // see if file exists
    let filename = &args[1];
    if !std::path::Path::new(filename).exists() {
        eprintln!("File not found: {}", filename);
        std::process::exit(1);
    }

    // open file and read into string
    let contents = std::fs::read_to_string(filename).expect("Failed to read file");

    // tokenize string
    let tokens: Vec<Token> = tokenize(contents, is_debug);

    // evaluate tokens
    let values: Vec<Value> = match evaluate(tokens) {
        Ok(values) => values,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // if debug is enabled, print the values
    if is_debug {
        println!("{:?}", values);
    }

    // render values to svg
    let svg = render(values, is_label, is_debug).expect("Failed to render");

    // if debug is enabled, print the svg elements
    if is_debug {
        println!("{}", svg);
    }

    // write svg to file
    let filename = "out.svg";
    fs::write(filename, svg).expect("Failed to write file");
}
