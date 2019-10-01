use std::io::{stdin, stdout, Write};

const POP_GROWTH_RATE: f32 = 1.25;
const SEED_GROWTH_RATE: f32 = 3.0;
const POP_BELOW_END_GAME: i32 = 10;
const LAND_SEED_COST: i32 = 10;
const SEED_LAND_COST: i32 = 5;

fn starting_output(year: u8, land: i32, pop: i32, seeds: i32) {
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

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn accept_input() -> String {
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

fn process_seed_planting(mut pop: i32, mut seeds: i32, amount_to_plant: i32) -> (i32, i32) {
    seeds = (seeds - amount_to_plant) + (amount_to_plant as f32 * SEED_GROWTH_RATE) as i32;
    if seeds < pop {
        pop = seeds;
        seeds = 0;
    } else {
        seeds = seeds - pop;
        pop = (pop as f32 * POP_GROWTH_RATE) as i32;
    }
    if pop < POP_BELOW_END_GAME {
        pop = 0;
    }
    (pop, seeds)
}

fn main() {
    let mut land: i32 = 150;
    let mut pop: i32 = 100;
    let mut seeds: i32 = 250;
    let mut year: u8 = 1;
    let mut quit: bool = false;
    let mut dead: bool = false;
    let mut showing_error_message: bool = false;

    clear_screen();
    while !quit {
        if showing_error_message {
            println!("Press enter to continue...");
            let input: String = accept_input();
            if input == "" {
                showing_error_message = false;
                clear_screen();
            }
        } else {
            clear_screen();
            starting_output(year, land, pop, seeds);
            let input: String = accept_input();
            clear_screen();
            if input == "1" {
                let mut planted: bool = false;
                while !planted {
                    println!("====[Planting Seeds]====");
                    println!("Current seeds: {}", seeds);
                    println!("Available land: {}", land);
                    println!("How many seeds would you like to plant?");
                    let amount_to_plant: i32 = accept_input().parse().unwrap(); // TODO - panics here if not int
                    if amount_to_plant > seeds || amount_to_plant > land {
                        clear_screen();
                        println!("You cannot plant that many seeds!");
                        showing_error_message = true;
                    } else {
                        let (new_pop, new_seeds) = process_seed_planting(pop, seeds, amount_to_plant);
                        pop = new_pop;
                        seeds = new_seeds;
                        if pop <= 0 {
                            quit = true;
                            dead = true;
                        }
                        year += 1;
                        planted = true;
                    }
                }
            }
            else if input == "2" {
                let mut bought: bool = false;
                while !bought {
                    println!("====[Buying Land]====");
                    println!("Current seeds: {}", seeds);
                    println!("Current land: {}", land);
                    println!("Exchange {} seeds per 1 land", LAND_SEED_COST);
                    println!("How much land would you like to buy?");
                    let amount_to_buy: i32 = accept_input().parse().unwrap();
                    if amount_to_buy * LAND_SEED_COST > seeds {
                        clear_screen();
                        println!("You cannot buy that much land!");
                        showing_error_message = true;
                    } else {
                        land = land + amount_to_buy;
                        seeds = seeds - (amount_to_buy * LAND_SEED_COST);
                        bought = true;
                    }
                }
            }
            else if input == "3" {
                let mut sold: bool = false;
                while !sold {
                    println!("====[Selling Land]====");
                    println!("Current seeds: {}", seeds);
                    println!("Current land: {}", land);
                    println!("Exchange {} seeds per 1 land", SEED_LAND_COST);
                    println!("How much land would you like to sell?");
                    let amount_to_sell: i32 = accept_input().parse().unwrap();
                    if amount_to_sell > land {
                        clear_screen();
                        println!("You cannot sell that much land!");
                        showing_error_message = true;
                    } else {
                        land = land - amount_to_sell;
                        seeds = seeds + (amount_to_sell * SEED_LAND_COST);
                        sold = true;
                    }
                }
            }
            else if input == "4" {
                let (new_pop, new_seeds) = process_seed_planting(pop, seeds, 0);
                pop = new_pop;
                seeds = new_seeds;
                if pop <= 0 {
                    quit = true;
                    dead = true;
                }
                year += 1; 
            }
            else if input == "q" {
                quit = true;
            }
            else {
                clear_screen();
                println!("Unrecognised input");
                showing_error_message = true;
            }
        }
    }
    if dead {
        println!("Your kingdom fell...");
    } else {
        println!("Bye");
    }
}

