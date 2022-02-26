use std::collections::HashMap;
use std::fmt::Formatter;
use std::io::BufRead;
use std::{fmt, io};

trait DiagnosticReport: fmt::Display {
    fn process(&mut self, line: String);
}

struct DiagnosticReportPart1 {
    row_count: usize,
    one_counts: HashMap<usize, usize>,
}

impl DiagnosticReport for DiagnosticReportPart1 {
    fn process(&mut self, line: String) {
        for (pos, ch) in line.chars().enumerate() {
            if ch == '1' {
                self.one_counts
                    .entry(pos)
                    .and_modify(|e| {
                        *e += 1;
                    })
                    .or_insert(1);
            }
        }
        self.row_count += 1;
    }
}

impl fmt::Display for DiagnosticReportPart1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Gamma rate is {}, epilson rate is {}, and power consumption is {}",
            self.gamma_rate(),
            self.epsilon_rate(),
            self.power_consumption()
        )
    }
}

impl DiagnosticReportPart1 {
    fn new() -> Self {
        Self {
            row_count: 0,
            one_counts: HashMap::new(),
        }
    }

    fn gamma_rate(&self) -> String {
        let mut bit_string = String::new();
        let mut pos: usize = 0;
        while self.one_counts.contains_key(&pos) {
            let &one_count = self.one_counts.get(&pos).unwrap();
            let zero_count = self.row_count - one_count;
            if one_count > zero_count {
                bit_string.push('1');
            } else {
                bit_string.push('0');
            }
            pos += 1;
        }
        bit_string
    }

    fn epsilon_rate(&self) -> String {
        let gr = self.gamma_rate();
        gr.chars()
            .map(|ch| match ch {
                '1' => '0',
                '0' => '1',
                _ => panic!("invalid character expected '0' or '1'."),
            })
            .collect()
    }

    fn power_consumption(&self) -> u32 {
        let gt = u32::from_str_radix(self.gamma_rate().as_str(), 2).unwrap();
        let er = u32::from_str_radix(self.epsilon_rate().as_str(), 2).unwrap();
        gt * er
    }
}

struct DiagnosticReportPart2 {
    rows: Vec<String>,
    one_counts: HashMap<usize, usize>,
}

impl DiagnosticReport for DiagnosticReportPart2 {
    fn process(&mut self, bit_str: String) {
        self.rows.push(bit_str.clone());
        for (pos, ch) in bit_str.chars().enumerate() {
            //println!("pos: {} ch: {}",pos, ch);
            if ch == '1' {
                if self.one_counts.contains_key(&pos) {
                    self.one_counts.entry(pos).and_modify(|e| {
                        *e += 1;
                    });
                } else {
                    self.one_counts.insert(pos, 1);
                }
            }
        }
    }
}

impl fmt::Display for DiagnosticReportPart2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
            "oxygen generator rating is {} and co2 scrubber rating is {}, so life support rating is {}",
               self.oxygen_generator_rating(),
               self.co2_scrubber_rating(),
               self.life_support_rating()
        )
    }
}

impl DiagnosticReportPart2 {
    fn new() -> Self {
        Self {
            rows: Vec::new(),
            one_counts: HashMap::new(),
        }
    }

    fn oxygen_generator_rating(&self) -> u32 {
        let mut rows: Vec<&String> = self.rows.iter().map(|x| x).collect();
        let mut pos = 0;
        while rows.len() > 1 {
            let one_count: usize = rows
                .iter()
                .map(|x| {
                    if x.chars().nth(pos) == Some('1') {
                        1
                    } else {
                        0
                    }
                })
                .sum();
            let zero_count = rows.len() - one_count;
            if one_count >= zero_count {
                rows = rows
                    .into_iter()
                    .filter(|&x| x.chars().nth(pos) == Some('1'))
                    .collect();
            } else {
                rows = rows
                    .into_iter()
                    .filter(|&x| x.chars().nth(pos) == Some('0'))
                    .collect();
            }
            pos += 1;
        }
        u32::from_str_radix(rows.first().unwrap(), 2).unwrap()
    }

    fn co2_scrubber_rating(&self) -> u32 {
        let mut rows: Vec<&String> = self.rows.iter().map(|x| x).collect();
        let mut pos = 0;
        while rows.len() > 1 {
            let one_count: usize = rows
                .iter()
                .map(|x| {
                    if x.chars().nth(pos) == Some('1') {
                        1
                    } else {
                        0
                    }
                })
                .sum();
            let zero_count = rows.len() - one_count;
            if zero_count <= one_count {
                rows = rows
                    .into_iter()
                    .filter(|&x| x.chars().nth(pos) == Some('0'))
                    .collect();
            } else {
                rows = rows
                    .into_iter()
                    .filter(|&x| x.chars().nth(pos) == Some('1'))
                    .collect();
            }
            pos += 1;
        }
        u32::from_str_radix(rows.first().unwrap(), 2).unwrap()
    }

    fn life_support_rating(&self) -> u32 {
        self.oxygen_generator_rating() * self.co2_scrubber_rating()
    }
}

fn main() {
    let part: usize = std::env::args()
        .nth(1)
        .expect("specified puzzle part")
        .parse()
        .expect("valid number");

    let mut report: Box<dyn DiagnosticReport> = match part {
        1 => Box::new(DiagnosticReportPart1::new()),
        2 => Box::new(DiagnosticReportPart2::new()),
        _ => panic!("Problem only has two parts. Valid values are 1 and 2."),
    };
    for line in io::stdin().lock().lines() {
        report.process(line.unwrap());
    }
    println!("{}", report);
}
