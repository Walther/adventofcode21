use itertools::Itertools;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    // The input has a newline-separated list of depths from a sonar scan of the seabed
    let lines = INPUT.lines();
    let depths: Vec<u32> = lines.map(|x| x.parse().unwrap()).collect();

    // We need to calculate how many times the depth value has increased; value is deeper than the previous value
    let mut num_increases = 0;

    // Part 1: sliding window of two, how many pairs are larger than the previous one?
    for (a, b) in depths.iter().tuple_windows() {
        if b > a {
            num_increases += 1;
        }
    }
    println!("Part 1: {}", num_increases);

    // Part 2: sliding window of three, how many three-sums are larger than the previous one?
    // This can be calculated with a sliding window of four
    let mut num_increases = 0;
    for (a, b, c, d) in depths.iter().tuple_windows() {
        if (b + c + d) > (a + b + c) {
            num_increases += 1;
        }
    }

    println!("Part 2: {}", num_increases)
}
