use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use clap::ArgAction::SetTrue;
use clap::Parser;
use colored::Colorize;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[arg(short, long)]
    lines: Option<i32>,

    #[arg(short, long)]
    file_name : String,

    #[arg(short, long)]
    search_word : String,

    #[arg(short, long, action = SetTrue)]
    number : bool,
}

fn main() -> Result<(), Box<dyn Error>>{
    let args : Args = Args::parse();
    let around_lines : i32 = args.lines.unwrap_or(0);
    let file_name : String = args.file_name;
    let search_word : String = args.search_word;
    let with_line_number = args.number;
    let bar = "=".repeat(10);

    let path: &Path = Path::new(&file_name);
    if !path.exists() {
        println!("The target file {} is not here.", path.to_str().expect("_"));
        return Ok(());
    }

    let mut candidate : HashSet<i32> = HashSet::new();
    let pattern = Regex::new(search_word.as_str())?;

    let mut index : i32 = 0;
    for result in BufReader::new(File::open(path)?).lines() {
        let l = result?;
        if pattern.is_match(l.as_str()) {
            for i in -around_lines..=around_lines { candidate.insert(index + i); }
        }
        index += 1;
    }
    let mut result : Vec<i32> = Vec::new();
    for e in candidate { result.push(e) }
    result.sort();
    let mut before = 0;

    let mut index = 0;
    for e in BufReader::new(File::open(path)?).lines() {
        if !result.contains(&index) {
            index += 1;
            continue
        };

        let need_bar = index == 0 || 1 < index - before;
        let l = e?;
        before = index;
        index += 1;

        if need_bar { println!("{}", &bar) };

        if pattern.is_match(&l) {
            let matched : String = pattern.captures(l.as_str()).unwrap().get(0).unwrap().as_str().to_string();
            print!("{}", prefix(&index, &with_line_number));
            color_print(&matched, &l);
        } else { println!("{}", prefix(&index, &with_line_number) + &l) };
    }
    Ok(())
}

fn prefix(index : &i32, with_number : &bool) -> String {
    if !with_number { "".to_string() } else { index.to_string() + " " }
}

fn color_print(word : &String, input : &String) {
    let mut input_copied : String = String::from(input);
    loop {
        let index = &input_copied.find(word);
        match index {
            None => {
                println!("{}", input_copied);
                break;
            },
            Some(e) => {
                if *e == 0usize {
                    print!("{}", word.red());
                } else {
                    let normal = &input_copied[0usize..*e];
                    print!("{}", normal);
                    input_copied = input_copied.replacen(normal, "", 1);
                    print!("{}", word.red());
                }
                input_copied = input_copied.replacen(word, "", 1);
            }
        };
    }
}
