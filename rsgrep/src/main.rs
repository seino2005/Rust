use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
// regex
extern crate regex;
use regex::Regex;

fn usage() {
  println!("rsgrep PATTERN FILENAME")
}

fn main() {
  // arg1
  let pattern = match env::args().nth(1) {
    Some(pattern) => pattern,
    None => {
      usage();
      return;
    }
  };
  
  // create regex
  let reg = match Regex::new(&pattern) {
    Ok(reg) => reg,
    Err(e) => {
      println!("invalid regexp {}: {}", pattern, e);
      return;
    }
  };

  // arg2
  let filename = match env::args().nth(2) {
    Some(filename) => filename,
    None => {
      usage();
      return;
    }
  };

  // file open
  let file = match File::open(&filename) {
    Ok(file) => file,
    Err(e) => {
      println!("An error occurred while opening file {}:{}", filename, e);
      return;
    }
  };

  // file input
  let input = BufReader::new(file);
  for line in input.lines() {
    let line = match line {
      Ok(line) => line,
      Err(e) => {
        println!("An error occurred while reading a line {}", e);
        return;
      }
    };
    //println!("[DEBUG] {}", line);
    // is match
    if reg.is_match(&line) {
      println!("{}", line);
    }
  }
 
 
}
