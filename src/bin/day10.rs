use std::io;
use std::io::BufRead;

enum SyntaxError {
    Incomplete(Vec<char>),
    Corrupted(char),
}

struct SyntaxValidator {
    line: String,
}

impl SyntaxValidator {
    fn new(line: String) -> Self {
        Self { line }
    }

    fn parse(&self) -> Result<(), SyntaxError> {
        let mut stack = Vec::new();
        let corrupted_result =
            self.line
                .chars()
                .fold(Ok(()), |acc: Result<(), SyntaxError>, curr_char| {
                    if acc.is_err() {
                        acc
                    } else {
                        match curr_char {
                            '(' | '[' | '{' | '<' => Ok(stack.push(curr_char)),
                            ')' | ']' | '}' | '>' => match stack.pop() {
                                Some(open_char) => match (open_char, curr_char) {
                                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => Ok(()),
                                    ('(', illegal_char) => {
                                        Err(SyntaxError::Corrupted(illegal_char))
                                    }
                                    ('[', illegal_char) => {
                                        Err(SyntaxError::Corrupted(illegal_char))
                                    }
                                    ('{', illegal_char) => {
                                        Err(SyntaxError::Corrupted(illegal_char))
                                    }
                                    ('<', illegal_char) => {
                                        Err(SyntaxError::Corrupted(illegal_char))
                                    }
                                    (_, _) => panic!("invalid character"),
                                },
                                None => panic!("stack should not be empty"),
                            },
                            _ => panic!("invalid character"),
                        }
                    }
                });
        if corrupted_result.is_err() {
            corrupted_result
        } else if !stack.is_empty() {
            Err(SyntaxError::Incomplete(stack))
        } else {
            Ok(())
        }
    }

    fn get_autocomplete_str(&self, mut stack: Vec<char>) -> String {
        let mut autocomplete_str = String::new();
        while !stack.is_empty() {
            match stack.pop() {
                Some(top) => {
                    autocomplete_str.push(match top {
                        '(' => ')',
                        '<' => '>',
                        '[' => ']',
                        '{' => '}',
                        _ => panic!("invalid character"),
                    });
                }
                None => panic!("should not enter here"),
            }
        }
        autocomplete_str
    }

    fn corrupted_score(&self) -> Option<u64> {
        match self.parse() {
            Ok(_) => None,
            Err(SyntaxError::Incomplete(_)) => None,
            Err(SyntaxError::Corrupted(illegal_char)) => Some(match illegal_char {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("{} is an invalid character.", illegal_char),
            }),
        }
    }

    fn autocomplete_score(&self) -> Option<u64> {
        match self.parse() {
            Ok(_) => None,
            Err(SyntaxError::Incomplete(stack)) => {
                let s = self.get_autocomplete_str(stack);
                Some(
                    s.chars()
                        .map(|x| match x {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => 0,
                        })
                        .fold(0, |acc, x| acc * 5 + x),
                )
            }
            Err(SyntaxError::Corrupted(_)) => None,
        }
    }
}

fn main() {
    let part: u8 = std::env::args()
        .nth(1)
        .expect("specified puzzle part")
        .parse()
        .expect("valid number");

    match part {
        1 => {
            let total_score: u64 = io::stdin()
                .lock()
                .lines()
                .filter_map(|line| SyntaxValidator::new(line.unwrap()).corrupted_score())
                .sum();
            println!("total score is {}", total_score);
        }
        2 => {
            let mut scores: Vec<u64> = io::stdin()
                .lock()
                .lines()
                .filter_map(|line| SyntaxValidator::new(line.unwrap()).autocomplete_score())
                .collect();
            scores.sort();
            let middle_index = scores.len() / 2;
            println!("middle score is {}", scores.get(middle_index).unwrap());
        }
        _ => panic!("Problem only has two parts. Valid values are 1 and 2"),
    }
}
