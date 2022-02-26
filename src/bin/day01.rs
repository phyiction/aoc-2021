use std::io;
use std::io::BufRead;

fn increased_count(window_size: usize) -> u32 {
    let mut count = 0_u32;
    let mut buffer: Vec<u32> = Vec::new();
    for line in io::stdin().lock().lines() {
        if buffer.len() < window_size {
            let curr: u32 = line.unwrap().parse().unwrap();
            buffer.push(curr);
        }

        if buffer.len() == window_size {
            let a = buffer.first().unwrap();
            let b = buffer.last().unwrap();
            if b > a {
                count += 1;
            }
            buffer.remove(0);
        }
    }
    count
}

fn main() {
    let part: usize = std::env::args()
        .nth(1)
        .expect("specified puzzle part")
        .parse()
        .expect("valid number");

    let count = match part {
        1 => increased_count(2),
        2 => increased_count(4),
        _ => panic!("Problem only has two parts. Valid values are 1 and 2."),
    };
    println!("Increased count is {}", count);
}
