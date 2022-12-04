
const RUN_AMOUNT:u32 = 1;

fn main()
{
    let _test_data = include_str!("../test_data.txt");
    let _data = include_str!("../../data/day04.txt");

    let now = std::time::Instant::now();
    /*
    for _ in 0..RUN_AMOUNT - 1
    {
        day04_1(false, &_data);
        day04_2(false, &_data);
    }
    */
    day04_1(true, &_data);
    day04_2(true, &_data);
    println!("Day03 duration: {}us", now.elapsed().as_micros() as f32 / RUN_AMOUNT as f32);
}
fn get_rooms(line: &str) -> u128
{
    let mut rooms = line.split("-");
    let first = rooms.next().unwrap().parse::<i32>().unwrap();
    let second = rooms.next().unwrap().parse::<i32>().unwrap();
    // Set bits up to second value.
    let result = (1u128 << second) - 1;
    // Get first value bits and invert the bits. The first value needs to be reduced by 1,
    // so for example 1 becomes 0th bit, then -1 will make it 0. Inverting 0 is ~0, all bits set.
    let first_invert_bits = !((1u128 << (first - 1)) - 1);
    // And them together to get the range from first to second bits
    let result = result & first_invert_bits;
    return result;
}

fn get_rooms_values(line: &str) -> (u128, u128)
{
    let mut pairs = line.split(",");
    let first_pair = pairs.next().unwrap();
    let second_pair = pairs.next().unwrap();
    let first_rooms = get_rooms(first_pair);
    let second_rooms = get_rooms(second_pair);
    return (first_rooms, second_rooms);
}

fn day04_1(print_outcome: bool, content: &str)
{
    let mut total_score: u32 = 0;
    for line in content.lines()
    {
        let (first, second) = get_rooms_values(line);
        let combined_room = first & second;
        if ((first & combined_room) == first)
            || ((second & combined_room) == second)

        {
            total_score += 1;
        }
    }
    if print_outcome
    {
        println!("Day 04-1: Fully overlapping room assignments: {}", total_score);
    }
}

fn day04_2(print_outcome: bool, content: &str)
{
    let mut total_score: u32 = 0;
    for line in content.lines()
    {
        let (first, second) = get_rooms_values(line);
        let combined_room = first & second;
        if combined_room != 0

        {
            total_score += 1;
        }
    }
    if print_outcome
    {
        println!("Day 04-2: Any overlapping room assignments: {}", total_score);
    }
}
