fn main() {
    const INPUT: &str = include_str!("input.txt");
    let lines = INPUT.lines();

    // We have two travel modes for the same instructions:
    // - simple, literal travel based on the instruction direction and offset
    // - aim, where up and down instructions change the tilt of the submarine, and only forward instruction is used for travel
    // We go through the instruction list once and track our travel based on both methods at the same time.
    let mut simple_coords: Vec<(u32, u32)> = Vec::new();
    let mut aim_coords: Vec<(u32, u32)> = Vec::new();
    let mut simple_position = (0, 0);
    let mut aim_position = (0, 0);
    let mut aim = 0;
    simple_coords.push(simple_position);
    aim_coords.push(simple_position);

    for line in lines {
        let instruction: Vec<&str> = line.split(' ').collect();
        let direction = instruction.get(0).unwrap();
        let offset: u32 = instruction[1].parse().unwrap();

        // NOTE: Submarine coordinates: origin is (0,0), y down is positive
        match *direction {
            "forward" => {
                simple_position.0 += offset;
                aim_position.0 += offset;
                aim_position.1 += aim * offset;
            }
            "down" => {
                simple_position.1 += offset;
                aim += offset;
            }
            "up" => {
                simple_position.1 -= offset;
                aim -= offset;
            }
            _ => {
                panic!("Unsupported instruction: {:?}", instruction);
            }
        }
        simple_coords.push(simple_position);
        aim_coords.push(aim_position);
    }

    // final x and y coordinates multiplied
    let simple_final = simple_coords.last().unwrap();
    println!("Part 1: {}", simple_final.0 * simple_final.1);
    let aim_final = aim_coords.last().unwrap();
    println!("Part 2: {}", aim_final.0 * aim_final.1);
}
