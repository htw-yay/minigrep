use minigrep::Config;
use std::{env, process};
fn main() {
    let args = env::args().collect::<Vec<_>>();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem during parsing arguments: {}", err);
        process::exit(1)
    });

    let contents = config.read().unwrap_or_else(|err| {
        eprintln!("Problem during reading the file: {err}");
        process::exit(1)
    });

    let results = config.search(&contents);
    println!("{results:#?}")
}
