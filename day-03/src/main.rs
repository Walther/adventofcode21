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

    let width = table[0].len();

    let mut gamma_bits: String = String::new();
    let mut epsilon_bits: String = String::new();

    for column in 0..width {
        gamma_bits.push_str(&most_common_bit_in_column(&table, column).to_string());
        epsilon_bits.push_str(&least_common_bit_in_column(&table, column).to_string());
    }

    let gamma_rate = u32::from_str_radix(&gamma_bits, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_bits, 2).unwrap();
    let power_consumption = gamma_rate * epsilon_rate;
    println!("Part 1: {}", power_consumption);

    // iteratively filter through the tables to reduce the numbers into one rating
    let mut oxygen_rating = table.clone();
    let mut co2_rating = table.clone();

    for column in 0..oxygen_rating[0].len() {
        let most_common = most_common_bit_in_column(&oxygen_rating, column);

        oxygen_rating = oxygen_rating
            .iter()
            .filter(|&x| x[column] == most_common)
            .cloned()
            .collect();

        if oxygen_rating.len() == 1 {
            break;
        }
    }

    for column in 0..co2_rating[0].len() {
        let least_common = least_common_bit_in_column(&co2_rating, column);

        co2_rating = co2_rating
            .iter()
            .filter(|&x| x[column] == least_common)
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

/// Given a 2D table of bits and a column index, returns the most common bit of that column. If there's an equal amount of 0s and 1s, 1 is returned.
fn most_common_bit_in_column(table: &[Vec<u32>], column: usize) -> u32 {
    let count: u32 = table.len() as u32;
    let mut sum = 0;
    for line in table {
        sum += line[column];
    }
    if sum >= (count - sum) {
        1
    } else {
        0
    }
}

/// Given a 2D table of bits and a column index, returns the least common bit of that column. If there's an equal amount of 0s and 1s, 0 is returned. This mirrors the behavior of [most_common_bit_in_column].
fn least_common_bit_in_column(table: &[Vec<u32>], column: usize) -> u32 {
    let most_common = most_common_bit_in_column(table, column);
    if most_common == 1 {
        0
    } else {
        1
    }
}
