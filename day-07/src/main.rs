fn main() {
    const INPUT: &str = include_str!("input.txt");
    // The input is a comma-separated list of horizontal positions of tiny crabs in tiny submarines 🦀
    let positions: Vec<i32> = INPUT.split(',').map(|x| x.parse().unwrap()).collect();

    // We need to calculate the optimal position for getting all the crabs aligned given certain fuel cost functions,
    // and return the minimum total fuel cost for getting the crabs into that alignment.

    // Part 1: one unit movement costs one unit of fuel

    // Calculate the median of the positions
    let mut sorted = positions.clone();
    sorted.sort_unstable();
    let length = sorted.len();
    let median = sorted[length / 2];

    // Calculate the total fuel cost of moving all crabs to the median position
    let fuel_cost: u32 = positions.iter().map(|&crab| distance(crab, median)).sum();
    println!("Part 1: {}", fuel_cost);

    // Part 2: first unit movement costs one unit of fuel, second movement costs two, and so on

    // For all possible target positions, calculate a weighted fuel cost,
    // where the unit fuel cost increases by one after each position moved
    let min_pos = sorted[0];
    let max_pos = sorted[length - 1];
    let weighted_fuel_sums = (min_pos..max_pos).map(|target| {
        positions
            .iter()
            .map(|&crab| {
                let distance = distance(crab, target);
                // Total fuel cost can be represented as the sum of arithmetic series 1..distance
                (distance * (1 + distance)) / 2
            })
            .sum()
    });
    // Pick the minimum fuel cost
    let fuel_cost: u32 = weighted_fuel_sums.min().unwrap();
    println!("Part 2: {}", fuel_cost);
}

/// Given two signed coordinates, return the unsigned distance between them
fn distance(a: i32, b: i32) -> u32 {
    (a - b).abs() as u32
}
