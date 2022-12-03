
const RUN_AMOUNT:u32 = 1;

fn main()
{
    let _test_data = include_str!("../test_data.txt");
    let _data = include_str!("../../data/day03.txt");

    let now = std::time::Instant::now();
    for _ in 0..RUN_AMOUNT - 1
    {
        day03_1(false, &_data);
        day03_2(false, &_data);
    }
    day03_1(true, &_data);
    day03_2(true, &_data);
    println!("Day03 duration: {}us", now.elapsed().as_micros() as f32 / RUN_AMOUNT as f32);
}

fn calculate_str_value(line: &[u8]) -> u64
{
    let mut value: u64 = 0;
    for n in 0..line.len()
    {
        // bitwise or for the priority score
        value |= 1u64 << get_priority_score(line[n]);
    }
    return value;
}

fn get_priority_score(priority: u8) -> u8
{
    let value = (priority - 'A' as u8) as u8 + 1u8;
    let value_bit = value & 32u8;          // Get the bit from lower case letters ('a' = 33, 'A' = 1).
    let value_bit = value_bit >> 5u8;      // Bit shift it to smallest value.
    let value_bit = value_bit ^ 1u8;       // Swap the bit.
    let value = value & 31u8;              // Get the lower 5 bits ('A' = 1, 'a' = 1, was 33).
    let value = value + value_bit * 26u8;  // Multiply the bit by 26 and add it to value
    return value;
}

fn day03_1(print_outcome: bool, content: &str)
{
    let mut total_score: u32 = 0;
    for line in content.lines()
    {
        let half = line.len() / 2;
        let line_bytes = line.as_bytes();
        let left_chars = &line_bytes[0..half];
        let right_chars = &line_bytes[half..];

        let value: u64 = calculate_str_value(left_chars) & calculate_str_value(right_chars);
        total_score += value.trailing_zeros();
    }
    if print_outcome
    {
        println!("Day 03-1: Sum of priorities: {}", total_score);
    }
}

fn get_str(option: Option<&str>) -> &str
{
    return match option
    {
        Some(value) => value,
        None => ""
    }
}

fn day03_2(print_outcome: bool, content: &str)
{
    let mut total_score: u32 = 0;
    let mut lines = content.lines(); //.peekable();
    // while lines.peek() != None
    while let Some(l0) = lines.next()
    {
        let value: u64 = calculate_str_value(l0.as_bytes());

        let l1 = get_str(lines.next());
        let value = value & calculate_str_value(l1.as_bytes());

        let l2 = get_str(lines.next());
        let value = value & calculate_str_value(l2.as_bytes());

//        total_score = total_score.wrapping_add(value as u32);
        total_score += value.trailing_zeros();
    }
    if print_outcome
    {
        println!("Day 03-2: Sum of priorities: {}", total_score);
    }
}
