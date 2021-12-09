use anyhow::Result;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    const INPUT: &str = include_str!("input.txt");
    // Input is a 2D map of digits, newline-separated
    let mut map = Map::default();
    for (y, line) in INPUT.lines().enumerate() {
        for (x, value) in line.chars().enumerate() {
            let number = value.to_string().parse()?;
            map.0.insert((x as i32, y as i32), number);
        }
    }
    // Part 1: calculate the sum of all risk values on the map
    let risk_sum: i32 = map.risk_values().iter().sum();
    println!("Part 1: {}", risk_sum);

    // Part 2: calculate the product of the sizes of the three largest basins
    let mut basin_sizes = map.basin_sizes();
    basin_sizes.sort_unstable();
    let basin_size_product: u32 = basin_sizes.iter().rev().take(3).product();
    println!("Part 2: {}", basin_size_product);

    Ok(())
}

#[derive(Default, Debug)]
struct Map(HashMap<(i32, i32), i32>);

impl Map {
    /// Given a coordinate, returns the directly adjacent horizontal and vertical coordinates, up to 4.
    fn get_neighbors(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = Vec::new();
        for dx in [-1, 1] {
            for dy in [-1, 1] {
                if self.0.get(&(x + dx, y)).is_some() {
                    points.push((x + dx, y))
                }
                if self.0.get(&(x, y + dy)).is_some() {
                    points.push((x, y + dy))
                }
            }
        }

        points
    }

    /// Returns the coordinates of all the low points of the map.
    fn low_points(&self) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = Vec::new();
        for (x, y) in self.0.keys() {
            let value = self.0.get(&(*x, *y)).unwrap();
            let neighbors = self.get_neighbors(*x, *y);
            if neighbors.iter().all(|&(nx, ny)| {
                let neighbor_value = self.0.get(&(nx, ny)).unwrap();
                neighbor_value > value
            }) {
                points.push((*x, *y));
            }
        }
        points
    }

    /// Calculates all the risk values of the map.
    fn risk_values(&self) -> Vec<i32> {
        let low_points = self.low_points();
        low_points
            .iter()
            .map(|(x, y)| self.0.get(&(*x, *y)).unwrap() + 1)
            .collect()
    }

    /// Calculates the basin sizes on the map.
    fn basin_sizes(&self) -> Vec<u32> {
        let mut basin_sizes: Vec<u32> = Vec::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        // Every basin starts at a low point, and fills up to the surrounding tiles with height 9
        let low_points = self.low_points();
        for (x, y) in low_points {
            visited.insert((x, y));
            // Start with one unit for the point we are in right now
            let mut basin_size: u32 = 1;
            // Flood fill until hitting tiles with height 9
            basin_size += self.flood(x, y, &mut visited);
            basin_sizes.push(basin_size);
        }
        basin_sizes
    }

    /// Helper function. Floods a single low point, returning the number of tiles that can be filled.
    /// Boundary condition: tiles with height 9
    fn flood(&self, x: i32, y: i32, visited: &mut HashSet<(i32, i32)>) -> u32 {
        self.get_neighbors(x, y)
            .iter()
            .map(|(x, y)| {
                // If we have already visited this square, return 0
                if visited.get(&(*x, *y)).is_some() {
                    return 0;
                } else {
                    visited.insert((*x, *y));
                }
                // Check the value: if we found a 9, stop. Otherwise, recurse
                let value = self.0.get(&(*x, *y)).unwrap();
                match value {
                    9 => 0,
                    _ => 1 + self.flood(*x, *y, visited),
                }
            })
            .sum()
    }
}
