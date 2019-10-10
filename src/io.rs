use std::io::{stdin, stdout, Write};
use crate::ascii;

use colored::*;

pub fn starting_output(year: u8, land: i32, pop: i32, seeds: i32) {
    println!("{}", "====[Welcome to Hammurabi]====".green());
    println!("Current Year: {}", year.to_string().bright_blue());
    println!("Current Land: {}", land.to_string().bright_blue());
    println!("Current Population: {}", pop.to_string().bright_blue());
    println!("Current Seeds: {}", seeds.to_string().bright_blue());
    if pop <= 15 {
        ascii::no_city();
    } else if pop > 15 && pop <= 125 {
        ascii::tiny_city();
    } else if pop > 125 && pop <= 175 {
        ascii::small_city();
    } else if pop > 175 && pop <= 275 {
        ascii::med_city();
    } else if pop > 275 && pop <= 500 {
        ascii::large_city();
    } else if pop > 500 && pop <= 1000 {
        ascii::huge_city();
    } else {
        ascii::massive_city();
    }
    println!("");
    println!("What would you like to do?");
    println!("");
    println!("{}: Plant Seeds (will also progress a year)", "[1]".yellow());
    println!("{}: Buy Land", "[2]".yellow());
    println!("{}: Sell Land", "[3]".yellow());
    println!("{}: Progress a year doing nothing", "[4]".yellow());
    println!("{}: Quit Game", "[q]".yellow());
}

pub fn accept_input() -> String {
    let mut s=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}

pub fn accept_number() -> i32 {
    let mut i = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut i).expect("Did not enter a correct string");
    if let Some('\n')=i.chars().next_back() {
        i.pop();
    }
    if let Some('\r')=i.chars().next_back() {
        i.pop();
    }
    match i.parse::<i32>() {
        Ok(i) => i,
        Err(_) => -1
    }
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}
