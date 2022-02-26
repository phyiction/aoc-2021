use std::collections::HashSet;
use std::fmt::Formatter;
use std::io::BufRead;
use std::{fmt, io};

#[derive(Hash, Eq, PartialEq)]
struct Point {
    row: usize,
    col: usize,
    height: u32,
}

impl Point {
    fn new(row: usize, col: usize, height: u32) -> Self {
        Self { row, col, height }
    }

    fn risk_level(&self) -> u32 {
        self.height + 1
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{({},{}),{}}}", self.row, self.col, self.height)
    }
}

struct HeightMap {
    rows: usize,
    cols: usize,
    points: Vec<Point>,
}

impl HeightMap {
    fn new(rows: Vec<String>) -> Self {
        let points = rows
            .iter()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| Point::new(row, col, ch.to_digit(10).unwrap()))
                    .collect()
            })
            .fold(Vec::new(), |mut acc, v: Vec<Point>| {
                acc.extend(v);
                acc
            });
        Self {
            cols: rows.get(0).unwrap().len(),
            rows: rows.len(),
            points,
        }
    }

    fn low_points(&self) -> Vec<&Point> {
        let mut result = Vec::new();
        for pt in self.points.iter() {
            let is_low_point = self
                .adjacent_points(pt)
                .iter()
                .map(|&x| x.height)
                .fold(true, |acc, height| acc && height > pt.height);
            if is_low_point {
                result.push(pt);
            }
        }
        result
    }

    fn adjacent_points(&self, p: &Point) -> Vec<&Point> {
        let up: Option<&Point> = if p.row > 0 {
            self.points.get((p.row - 1) * self.cols + p.col)
        } else {
            None
        };
        let right: Option<&Point> = if p.col + 1 < self.cols {
            self.points.get(p.row * self.cols + p.col + 1)
        } else {
            None
        };
        let down: Option<&Point> = if p.row + 1 < self.rows {
            self.points.get((p.row + 1) * self.cols + p.col)
        } else {
            None
        };
        let left: Option<&Point> = if p.col > 0 {
            self.points.get(p.row * self.cols + p.col - 1)
        } else {
            None
        };
        vec![up, right, down, left]
            .iter()
            .filter_map(|&x| x)
            .collect()
    }

    fn find_basin<'a>(&'a self, low_pt: &'a Point) -> HashSet<&'a Point> {
        let mut basin = HashSet::new();
        basin.insert(low_pt);
        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        queue.push(low_pt);
        while !queue.is_empty() {
            let curr_pt = queue.pop().unwrap();
            visited.insert(curr_pt);
            self.adjacent_points(curr_pt).iter().for_each(|&adj_pt| {
                if adj_pt.height > curr_pt.height && adj_pt.height != 9 && !visited.contains(adj_pt)
                {
                    basin.insert(adj_pt);
                    queue.push(adj_pt);
                }
            });
        }
        basin
    }

    fn find_all_basins(&self) -> Vec<HashSet<&Point>> {
        self.low_points()
            .iter()
            .map(|&p| self.find_basin(p))
            .collect()
    }
}

fn main() {
    let part: usize = std::env::args()
        .nth(1)
        .expect("specified puzzle part")
        .parse()
        .expect("valid number");

    let map = HeightMap::new(io::stdin().lock().lines().map(|l| l.unwrap()).collect());
    match part {
        1 => {
            let sum: u32 = map.low_points().iter().map(|&x| x.risk_level()).sum();
            println!("Sum or risk levels of low points is {}", sum);
        }
        2 => {
            let mut basins = map.find_all_basins();
            basins.sort_by(|a, b| a.len().cmp(&b.len()));
            let product = basins.iter().rev().take(3).fold(1, |acc, b| acc * b.len());
            println!("Product of 3 largest basins is {}", product);
        }
        _ => panic!("Problem only has two parts. Valid values are 1 and 2"),
    }
}

#[cfg(test)]
mod tests {
    use crate::HeightMap;

    #[test]
    fn test_adjacent_points() {
        let map = HeightMap::new(vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ]);
        let origin = map
            .points
            .iter()
            .find(|&pt| pt.row == 0 && pt.col == 0)
            .unwrap();
        let right = map
            .points
            .iter()
            .find(|&pt| pt.row == 0 && pt.col == 1)
            .unwrap();
        let down = map
            .points
            .iter()
            .find(|&pt| pt.row == 1 && pt.col == 0)
            .unwrap();
        assert_eq!(vec![right, down], map.adjacent_points(origin));
    }
}
