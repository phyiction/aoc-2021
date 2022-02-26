use std::{fmt, io};
use std::cell::RefCell;
use std::fmt::Formatter;
use std::io::BufRead;
use std::rc::Rc;

#[derive(PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn index(&self, row_width: usize) -> usize {
        self.row * row_width + self.col
    }

    fn north(&self) -> Option<Self> {
        if self.row > 0 {
            Some(Self::new(self.row - 1, self.col))
        } else {
            None
        }
    }

    fn north_east(&self, row_width: usize) -> Option<Self> {
        if self.row > 0 && self.col + 1 < row_width {
            Some(Self::new(self.row - 1, self.col + 1))
        } else {
            None
        }
    }

    fn east(&self, row_width: usize) -> Option<Self> {
        if self.col + 1 < row_width {
            Some(Self::new(self.row, self.col + 1))
        } else {
            None
        }
    }

    fn south_east(&self, row_count: usize, row_width: usize) -> Option<Self> {
        if self.row + 1 < row_count && self.col + 1 < row_width {
            Some(Self::new(self.row + 1, self.col + 1))
        } else {
            None
        }
    }

    fn south(&self, row_count: usize) -> Option<Self> {
        if self.row + 1 < row_count {
            Some(Self::new(self.row + 1, self.col))
        } else {
            None
        }
    }

    fn south_west(&self, row_count: usize) -> Option<Self> {
        if self.row + 1 < row_count && self.col > 0 {
            Some(Self::new(self.row + 1, self.col - 1))
        } else {
            None
        }
    }

    fn west(&self) -> Option<Self> {
        if self.col > 0 {
            Some(Self::new(self.row, self.col - 1))
        } else {
            None
        }
    }

    fn north_west(&self) -> Option<Self> {
        if self.row > 0 && self.col > 0 {
            Some(Self::new(self.row - 1, self.col - 1))
        } else {
            None
        }
    }

    fn neighbors(&self, row_count: usize, row_width: usize) -> Vec<Option<Position>> {
        vec![
            self.north(),
            self.north_east(row_width),
            self.east(row_width),
            self.south_east(row_count, row_width),
            self.south(row_count),
            self.south_west(row_count),
            self.west(),
            self.north_west(),
        ]
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Octopus {
    position: Position,
    energy_level: u32,
    flashed: bool,
}

impl Octopus {
    fn new(position: Position, energy_level: u32) -> Self {
        Self {
            position,
            energy_level,
            flashed: false,
        }
    }
}

impl fmt::Debug for Octopus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.energy_level)
    }
}

struct OctopusSimulation {
    octopi: Vec<Rc<RefCell<Octopus>>>,
    width: usize,
}

impl OctopusSimulation {
    fn new(input: Vec<String>) -> Self {
        Self {
            octopi: input
                .iter()
                .enumerate()
                .map(|(r, row)| (r, row.chars()))
                .fold(Vec::new(), |mut acc, (r, chars)| {
                    acc.extend(chars.enumerate().map(|(c, x)| {
                        Rc::new(RefCell::new(Octopus::new(
                            Position::new(r, c),
                            x.to_digit(10).unwrap(),
                        )))
                    }));
                    acc
                }),
            width: 10,
        }
    }

    fn neighbors(&self, position: &Position) -> Vec<&Rc<RefCell<Octopus>>> {
        let row_count = self.octopi.len() / self.width;
        position
            .neighbors(row_count, self.width)
            .iter()
            .filter_map(|p| {
                if p.is_some() {
                    self.octopi.get(p.as_ref().unwrap().index(self.width))
                }else{
                    None
                }
            })
            .collect()
    }

    fn step(&self, octopus: Rc<RefCell<Octopus>>) {
        let mut queue = Vec::new();
        queue.push(octopus);
        while !queue.is_empty() {
            match queue.pop() {
                Some(octo) => {
                    let mut o = octo.borrow_mut();
                    if !o.flashed {
                        o.energy_level += 1;
                        if o.energy_level > 9 {
                            o.energy_level = 0;
                            o.flashed = true;
                            for &neighbor in self.neighbors(&o.position).iter() {
                                queue.push(neighbor.clone());
                            }
                        }
                    }
                }
                None => break,
            }
        }
    }

