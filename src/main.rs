use webbrowser;
use std::env;
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: butler <command>");
        return;
    }

    let command = args[1].as_str();

    match command {
        "search" => open_chrome(),
        "grab" => {
		if args.len() < 4 {
			println!("Usage: butler grab <pattern> <filename>");
		}
		else {
			butler_grab(&args[2], &args[3]);
		}
	},
        _ => println!("Unknown command"),
    }
}

fn open_chrome() {
    // Code to open Chrome goes here
	webbrowser::open("https://www.google.com/");
}


fn butler_grab(pattern: &str, filename: &str){
	use regex::Regex;
	
	//create new regex pattern, throws an error if pattern is invalid.
	let re = match Regex::new(pattern) {
		Ok(re) => re,
		Err(e) => {
			println!("Invalid regex pattern: {}", e);
			return;
		}
	};
	
	//opens a new file, throws an error if file is not found.
	let f = match File::open(filename) {
		Ok(file) => BufReader::new(file),
		Err(e) => {
			println!("Error opening file: {}", e);
			return;	
		}
	};
	
	for line in f.lines() {
		let line = line.expect("Unable to read line");
		if re.is_match(&line) {
			println!("{}", line);
		}
	}
}
	
