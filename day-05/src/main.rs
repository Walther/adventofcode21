use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let lines = INPUT.lines();

    // Maps of visit counts: (x,y), count
    let mut map_straight: HashMap<(i32, i32), i32> = HashMap::new();
    let mut map_diagonal: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        // x1,y1 -> x2,y2
        // TODO: cleanup
        let (a, b) = line.split_once(" -> ").unwrap();
        let (x1, y1) = a.split_once(",").unwrap();
        let (x2, y2) = b.split_once(",").unwrap();
        let x1: i32 = x1.parse().unwrap();
        let y1: i32 = y1.parse().unwrap();
        let x2: i32 = x2.parse().unwrap();
        let y2: i32 = y2.parse().unwrap();

        // First consider only the lines that are along the axes
        if let Some(points_straight) = line_get_points(x1, y1, x2, y2) {
            for (x, y) in points_straight {
                if let Some(point) = map_straight.get_mut(&(x, y)) {
                    *point += 1;
                } else {
                    map_straight.insert((x, y), 1);
                }
            }
        } else {
            // Otherwise, consider the diagonal
            let points_diagonal = diag_get_points(x1, y1, x2, y2);
            for (x, y) in points_diagonal {
                if let Some(point) = map_diagonal.get_mut(&(x, y)) {
                    *point += 1;
                } else {
                    map_diagonal.insert((x, y), 1);
                }
            }
        }
    }

    let repeat_visits_straight = map_straight
        .iter()
        .filter(|(&(_x, _y), &visits)| visits > 1)
        .count();
    println!("Part 1: {}", repeat_visits_straight);

    // Combine the straight lines into the diagonal map
    for (&(x, y), visits_s) in map_straight.iter_mut() {
        if let Some(visits_d) = map_diagonal.get_mut(&(x, y)) {
            *visits_d += *visits_s;
        } else {
            map_diagonal.insert((x, y), *visits_s);
        }
    }

    let repeat_visits_diagonal = map_diagonal
        .iter()
        .filter(|(&(_x, _y), &visits)| visits > 1)
        .count();
    println!("Part 2: {}", repeat_visits_diagonal);

    // debug print
    // print_map(map_diagonal);
}

/// Given the start coordinate x1,y1 and end coordinate x2,y2, return all coordinates on the axis-aligned straight line between those points.
/// If the given line would not be axis-aligned, an empty list is returned.
fn line_get_points(x1: i32, y1: i32, x2: i32, y2: i32) -> Option<Vec<(i32, i32)>> {
    let mut coords = Vec::new();

    // Make sure we have a straight line
    if !(x1 == x2 || y1 == y2) {
        return None;
    }

    let xmin = x1.min(x2);
    let xmax = x1.max(x2);
    let ymin = y1.min(y2);
    let ymax = y1.max(y2);

    for x in xmin..=xmax {
        for y in ymin..=ymax {
            coords.push((x, y));
        }
    }

    Some(coords)
}

/// Given the start coordinate x1,y1 and end coordinate x2,y2, return the points on the 45deg diagonal line.
/// Note: this function is not guaranteed to work, if the given coordinates do not produce a 45deg diagonal line.
fn diag_get_points(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    let mut coords = Vec::new();

    let xdir = if x2 > x1 { 1 } else { -1 };
    let ydir = if y2 > y1 { 1 } else { -1 };

    let mut x = x1;
    let mut y = y1;
    while x != x2 && y != y2 {
        coords.push((x, y));
        x += xdir;
        y += ydir;
    }
    // push the end point too
    coords.push((x, y));
    coords
}

#[allow(dead_code)]
/// Debug print helper
fn print_map(map: HashMap<(i32, i32), i32>) {
    let ((xmax, _), _) = map
        .iter()
        .max_by(|((x1, _y1), _value1), ((x2, _y2), _value2)| x1.cmp(x2))
        .unwrap();
    let ((_, ymax), _) = map
        .iter()
        .max_by(|((_x1, y1), _value1), ((_x2, y2), _value2)| y1.cmp(y2))
        .unwrap();
    for y in 0..=*ymax {
        for x in 0..=*xmax {
            let tile = map.get(&(x, y)).unwrap_or(&0);
            print!("{}", tile)
        }
        println!();
    }
}
