use std::io::{stdin, stdout, Write};

pub fn starting_output(year: u8, land: i32, pop: i32, seeds: i32) {
    println!("====[Welcome to Hammurabi]====");
    println!("Current Year: {}", year);
    println!("Current Land: {}", land);
    println!("Current Population: {}", pop);
    println!("Current Seeds: {}", seeds);
    println!("");
    println!("What would you like to do?");
    println!("");
    println!("[1]: Plant Seeds (will also progress a year)");
    println!("[2]: Buy Land");
    println!("[3]: Sell Land");
    println!("[4]: Progress a year doing nothing");
    println!("[q]: Quit Game");
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

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}
