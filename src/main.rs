use gvko_minigrep::Input;
use std::{env, process};

fn main() {
    let input = Input::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing input args: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}'", input.query);
    println!("In file {}\n---\n", input.filename);

    if let Err(err) = gvko_minigrep::run(input) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }

    println!("---");
}
