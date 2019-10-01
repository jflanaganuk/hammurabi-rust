mod kingdom;
mod io;
mod consts;
mod ascii;

fn main() {
    let mut land: i32 = 150;
    let mut pop: i32 = 100;
    let mut seeds: i32 = 250;
    let mut year: u8 = 1;
    let mut quit: bool = false;
    let mut dead: bool = false;
    let mut win: bool = false;
    let mut showing_error_message: bool = false;
    let mut score: i32 = 0;

    while !quit {
        if score > 100000 {
            quit = true;
            win = true;
        } else if showing_error_message {
            println!("Press enter to continue...");
            let input: String = io::accept_input();
            if input == "" {
                showing_error_message = false;
                io::clear_screen();
            }
        } else {
            io::clear_screen();
            io::starting_output(year, land, pop, seeds);
            let input: String = io::accept_input();
            io::clear_screen();
            if input == "1" {
                let mut planted: bool = false;
                while !planted {
                    let (new_planted, new_seeds, new_pop) = kingdom::plant_seeds(seeds, land, pop);
                    seeds = new_seeds;
                    pop = new_pop;
                    planted = new_planted;
                }
                if pop <= 0 {
                    quit = true;
                    dead = true;
                }
                let (new_year, new_seeds, new_land, new_pop) = kingdom::progress_year(year, seeds, land, pop);
                year = new_year;
                seeds = new_seeds;
                land = new_land;
                pop = new_pop;
            }
            else if input == "2" {
                let mut bought: bool = false;
                while !bought {
                    let (new_bought, new_seeds, new_land) = kingdom::buy_land(seeds, land);
                    bought = new_bought;
                    seeds = new_seeds;
                    land = new_land;
                }
            }
            else if input == "3" {
                let mut sold: bool = false;
                while !sold {
                    let (new_sold, new_seeds, new_land) = kingdom::sell_land(seeds, land);
                    sold = new_sold;
                    seeds = new_seeds;
                    land = new_land;
                }
            }
            else if input == "4" {
                ascii::clock();
                println!("");
                println!("Progressing kingdom... (press enter)");
                io::accept_input();
                let (new_pop, new_seeds) = kingdom::process_seed_planting(pop, seeds, 0);
                pop = new_pop;
                seeds = new_seeds;
                if pop <= 0 {
                    quit = true;
                    dead = true;
                }
                let (new_year, new_seeds, new_land, new_pop) = kingdom::progress_year(year, seeds, land, pop);
                year = new_year;
                seeds = new_seeds;
                land = new_land;
                pop = new_pop;
            }
            else if input == "q" {
                quit = true;
            }
            else {
                io::clear_screen();
                println!("Unrecognised input");
                showing_error_message = true;
            }
            score += pop + seeds + land;
        }
    }
    io::clear_screen();
    if dead {
        println!("Your kingdom fell...");
        println!("Your final score was: {}", score);
    } else if win {
        ascii::victory_image();
        println!("Your score is greater than 100000");
        println!("You have beaten the game! Congratulations!");
    } else {
        println!("Bye");
    }
}
