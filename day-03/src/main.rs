use bitvec::prelude::*;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let lines = INPUT.lines();

    // parse into a 2d vec
    let mut table: Vec<BitVec> = Vec::new();
    for line in lines {
        let mut row = bitvec![];

        for bit in line.chars() {
            let value: u32 = bit.to_string().parse().unwrap();
            // TODO: cleanup
            let value = match value {
                0 => false,
                1 => true,
                _ => panic!("Invalid bit deteced"),
            };
            row.push(value);
        }
        table.push(row);
    }

    let width = table[0].len();

    let mut gamma_bits = bitvec![];
    let mut epsilon_bits = bitvec![];

    for column in 0..width {
        gamma_bits.push(most_common_bit_in_column(&table, column));
        epsilon_bits.push(least_common_bit_in_column(&table, column));
    }
    // bit order shenanigans. // TODO: cleanup!
    gamma_bits.reverse();
    epsilon_bits.reverse();

    let gamma_rate = gamma_bits.into_vec()[0];
    let epsilon_rate = epsilon_bits.into_vec()[0];

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

    let mut oxygen_bits = bitvec![];
    for bit in &oxygen_rating[0] {
        oxygen_bits.push(*bit);
    }

    let mut co2_bits = bitvec![];
    for bit in &co2_rating[0] {
        co2_bits.push(*bit);
    }

    // bit order shenanigans. // TODO: cleanup!
    oxygen_bits.reverse();
    co2_bits.reverse();

    let oxygen_rating = oxygen_bits.into_vec()[0];
    let co2_rating = co2_bits.into_vec()[0];

    println!("Part 2: {}", oxygen_rating * co2_rating);
}

/// Given a 2D table of bits and a column index, returns the most common bit of that column. If there's an equal amount of 0s and 1s, 1 is returned.
fn most_common_bit_in_column(table: &[BitVec], column: usize) -> bool {
    let count: u32 = table.len() as u32;
    let mut sum = 0;
    for line in table {
        sum += line[column] as u32;
    }
    sum >= (count - sum)
}

/// Given a 2D table of bits and a column index, returns the least common bit of that column. If there's an equal amount of 0s and 1s, 0 is returned. This mirrors the behavior of [most_common_bit_in_column].
fn least_common_bit_in_column(table: &[BitVec], column: usize) -> bool {
    let most_common = most_common_bit_in_column(table, column);
    !most_common
}
