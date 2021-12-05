use anyhow::{Context, Error, Result};
use std::{collections::HashMap, str::FromStr};

fn main() -> Result<()> {
    const INPUT: &str = include_str!("input.txt");
    let rows = INPUT.lines();

    // Parse and collect all Line objects
    let lines: Vec<Line> = rows.map(|row| row.parse()).collect::<Result<_, _>>()?;

    // Maps of visit counts: (x,y), count
    let mut map_axis_aligned: HashMap<(i32, i32), i32> = HashMap::new();
    let mut map_all: HashMap<(i32, i32), i32> = HashMap::new();

    // Lines: axis-aligned only
    for line in lines.iter().filter(|line| line.is_axis_aligned()) {
        for (x, y) in line.get_points() {
            *map_axis_aligned.entry((x, y)).or_insert(0) += 1;
        }
    }

    // Lines: all
    for line in lines.iter() {
        for (x, y) in line.get_points() {
            *map_all.entry((x, y)).or_insert(0) += 1;
        }
    }

    let repeat_visits_aa = map_axis_aligned
        .values()
        .filter(|&visits| *visits > 1)
        .count();
    println!("Part 1: {}", repeat_visits_aa);

    let repeat_visits_all = map_all.values().filter(|&visits| *visits > 1).count();
    println!("Part 2: {}", repeat_visits_all);

    // debug print
    // print_map(map_all);
    Ok(())
}

struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    /// Create a new [Line] from two tuples: start and end coordinates
    fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        Self { start, end }
    }

    /// Destructures a given [Line] into two tuples: start and end coordinates
    fn coordinates(&self) -> ((i32, i32), (i32, i32)) {
        let ((x1, y1), (x2, y2)) = (self.start, self.end);
        ((x1, y1), (x2, y2))
    }

    /// Returns whether the [Line] is axis aligned; that is, a straight line on either the X or Y axis
    fn is_axis_aligned(&self) -> bool {
        let ((x1, y1), (x2, y2)) = self.coordinates();
        x1 == x2 || y1 == y2
    }

    /// Given the start coordinate x1,y1 and end coordinate x2,y2, return the points on the lines.
    /// Note: this is only guaranteed to work on lines that are either axis-aligned or at an exact 45deg angle.
    fn get_points(&self) -> Vec<(i32, i32)> {
        let ((x1, y1), (x2, y2)) = self.coordinates();
        let mut points = Vec::new();

        // becomes 0 if equal, -1 if less, 1 if greater
        let xdir = x2.cmp(&x1) as i32;
        let ydir = y2.cmp(&y1) as i32;

        let mut x = x1;
        let mut y = y1;
        while x != x2 || y != y2 {
            points.push((x, y));
            x += xdir;
            y += ydir;
        }
        // End point inclusive
        points.push((x, y));
        points
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once(" -> ")
            .context("Failed to split line to start and end coordinates")?;
        let (x1, y1) = start
            .split_once(",")
            .context("Failed to split start coordinate to x and y")?;
        let (x2, y2) = end
            .split_once(",")
            .context("Failed to split end coordinate to x and y")?;
        let x1: i32 = x1.parse().context("Invalid x1 coordinate")?;
        let y1: i32 = y1.parse().context("Invalid y1 coordinate")?;
        let x2: i32 = x2.parse().context("Invalid x2 coordinate")?;
        let y2: i32 = y2.parse().context("Invalid y2 coordinate")?;
        let (start, end) = ((x1, y1), (x2, y2));

        Ok(Line::new(start, end))
    }
}

#[allow(dead_code)]
/// Debug print helper
fn print_map(map: HashMap<(i32, i32), i32>) {
    let (xmax, _) = map
        .keys()
        .max_by(|(x1, _y1), (x2, _y2)| x1.cmp(x2))
        .unwrap();
    let (_, ymax) = map
        .keys()
        .max_by(|(_x1, y1), (_x2, y2)| y1.cmp(y2))
        .unwrap();
    for y in 0..=*ymax {
        for x in 0..=*xmax {
            let tile = map.get(&(x, y)).unwrap_or(&0);
            print!("{}", tile)
        }
        println!();
    }
}
