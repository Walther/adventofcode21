fn main() {
    const INPUT: &str = include_str!("input.txt");
    let lines = INPUT.lines();

    // parse into a 2d vec
    let mut table: Vec<Vec<u32>> = Vec::new(); // TODO: binary, not u32?
    for line in lines {
        let mut row = vec![];

        for bit in line.chars() {
            let value: u32 = bit.to_string().parse().unwrap(); // TODO: better parse
            row.push(value);
        }
        table.push(row);
    }

    // count column sums
    let mut column_sums: Vec<u32> = Vec::new();
    for column in 0..table[0].len() {
        let mut sum = 0;
        for line in &table {
            sum += line[column];
        }
        column_sums.push(sum);
    }

    // if the sum is larger than half the length of the list, one was more common
    // if the sum is smaller, zero was more common in that column

    let half: u32 = table.len() as u32 / 2;

    let gamma_bits: String = column_sums
        .iter()
        .map(|&x| if x > half { '1' } else { '0' })
        .collect();

    // epsilon uses least common.
    // TODO: optimize; this would be just a bitflip of the other
    let epsilon_bits: String = column_sums
        .iter()
        .map(|&x| if x < half { '1' } else { '0' })
        .collect();

    let gamma_rate = u32::from_str_radix(&gamma_bits, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_bits, 2).unwrap();
    let power_consumption = gamma_rate * epsilon_rate;
    println!("Part 1: {}", power_consumption);

    // Part 2 shenanigans, very ugly for now!

    // iteratively filter through the tables to reduce the numbers into one rating
    let mut oxygen_rating = table.clone();
    let mut co2_rating = table.clone();

    for column in 0..oxygen_rating[0].len() {
        let count: u32 = oxygen_rating.len() as u32;
        let mut sum = 0;
        for line in &oxygen_rating {
            sum += line[column];
        }
        let most_common = if sum >= (count - sum) { "1" } else { "0" };

        oxygen_rating = oxygen_rating
            .iter()
            .filter(|&x| *x.get(column).unwrap().to_string() == *most_common)
            .cloned()
            .collect();

        if oxygen_rating.len() == 1 {
            break;
        }
    }

    for column in 0..co2_rating[0].len() {
        let count: u32 = co2_rating.len() as u32;
        let mut sum = 0;
        for line in &co2_rating {
            sum += line[column];
        }
        let least_common = if sum >= (count - sum) { "0" } else { "1" };

        co2_rating = co2_rating
            .iter()
            .filter(|&x| *x.get(column).unwrap().to_string() == *least_common)
            .cloned()
            .collect();

        if co2_rating.len() == 1 {
            break;
        }
    }

    let oxygen_bits: String = oxygen_rating[0].iter().map(|x| x.to_string()).collect();
    let co2_bits: String = co2_rating[0].iter().map(|x| x.to_string()).collect();

    let oxygen_rating = u32::from_str_radix(&oxygen_bits, 2).unwrap();
    let co2_rating = u32::from_str_radix(&co2_bits, 2).unwrap();

    println!("Part 2: {}", oxygen_rating * co2_rating);
}
