use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::io;
use std::io::BufRead;
use std::iter;
use std::iter::FromIterator;

fn freq_of_1_4_7_8(output: Vec<&str>) -> u32 {
    output
        .iter()
        .map(|display| {
            match display.len() {
                2 => 1, // 2 segments for 1,
                4 => 1, // 4 segments for 4
                3 => 1, // 3 segments for 7
                7 => 1, // 7 segments for 8
                _ => 0,
            }
        })
        .sum::<u32>()
}

///
///  0000
/// 1    2
/// 1    2
///  3333
/// 4    5
/// 5    5
///  6666
struct SevenSegmentDisplay {
    segments: [Option<char>; 7],
}

impl SevenSegmentDisplay {
    fn new(segments: [Option<char>; 7]) -> Self {
        Self { segments }
    }

    fn from_signals(signal_patterns: Vec<&str>) -> Self {
        let mut segments = [None; 7];

        let &one = signal_patterns.iter().find(|x| x.len() == 2).unwrap();
        let &four = signal_patterns.iter().find(|x| x.len() == 4).unwrap();
        let &seven = signal_patterns.iter().find(|x| x.len() == 3).unwrap();

        // find segment at position 0 from 1 and 7
        let seven_set: HashSet<char> = HashSet::from_iter(seven.chars());
        let one_set: HashSet<char> = HashSet::from_iter(one.chars());
        let intersection_set: HashSet<char> =
            one_set.intersection(&seven_set).map(|x| *x).collect();
        assert_eq!(1, seven_set.difference(&intersection_set).count());
        if let Some(ch) = seven_set.difference(&intersection_set).next() {
            segments[0] = Some(*ch);
        }

        // find segment at position 3 from 4 and signal patterns of length 5 (2, 3, 5)
        let four_set = HashSet::from_iter(four.chars());
        let intersection_set = signal_patterns
            .iter()
            .filter(|&&x| x.len() == 5)
            .map(|&x| HashSet::from_iter(x.chars()))
            .fold(four_set, |acc: HashSet<char>, x: HashSet<char>| {
                acc.intersection(&x).map(|x| *x).collect()
            });
        assert_eq!(1, intersection_set.iter().count());
        if let Some(ch) = intersection_set.iter().next() {
            segments[3] = Some(*ch);
        }

        // find segment at position 1
        let four_set: HashSet<char> = HashSet::from_iter(four.chars());
        let mut one_set = HashSet::from_iter(one.chars());
        one_set.insert(segments[3].unwrap());
        let intersection_set = four_set.intersection(&one_set).map(|x| *x).collect();
        assert_eq!(1, four_set.difference(&intersection_set).count());
        if let Some(ch) = four_set.difference(&intersection_set).next() {
            segments[1] = Some(*ch);
        }

        // find segment at position 6
        let mut init_set: HashSet<char> = HashSet::new();
        init_set.insert(segments[0].unwrap());
        init_set.insert(segments[3].unwrap());
        let intersection_set = signal_patterns
            .iter()
            .filter(|&&x| x.len() == 5)
            .map(|&x| HashSet::from_iter(x.chars()))
            .fold(None, |acc: Option<HashSet<char>>, x: HashSet<char>| {
                if acc.is_none() {
                    Some(x)
                } else {
                    Some(acc.unwrap().intersection(&x).map(|x| *x).collect())
                }
            })
            .unwrap();
        assert_eq!(1, intersection_set.difference(&init_set).count());
        if let Some(ch) = intersection_set.difference(&init_set).next() {
            segments[6] = Some(*ch);
        }

        // find segment at position 5 from signal patterns of length 6
        let intersection_set = signal_patterns
            .iter()
            .filter(|&&x| x.len() == 6)
            .map(|&x| HashSet::from_iter(x.chars()))
            .fold(None, |acc: Option<HashSet<char>>, x: HashSet<char>| {
                if acc.is_none() {
                    Some(x)
                } else {
                    Some(acc.unwrap().intersection(&x).map(|x| *x).collect())
                }
            })
            .unwrap();
        let mut init_set = HashSet::new();
        init_set.insert(segments[0].unwrap());
        init_set.insert(segments[1].unwrap());
        init_set.insert(segments[6].unwrap());
        assert_eq!(1, intersection_set.difference(&init_set).count());
        if let Some(ch) = intersection_set.difference(&init_set).next() {
            segments[5] = Some(*ch);
        }

        // find segment at position 2
        let mut one_set: HashSet<char> = HashSet::from_iter(one.chars());
        one_set.remove(&segments[5].unwrap());
        assert_eq!(1, one_set.iter().count());
        if let Some(ch) = one_set.iter().next() {
            segments[2] = Some(*ch);
        }

        // find segment at position 4
        let &eight = signal_patterns.iter().find(|x| x.len() == 7).unwrap();
        let eight_set: HashSet<char> = HashSet::from_iter(eight.chars());
        let mut init_set: HashSet<char> = HashSet::new();
        for i in 0..7 {
            if segments[i].is_some() {
                init_set.insert(segments[i].unwrap());
            }
        }
        assert_eq!(1, eight_set.difference(&init_set).count());
        if let Some(ch) = eight_set.difference(&init_set).next() {
            segments[4] = Some(*ch);
        }

        Self::new(segments)
    }

