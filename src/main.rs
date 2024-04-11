use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use clap::ArgAction::SetTrue;
use clap::ValueHint::FilePath;
use clap::Parser;
use colored::Colorize;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[arg(short, long)]
    lines: Option<i32>,

    #[arg(short, long, value_hint = FilePath)]
    file_name : PathBuf,

    #[arg(short, long)]
    search_word : String,

    #[arg(short, long, action = SetTrue)]
    number : bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args : Args = Args::parse();
    let around_lines : i32 = args.lines.unwrap_or(0);
    let file_name : PathBuf = args.file_name;
    let search_word : String = args.search_word;
    let with_line_number = args.number;
    let bar = "=".repeat(10);

    if !file_name.exists() {
        return Err(Box::from(file_name.try_exists().expect_err("The specified file does not exist.")));
    }

    let pattern = Regex::new(search_word.as_str())?;
    let mut display : HashSet<i32> = HashSet::new();
    let mut matched : HashSet<i32> = HashSet::new();
    let mut index : i32 = 0;
    let mut before_matched : i32 = -1;

    for line in  BufReader::new(File::open(&file_name)?).lines() {
        let line = line?;
        let matches : bool = pattern.is_match(line.as_str());
        let diff : i32 = if matches || index == 0 { -1 } else { index - before_matched };
        let displays : bool = matches || (-1 < diff && diff < around_lines);
        let current_index = index;
        index += 1;

        if !displays { continue };
        if matches {
            before_matched = current_index;
            matched.insert(current_index);
            for i in -around_lines..=around_lines {
                if i == 0 { continue };
                display.insert(current_index + i);
            }
        } else {
            display.insert(current_index);
        }
    }

    let mut index : i32 = 0;
    for line in BufReader::new(File::open(&file_name)?).lines() {
        let line = line?;
        if (display.contains(&index) || matched.contains(&index)) && index != 0 && !display.contains(&(index - 1)) && !matched.contains(&(index - 1)) { println!("{}", &bar) };
        if matched.contains(&index) {
            print!("{}", prefix(&index, &with_line_number));
            color_print(&search_word, &line);
        } else if display.contains(&index) { println!("{}", prefix(&index, &with_line_number) + &line) };
        index += 1;
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
