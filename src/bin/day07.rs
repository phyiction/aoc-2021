use std::io;
use std::io::BufRead;

type CostFunction = Box<dyn Fn(u32) -> u32>;

fn get_cost_function(part: usize) -> CostFunction {
    Box::new(match part {
        1 => |steps| steps,
        2 => |steps| steps * (1 + steps) / 2,
        _ => panic!("Problem only has 2 parts. Valid values are 1 and 2."),
    })
}

fn cheapest_alignment(positions: &Vec<u32>, cost: &CostFunction) -> (u32, u32) {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let mut min_cost = u32::MAX;
    let mut cheapest_pos = min;
    for pos in min..(max + 1) {
        let cost = fuel_spent(positions, pos, cost);
        if cost < min_cost {
            min_cost = cost;
            cheapest_pos = pos;
        }
    }
    (cheapest_pos, min_cost)
}

fn fuel_spent(positions: &Vec<u32>, align_at: u32, cost: &CostFunction) -> u32 {
    let mut total_fuel = 0;
    for pos in positions.iter() {
        let steps = if align_at > *pos {
            align_at - *pos
        } else {
            *pos - align_at
        };
        total_fuel += cost(steps);
    }
    total_fuel
}

fn main() {
    let part: usize = std::env::args()
        .nth(1)
        .expect("specified puzzle part")
        .parse()
        .expect("valid number");

    let mut input = String::new();
    match io::stdin().lock().read_line(&mut input) {
        Ok(_) => {
            let curr_positions: Vec<u32> = input.split(',').map(|n| n.parse().unwrap()).collect();
            let cost_function = get_cost_function(part);
            let (pos, cost) = cheapest_alignment(&curr_positions, &cost_function);
            println!("Cheapest alignment is {} and fuel cost is {}", pos, cost);
        }
        Err(_) => {}
    }
}
