const RUN_AMOUNT:u32 = 1;
const DAY_STR: &'static str = "06";

fn main()
{
    let _test_data = include_str!("../test_data.txt");
    let _data = include_str!("../../data/day06.txt");

    let now = std::time::Instant::now();
    /*
    for _ in 0..RUN_AMOUNT - 1
    {
        part_a(false, &_data);
        part_b(false, &_data);
    }
    */
    part_a(true, &_data);
    part_b(true, &_data);
    println!("Day {} duration: {}us", DAY_STR, now.elapsed().as_micros() as f32 / RUN_AMOUNT as f32);
}

fn find_amount_unique(line: &str, amount: usize) -> usize
{
    let bytes = line.as_bytes();
    let mut char_count_per_char: [u8; 32] = [0u8; 32];
    let mut unique_char_count = 0usize;
    for n in 0..bytes.len()
    {

        let new_value = (bytes[n] - 'a' as u8) as usize;
        unique_char_count += if char_count_per_char[new_value] == 0 { 1 } else { 0 };
        char_count_per_char[new_value] += 1;
        if n >= amount
        {
            let old_value = (bytes[n - amount] - 'a' as u8) as usize;
            char_count_per_char[old_value] -= 1;
            unique_char_count -= if char_count_per_char[old_value] == 0 { 1 } else { 0 };
        }
        if unique_char_count == amount
        {
            return n + 1;
        }
    }
    return 0usize;
}

fn part_a(print_outcome: bool, content: &str)
{
    let chars = find_amount_unique(content, 4);
    if print_outcome
    {
        println!("Day {}-1: First occuring 4 diff chars: {}", DAY_STR, chars);
    }
}


fn part_b(print_outcome: bool, content: &str)
{
    let chars = find_amount_unique(content, 14);
    if print_outcome
    {
        println!("Day {}-2: First occuring 14 diff chars: {}", DAY_STR, chars);
    }
}
