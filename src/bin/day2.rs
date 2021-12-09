use std::fmt;
use std::io;
use std::io::BufRead;

trait Submarine {
    fn forward(&mut self, x: u32);
    fn down(&mut self, x: u32);
    fn up(&mut self, x: u32);
    fn position(&self) -> Coordinates;
}

#[derive(Clone)]
struct Coordinates {
    /// horizontal position
    x: u32,
    /// vertical position
    y: u32,
}

impl Coordinates {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y,)
    }
}

struct SubmarinePart1 {
    position: Coordinates,
}

impl SubmarinePart1 {
    fn new() -> Self {
        Self {
            position: Coordinates::new(),
        }
    }
}

impl Submarine for SubmarinePart1 {
    fn forward(&mut self, distance: u32) {
        self.position.x += distance;
    }

    fn down(&mut self, distance: u32) {
        self.position.y += distance;
    }

    fn up(&mut self, distance: u32) {
        self.position.y -= distance;
    }

    fn position(&self) -> Coordinates {
        self.position.clone()
    }
}

impl fmt::Display for SubmarinePart1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ position: {} }}", self.position)
    }
}

struct SubmarinePart2 {
    position: Coordinates,
    aim: u32,
}

impl SubmarinePart2 {
    fn new() -> Self {
        Self {
            position: Coordinates::new(),
            aim: 0,
        }
    }
}

impl Submarine for SubmarinePart2 {
    fn forward(&mut self, distance: u32) {
        self.position.x += distance;
        self.position.y += self.aim * distance;
    }

    fn down(&mut self, distance: u32) {
        self.aim += distance;
    }

    fn up(&mut self, distance: u32) {
        self.aim -= distance;
    }

    fn position(&self) -> Coordinates {
        self.position.clone()
    }
}

impl fmt::Display for SubmarinePart2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ position: {}, aim: {} }}", self.position, self.aim)
    }
}

fn main() {
    let part: usize = std::env::args()
        .nth(1)
        .expect("specified puzzle part")
        .parse()
        .expect("valid number");

    let mut sub: Box<dyn Submarine> = match part {
        1 => Box::new(SubmarinePart1::new()),
        2 => Box::new(SubmarinePart2::new()),
        _ => panic!("Problem only has two parts. Valid values are 1 and 2."),
    };
    for line in io::stdin().lock().lines() {
        let instruction = line.unwrap();
        let mut iter = instruction.split(" ").into_iter();
        let direction = iter.next().unwrap();
        let distance: u32 = iter.next().unwrap().parse().unwrap();
        match direction {
            "forward" => sub.forward(distance),
            "down" => sub.down(distance),
            "up" => sub.up(distance),
            _ => panic!("should not enter here!"),
        }
    }
    let sub_position = sub.position();
    println!(
        "Submarine's location is {} and product is {}",
        sub_position,
        sub_position.x * sub_position.y
    );
}
