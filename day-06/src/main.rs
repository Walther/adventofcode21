fn main() {
    const INPUT: &str = include_str!("input.txt");
    // Input file has a comma-separated list of numbers.
    // Each number represents one fish, showing the day number in its biological clock,
    // counting down to zero for spawning day.
    let numbers: Vec<usize> = INPUT.split(',').map(|x| x.parse().unwrap()).collect();

    // Fish biological clock counter:
    // - array of days 0 to 8 inclusive
    // - array[n] contains the count of fish at clock date n
    let mut fish_clock = [0u64; 9];
    for number in numbers {
        fish_clock[number] += 1;
    }

    let mut day = 0;

    loop {
        // Each day, every biological clock goes down a notch.
        // At the same time, each fish at zero clock spawns a new fish with clock set to 8
        // This can be modeled as a left rotation of the array.
        fish_clock.rotate_left(1);
        // Additionally, each fish that just spawned a new fish, have their clock set to 6
        // as they don't need to undergo an initial 2 day growth to reproductive maturity.
        // Thus we add the amount of fish at clock 8 to the sum of fish at clock 6.
        fish_clock[6] += fish_clock[8];

        // Increment our day counter
        day += 1;
        // If we have hit the target date, print solution
        if day == 80 {
            let total_fish: u64 = fish_clock.iter().sum();
            println!("Part 1: {}", total_fish);
        }
        if day == 256 {
            let total_fish: u64 = fish_clock.iter().sum();
            println!("Part 2: {}", total_fish);
            break;
        }
    }
}
