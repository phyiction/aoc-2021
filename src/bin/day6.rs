use std::io::BufRead;
use std::{env, io};

fn simulate(fish_ages: Vec<u32>, days: u32) {
    // fish age -> quantity
    let mut curr_fish_ages: [u64; 9] = [0; 9];
    for age in fish_ages.iter() {
        curr_fish_ages[*age as usize] += 1;
    }
    for _ in 0..days {
        let num_new_fish = curr_fish_ages[0];
        for age in 1..9 {
            curr_fish_ages[age - 1] = curr_fish_ages[age];
        }
        curr_fish_ages[6] += num_new_fish;
        curr_fish_ages[8] = num_new_fish;
    }
    let mut total_fish = 0;
    for qty in curr_fish_ages.iter() {
        total_fish += *qty;
    }
    println!(
        "There will be {} laternfish after {} days",
        total_fish, days
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let days = if args.len() == 1 {
        80
    } else {
        args[1].parse().unwrap()
    };

    let mut input = String::new();
    match io::stdin().lock().read_line(&mut input) {
        Ok(_) => {
            let ages: Vec<u32> = input.split(',').map(|n| n.parse().unwrap()).collect();

            simulate(ages, days);
        }
        Err(_) => {}
    }
}
