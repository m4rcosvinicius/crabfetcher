mod ascii;
mod get_infos;

use columns::Columns;
use std::env;

const CORRO_COLOR: &str = "\x1b[38;5;1m";
const FERRIS_COLOR: &str = "\x1b[38;5;209m";

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Option::Some(arg) => match arg.as_str() {
            "--ferris" | "-f" => exec(2, CORRO_COLOR),
            "--corro" | "-c" => exec(1, FERRIS_COLOR),
            "--help" => {
                help();
            }
            _ => {
                help();
            }
        },
        Option::None => exec(2, CORRO_COLOR)
    };
}

fn exec(ascii_art: i32, color: &str) {
    let ascii = ascii::choose_ascii(ascii_art);
    let all_infos = get_infos::all_infos(color);

    println!("{}", Columns::from(vec![
                  ascii.lines().collect(),
                  all_infos.lines().collect(),
    ]).base_tabsize_in(0));
}

fn help() {
    println!("crabfetcher --ferris => Use ferris ascii art.");
    println!("crabfetcher --corro  => Use corro ascii art.");
    println!("crabfetcher --help   => Show help.");
    println!("You can use crabfetcher -c or crabfetcher -f too!");
}
