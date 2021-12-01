use itertools::Itertools;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let lines = INPUT.lines();
    let numbers: Vec<u32> = lines.map(|x| x.parse().unwrap()).collect();
    let mut num_increases = 0;

    // Part 1: sliding window of two, how many pairs have an increasing order?
    for (a, b) in numbers.iter().tuple_windows() {
        if b > a {
            num_increases += 1;
        }
    }
    println!("Part 1: {}", num_increases);

    // Part 2: sliding window of three, how many three-sums are in increasing order?
    // Collect sums
    let mut sums: Vec<u32> = Vec::new();
    for (a, b, c) in numbers.iter().tuple_windows() {
        sums.push(a + b + c);
    }
    // Check increases
    let mut num_increases = 0;
    for (a, b) in sums.iter().tuple_windows() {
        if b > a {
            num_increases += 1;
        }
    }

    println!("Part 2: {}", num_increases)
}
