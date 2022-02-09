use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::io;
use std::io::BufRead;

trait CoordinatePlaneObject {
    fn draw(&self) -> Result<Vec<Point>, String>;
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn new(from: Point, to: Point) -> Self {
        Self { from, to }
    }
}

impl CoordinatePlaneObject for Line {
    fn draw(&self) -> Result<Vec<Point>, String> {
        if self.from.x == self.to.x {
            // vertical line
            Ok((0..(self.to.y - self.from.y).abs() + 1)
                .map(|off| Point::new(self.from.x, cmp::min(self.to.y, self.from.y) + off))
                .collect())
        } else if self.from.y == self.to.y {
            // horizontal line
            Ok((0..(self.to.x - self.from.x).abs() + 1)
                .map(|off| Point::new(cmp::min(self.to.x, self.from.x) + off, self.from.y))
                .collect())
        } else {
            Err(format!("{:?} is not an horizontal or vertical line", self))
        }
    }
}

#[derive(Debug)]
struct LineV2 {
    line: Line,
}

impl LineV2 {
    fn new(from: Point, to: Point) -> Self {
        Self {
            line: Line::new(from, to),
        }
    }
}

impl CoordinatePlaneObject for LineV2 {
    fn draw(&self) -> Result<Vec<Point>, String> {
        let to = &self.line.to;
        let from = &self.line.from;
        let diff_y = to.y - from.y;
        let diff_x = to.x - from.x;
        if from.x == to.x || from.y == to.y {
            // vertical or horizontal line
            self.line.draw()
        } else if diff_x.abs() == diff_y.abs() {
            Ok((0..diff_x.abs() + 1)
                .map(|off| {
                    let x_sign = if diff_x < 0 { -1 } else { 1 };
                    let y_sign = if diff_y < 0 { -1 } else { 1 };
                    Point::new(from.x + off * x_sign, from.y + off * y_sign)
                })
                .collect())
        } else {
            Err(format!(
                "{:?} is not an horizontal, vertical, or diagonal (45 degree) line",
                self
            ))
        }
    }
}

struct CoordinatePlane {
    filled: HashMap<Point, i32>,
}

impl CoordinatePlane {
    fn new() -> Self {
        Self {
            filled: HashMap::new(),
        }
    }

    fn add(&mut self, obj: Box<dyn CoordinatePlaneObject>) {
        match obj.draw() {
            Ok(filled_points) => {
                for pt in filled_points {
                    self.filled
                        .entry(pt)
                        .and_modify(|freq| *freq = *freq + 1)
                        .or_insert(1);
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    fn num_overlaps(&self) -> i32 {
        self.filled
            .values()
            .fold(0, |acc, &freq| if freq > 1 { acc + 1 } else { acc })
    }
}

impl fmt::Display for CoordinatePlane {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max_point = self.filled.keys().fold(Point::new(0, 0), |acc, pt| {
            Point::new(cmp::max(acc.x, pt.x), cmp::max(acc.y, pt.y))
        });
        for y in 0..max_point.y + 1 {
            for x in 0..max_point.x + 1 {
                let key = Point::new(x, y);
                if self.filled.contains_key(&key) {
                    write!(f, "{}", self.filled.get(&key).unwrap())?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
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

    let mut plane = CoordinatePlane::new();
    let re = Regex::new(r"(\d+),(\d+) \-> (\d+),(\d+)").unwrap();
    io::stdin().lock().lines().for_each(|line_result| {
        for cap in re.captures_iter(line_result.unwrap().as_str()) {
            let x1: i32 = cap[1].parse().unwrap();
            let y1: i32 = cap[2].parse().unwrap();

            let x2: i32 = cap[3].parse().unwrap();
            let y2: i32 = cap[4].parse().unwrap();

            let line: Box<dyn CoordinatePlaneObject> = match part {
                1 => Box::new(Line::new(Point::new(x1, y1), Point::new(x2, y2))),
                2 => Box::new(LineV2::new(Point::new(x1, y1), Point::new(x2, y2))),
                _ => panic!("Problem only has two parts. Valid values are 1 and 2."),
            };
            plane.add(line);
        }
    });
    println!("There are {} points that overlap.", plane.num_overlaps());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_vertical_line_draw() {
        let a = Point::new(1, 1);
        let b = Point::new(1, 3);
        let line = Line::new(a, b);
        let points = line.draw().unwrap();
        assert_eq!(
            points,
            vec![Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)]
        )
    }
    #[test]
    fn test_horizontal_line_draw() {
        let a = Point::new(9, 7);
        let b = Point::new(7, 7);
        let line = Line::new(a, b);
        let points = line.draw().unwrap();
        assert_eq!(
            points,
            vec![Point::new(7, 7), Point::new(8, 7), Point::new(9, 7)]
        )
    }

    #[test]
    fn test_diagonal_line_draw1() {
        let a = Point::new(1, 1);
        let b = Point::new(3, 3);
        let line = LineV2::new(a, b);
        let points = line.draw().unwrap();
        assert_eq!(
            points,
            vec![Point::new(1, 1), Point::new(2, 2), Point::new(3, 3)]
        )
    }

    #[test]
    fn test_diagonal_line_draw2() {
        let a = Point::new(9, 7);
        let b = Point::new(7, 9);
        let line = LineV2::new(a, b);
        let points = line.draw().unwrap();
        assert_eq!(
            points,
            vec![Point::new(9, 7), Point::new(8, 8), Point::new(7, 9)]
        )
    }
}