    fn decode(&self, output: &str) -> char {
        let mut lit = [false; 7];
        for ch in output.chars() {
            match self
                .segments
                .iter()
                .enumerate()
                .find(|(_, &x)| x.is_some() && x.unwrap() == ch)
            {
                Some((pos, _)) => {
                    lit[pos] = true;
                }
                None => panic!("Expect segments to be known."),
            }
        }
        match lit {
            [true, true, true, false, true, true, true] => '0',
            [false, false, true, false, false, true, false] => '1',
            [true, false, true, true, true, false, true] => '2',
            [true, false, true, true, false, true, true] => '3',
            [false, true, true, true, false, true, false] => '4',
            [true, true, false, true, false, true, true] => '5',
            [true, true, false, true, true, true, true] => '6',
            [true, false, true, false, false, true, false] => '7',
            [true, true, true, true, true, true, true] => '8',
            [true, true, true, true, false, true, true] => '9',
            _ => panic!("Invalid digit {:?}", lit),
        }
    }
}

impl fmt::Display for SevenSegmentDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut i = 0;
        for r in 0..7 {
            if r == 0 || r == 3 || r == 6 {
                let ch = self.segments[i].unwrap_or('.');
                let repeated_ch: String = iter::repeat(ch).take(4).collect();
                writeln!(f, "  {}  ", repeated_ch)?;
                i += 1;
            } else {
                let left_ch = self.segments[i].unwrap_or('.');
                let right_ch = self.segments[i + 1].unwrap_or('.');
                writeln!(f, " {}    {} ", left_ch, right_ch)?;
                if r == 2 || r == 5 {
                    i += 2;
                }
            }
        }
        write!(f, "\n")
    }
}

fn main() {
    let part: usize = std::env::args()
        .nth(1)
        .expect("specified puzzle part")
        .parse()
        .expect("valid number");

    let total: u32 = io::stdin()
        .lock()
        .lines()
        .map(|line: Result<String, io::Error>| {
            let line = line.unwrap();
            let split: Vec<&str> = line.as_str().split("|").collect();
            let signal_patterns: Vec<&str> = split
                .get(0)
                .unwrap()
                .split(' ')
                .filter(|&x| x.len() > 0)
                .collect();
            let output: Vec<&str> = split
                .get(1)
                .unwrap()
                .split(' ')
                .filter(|&x| x.len() > 0)
                .collect();
            match part {
                1 => freq_of_1_4_7_8(output),
                2 => {
                    let display = SevenSegmentDisplay::from_signals(signal_patterns);
                    let output_str: String = output.iter().map(|&x| display.decode(x)).collect();
                    output_str.parse().unwrap()
                }
                _ => panic!("Problem only has 2 parts. Valid values are 1 or 2."),
            }
        })
        .sum::<u32>();
    match part {
        1 => println!("1, 4, 7, or 8 appear {} times", total),
        2 => println!("Sum is {}", total),
        _ => panic!("Problem only has 2 parts. Valid values are 1 or 2."),
    }
}

#[cfg(test)]
mod tests {
    use crate::SevenSegmentDisplay;

    #[test]
    fn test_seven_segment_display_decode() {
        let display = SevenSegmentDisplay::new([
            Some('d'),
            Some('e'),
            Some('a'),
            Some('f'),
            Some('g'),
            Some('b'),
            Some('c'),
        ]);
        println!("{}", display);
        assert_eq!('0', display.decode("deagbc"));
        assert_eq!('1', display.decode("ab"));
        assert_eq!('2', display.decode("dafgc"));
        assert_eq!('3', display.decode("fcadb"));
        assert_eq!('4', display.decode("efab"));
        assert_eq!('5', display.decode("cdfeb"));
        assert_eq!('6', display.decode("defgbc"));
        assert_eq!('7', display.decode("dab"));
        assert_eq!('8', display.decode("deafgbc"));
        assert_eq!('9', display.decode("deafbc"));
    }

    #[test]
    fn test_seven_segment_display_from_signals() {
        let display = SevenSegmentDisplay::from_signals(vec![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ]);
        let actual: Vec<char> = display.segments.iter().map(|x| x.unwrap()).collect();
        assert_eq!(vec!['d', 'e', 'a', 'f', 'g', 'b', 'c'], actual);
    }
}