    fn get_flash_count(&mut self, num_steps: usize) -> u32 {
        println!("{}", self);
        let mut flash_count = 0;
        for step in 1..num_steps + 1 {
            for octopus in self.octopi.iter() {
                self.step(octopus.clone());
            }
            println!("After step {}\n{}", step, self);
            for octopus in self.octopi.iter() {
                let mut o = octopus.borrow_mut();
                if o.flashed {
                    flash_count += 1;
                    o.flashed = false;
                }
            }
        }
        flash_count
    }

    fn all_flash_at_step(&mut self) -> u32 {
        println!("{}", self);
        let mut step = 0;
        let mut all_flash = false;
        while !all_flash {
            for octopus in self.octopi.iter() {
                self.step(octopus.clone());
            }
            println!("After step {}\n{}", step, self);
            all_flash = true;
            for octopus in self.octopi.iter() {
                let mut o = octopus.borrow_mut();
                all_flash = all_flash && o.flashed;
                if o.flashed {
                    o.flashed = false;
                }
            }
            step += 1;
        }
        step
    }
}

impl fmt::Display for OctopusSimulation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, o) in self.octopi.iter().enumerate() {
            write!(f, "{:?}", o.borrow())?;
            if i % self.width == self.width - 1 {
                write!(f, "\n")?;
            }
        }
        write!(f, "")
    }
}

fn main() {
    let part: u8 = std::env::args()
        .nth(1)
        .expect("specified puzzle part")
        .parse()
        .expect("valid number");
    let input: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let mut sim = OctopusSimulation::new(input);
    match part {
        1 => println!("{} flashes after 100 steps.", sim.get_flash_count(100)),
        2 => println!("First step all octopi flash is {}", sim.all_flash_at_step()),
        _ => panic!("Problem only has two parts. Valid values are 1 and 2"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_neighbors_nw_corner() {
        assert_eq!(
            vec![
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(1, 0),
            ],
            Position::new(0, 0).neighbors(10, 10),
        );
    }

    #[test]
    fn test_position_neighbors_ne_corner() {
        assert_eq!(
            vec![
                Position::new(1, 9),
                Position::new(1, 8),
                Position::new(0, 8),
            ],
            Position::new(0, 9).neighbors(10, 10),
        );
    }

    #[test]
    fn test_position_neighbors_se_corner() {
        assert_eq!(
            vec![
                Position::new(8, 9),
                Position::new(9, 8),
                Position::new(8, 8),
            ],
            Position::new(9, 9).neighbors(10, 10),
        );
    }

    #[test]
    fn test_position_neighbors_sw_corner() {
        assert_eq!(
            vec![
                Position::new(8, 0),
                Position::new(8, 1),
                Position::new(9, 1),
            ],
            Position::new(9, 0).neighbors(10, 10),
        );
    }

    #[test]
    fn test_position_neighbors_center() {
        assert_eq!(
            vec![
                Position::new(4, 5),
                Position::new(4, 6),
                Position::new(5, 6),
                Position::new(6, 6),
                Position::new(6, 5),
                Position::new(6, 4),
                Position::new(5, 4),
                Position::new(4, 4),
            ],
            Position::new(5, 5).neighbors(10, 10),
        );
    }

    macro_rules! octopus_simulation_get_flash_count_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (steps, expected) = $value;
                let input = vec![
                    "5483143223",
                    "2745854711",
                    "5264556173",
                    "6141336146",
                    "6357385478",
                    "4167524645",
                    "2176841721",
                    "6882881134",
                    "4846848554",
                    "5283751526",
                ]
                .iter()
                .map(|&x| x.to_string())
                .collect();
                let mut sim = OctopusSimulation::new(input);
                sim.get_flash_count(steps);
                assert_eq!(expected, format!("{}", sim));
            }
        )*
        }
    }

    octopus_simulation_get_flash_count_tests! {
        steps_1: (1, "6594254334\n3856965822\n6375667284\n7252447257\n7468496589\n5278635756\n3287952832\n7993992245\n5957959665\n6394862637\n"),
        steps_2: (2, "8807476555\n5089087054\n8597889608\n8485769600\n8700908800\n6600088989\n6800005943\n0000007456\n9000000876\n8700006848\n"),
        steps_3: (3, "0050900866\n8500800575\n9900000039\n9700000041\n9935080063\n7712300000\n7911250009\n2211130000\n0421125000\n0021119000\n"),
        steps_4: (4, "2263031977\n0923031697\n0032221150\n0041111163\n0076191174\n0053411122\n0042361120\n5532241122\n1532247211\n1132230211\n"),
    }
}
