extern crate rand;
use rand::Rng;

use crate::io;
use crate::consts;
use crate::ascii;

pub fn plant_seeds(mut seeds: i32, land: i32, mut pop: i32) -> (bool, i32, i32) {
    let mut success: bool = false;
    let mut input_amount: i32 = -1;
    while input_amount == -1 {
        io::clear_screen();
        ascii::grape();
        println!("");
        println!("====[Planting Seeds]====");
        println!("Current seeds: {}", seeds);
        println!("Available land: {}", land);
        println!("How many seeds would you like to plant?");
        input_amount = io::accept_number();
    }
    let amount_to_plant: i32 = input_amount;
    if amount_to_plant > seeds || amount_to_plant > land {
        io::clear_screen();
        println!("You cannot plant that many seeds!");
        io::accept_input();
    } else {
        let (new_pop, new_seeds) = process_seed_planting(pop, seeds, amount_to_plant);
        pop = new_pop;
        seeds = new_seeds;
        success = true;
    }
    (success, seeds, pop)
}

pub fn process_seed_planting(mut pop: i32, mut seeds: i32, amount_to_plant: i32) -> (i32, i32) {
    seeds = (seeds - amount_to_plant) + (amount_to_plant as f32 * consts::SEED_GROWTH_RATE) as i32;
    if seeds < pop {
        pop = seeds;
        seeds = 0;
    } else {
        seeds = seeds - pop;
        pop = (pop as f32 * consts::POP_GROWTH_RATE) as i32;
    }
    if pop < consts::POP_BELOW_END_GAME {
        pop = 0;
    }
    (pop, seeds)
}

pub fn buy_land(mut seeds: i32, mut land: i32) -> (bool, i32, i32) {
    let mut bought: bool = false;
    let mut input_amount: i32 = -1;
    while input_amount == -1 {
        io::clear_screen();
        ascii::land();
        println!("");
        println!("====[Buying Land]====");
        println!("Current seeds: {}", seeds);
        println!("Current land: {}", land);
        println!("Exchange {} seeds per 1 land", consts::LAND_SEED_COST);
        println!("How much land would you like to buy?");
        input_amount = io::accept_number();
    }
    let amount_to_buy: i32 = input_amount;
    if amount_to_buy * consts::LAND_SEED_COST > seeds {
        io::clear_screen();
        println!("You cannot buy that much land!");
        io::accept_input();
    } else {
        land = land + amount_to_buy;
        seeds = seeds - (amount_to_buy * consts::LAND_SEED_COST);
        bought = true;
    }
    (bought, seeds, land)
}

pub fn sell_land(mut seeds: i32, mut land: i32) -> (bool, i32, i32) {
    let mut sold: bool = false;
    let mut input_amount: i32 = -1;
    while input_amount == -1 {
        io::clear_screen();
        ascii::land();
        println!("");
        println!("====[Selling Land]====");
        println!("Current seeds: {}", seeds);
        println!("Current land: {}", land);
        println!("Exchange {} seeds per 1 land", consts::SEED_LAND_COST);
        println!("How much land would you like to sell?");
        input_amount = io::accept_number();
    }
    let amount_to_sell: i32 = input_amount;
    if amount_to_sell > land {
        io::clear_screen();
        println!("You cannot sell that much land!");
        io::accept_input();
    } else {
        land = land - amount_to_sell;
        seeds = seeds + (amount_to_sell * consts::SEED_LAND_COST);
        sold = true;
    }
    (sold, seeds, land)
}

pub fn progress_year(mut year: u8, mut seeds: i32, mut land: i32, mut pop: i32) -> (u8, i32, i32, i32) {
    year = year + 1;
    let (new_seeds, new_land, new_pop) = apply_random_events(seeds, land, pop);
    seeds = new_seeds;
    land = new_land;
    pop = new_pop;
    if pop < 0 {
        pop = 0;
    }
    (year, seeds, land, pop)
}

fn apply_random_events(mut seeds: i32, land: i32, mut pop: i32) -> (i32, i32, i32) {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0, 10);
    io::clear_screen();
    match number {
        1 => seeds = good_harvest(seeds),
        2 => seeds = bad_harvest(seeds),
        3 => pop = population_boom(pop),
        4 => pop = barbarian_attack(pop, land),
        5 => pop = disease(pop, seeds),
        _number => no_event()
    }
    io::accept_input();
    (seeds, land, pop)
}

fn good_harvest(seeds: i32) -> i32 {
    println!("You had a more bountiful harvest this year!");
    seeds * 2
}

fn bad_harvest(seeds: i32) -> i32 {
    println!("Crops were wiped out by locusts this year, your seeds stores have taken a hit...");
    seeds / 2
}

fn population_boom(pop: i32) -> i32 {
    if pop > 500 {
        println!("Your people are prospering under your just rule! You have a population explosion going on right now!");
        pop * 2
    } else {
        println!("A normal year concludes, but you wish you had more subjects");
        pop
    }
}

fn barbarian_attack(pop: i32, land: i32) -> i32 {
    if land > pop {
        println!("Barbarians attacked your kingdom, but your people bravely fought them off!");
        pop
    } else if land < 100 {
        println!("Barbarians attacked your kingdom, but you do not have the land in order to fend them off!");
        pop / 2
    } else {
        println!("Barbarians attacked your kingdom, your people fought well, but casualties were made...");
        land
    }
}

fn disease(pop: i32, seeds: i32) -> i32 {
    if seeds > (pop * 2) {
        println!("A plague has swept your kingdom, thankfully your food stores have helped most people recover fully!");
        (pop as f32 * 0.9) as i32
    } else if seeds < 100 {
        println!("A deadly plague has swept your kingdom, wiping out all life...");
        0
    } else {
        println!("A plague has swept your kingdom, many have died");
        pop - 100
    }
}

fn no_event(){
    println!("A normal year concludes.");
}
