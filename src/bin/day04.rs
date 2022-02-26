use regex::Regex;
use std::fmt::Formatter;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct BingoBoard {
    cells: Vec<(u32, bool)>,
}

impl BingoBoard {
    fn new(numbers: Vec<u32>) -> Self {
        Self {
            cells: numbers.iter().map(|n| (*n, false)).collect(),
        }
    }

    fn put_chip(&mut self, called_number: u32) {
        // TODO are numbers on board unique?
        if let Some((_, marked)) = self.cells.iter_mut().find(|(n, _)| *n == called_number) {
            *marked = true;
        }
    }

    fn row_marked(&self, row: usize) -> bool {
        (0..5)
            .map(|i| self.cells[row * 5 + i].1)
            .fold(true, |acc, curr| acc && curr)
    }

    fn col_marked(&self, col: usize) -> bool {
        (0..5)
            .map(|i| self.cells[i * 5 + col].1)
            .fold(true, |acc, curr| acc && curr)
    }

    fn has_bingo(&self) -> bool {
        for i in 0..5 {
            if self.row_marked(i) {
                return true;
            }
            if self.col_marked(i) {
                return true;
            }
        }
        false
    }

    fn unmarked_sum(&self) -> u32 {
        self.cells
            .iter()
            .filter(|(_, marked)| !(*marked))
            .map(|(n, _)| *n)
            .sum()
    }
}

impl std::fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, (n, marked))| {
                let col = i % 5;
                if *marked {
                    write!(f, "{:^2}", "X")?;
                } else {
                    write!(f, "{:^2}", *n)?;
                }
                if col == 4 {
                    write!(f, "\n")
                } else {
                    write!(f, " ")
                }
            })
            .fold(Ok(()), |acc, result| match (acc, result) {
                (Ok(_), Ok(_)) => Ok(()),
                (Ok(_), e) => e,
                (e, _) => e,
            })
    }
}

fn find_first_winner(mut boards: Vec<BingoBoard>, called_numbers: Vec<u32>) -> u32 {
    for n in called_numbers {
        for (_, b) in boards.iter_mut().enumerate() {
            b.put_chip(n);
            if b.has_bingo() {
                return n * b.unmarked_sum();
            }
        }
    }
    panic!("Expected to find first winner.");
}

fn find_last_winner(mut boards: Vec<BingoBoard>, called_numbers: Vec<u32>) -> u32 {
    let mut won: Vec<bool> = (0..boards.len()).map(|_| false).collect();
    for n in called_numbers {
        for (i, b) in boards.iter_mut().enumerate() {
            b.put_chip(n);
            let last_winner = won
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .fold(true, |acc, (_, curr)| acc && *curr);
            if b.has_bingo() {
                if last_winner {
                    return n * b.unmarked_sum();
                }
                won[i] = true;
            }
        }
    }
    panic!("Expected to find last winner");
}

fn main() {
    let part: usize = std::env::args()
        .nth(1)
        .expect("specified puzzle part")
        .parse()
        .expect("valid number");

    let mut called_numbers = Vec::new();
    let mut boards = Vec::new();
    let mut new_board_numbers: Vec<u32> = Vec::new();
    io::stdin()
        .lock()
        .lines()
        .enumerate()
        .for_each(|(line_num, line_result)| match line_result {
            Ok(line) => {
                if line_num == 0 {
                    called_numbers.extend(line.split(",").map(|s| s.parse::<u32>().unwrap()));
                } else if line.len() == 0 && new_board_numbers.len() > 0 {
                    boards.push(BingoBoard::new(new_board_numbers.clone()));
                    new_board_numbers.clear();
                } else if line.len() > 0 {
                    let re = Regex::new(r"\s+").unwrap();
                    let row = re
                        .split(line.as_str())
                        .filter(|&num| num.len() > 0)
                        .map(|s| s.parse::<u32>().unwrap());
                    new_board_numbers.extend(row);
                }
            }
            Err(_) => panic!("should never reach here"),
        });
    boards.push(BingoBoard::new(new_board_numbers.clone()));

    let score = match part {
        1 => find_first_winner(boards, called_numbers),
        2 => find_last_winner(boards, called_numbers),
        _ => panic!("Problem only has two parts. Valid values are 1 and 2."),
    };
    println!("final score is {}", score);
}
