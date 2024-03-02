use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use clap::Parser;
use regex::Regex;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    search: String,

    #[arg(short, long)]
    replace: String
}

pub fn main() {
    println!("File processor");
    println!("{:?}", std::env::current_dir());

    let args = Args::parse();

    let file_path = args.file;
    let search = args.search;
    let replace = args.replace;

    let path = Path::new(&file_path);
    let display = path.display();

    // Open file
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(file) => file
    };

    // Read contents
    let mut read_string = String::new();
    match file.read_to_string(&mut read_string) {
        Err(why) => panic!("couldn't get contents {}: {}", display, why),
        Ok(_) => println!("contains {}: {}", display, &read_string)
    }

    //////////////////
    // This section tests how cycles work
    println!("Reading letter-by-letter");
    let mut read_string_chars: Vec<char> = read_string.chars().collect();
    for i in 0..read_string_chars.len() {
        let letter: char = read_string_chars[i];
        print!("{}", letter)
    }
    println!();

    for letter in read_string.chars() {
        print!("{}", letter)
    }

    for (i, letter) in read_string.chars().enumerate() {
        print!("{}={}, ", i, letter);
    }

    // end test section
    ///////////////////

    let search_pattern = Regex::new(&search).unwrap();
    let replaced_string = search_pattern.replace_all(&read_string, replace);

    println!("replaced string");
    println!("{}", replaced_string);

    let parts: Vec<&str> = file_path.split(".").collect();
    let name_without_extension: String = if parts.len() > 1 {
        parts[..parts.len() - 1].join(".")
    } else {
        file_path.to_string()
    };
    let new_file_path = name_without_extension + "_replaced.txt";

    let mut file2 = File::create(new_file_path).unwrap();
    file2.write_all(replaced_string.as_bytes()).unwrap()
}