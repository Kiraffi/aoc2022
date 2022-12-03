const RUN_AMOUNT:u32 = 1;

fn main()
{
    let _test_data = include_str!("../test_data.txt");
    let _data = include_str!("../../data/day03.txt");

    let now = std::time::Instant::now();
    for _ in 0..RUN_AMOUNT
    {
        day03_1(true, &_data);
        day03_2(true, &_data);
    }
    println!("Day03 duration: {}us", now.elapsed().as_micros() as f32 / RUN_AMOUNT as f32);
}

fn get_priority(line: &str) -> u8
{
    let half = line.len() / 2;
    let line_bytes = line.as_bytes();
    let left_chars = &line_bytes[0..half];
    let right_chars = &line_bytes[half..];
    for left in left_chars
    {
        if right_chars.contains(left)
        {
            return *left;
        }
    }
    return 0;
}

fn get_priority_score(priority: u8) -> u32
{
    if priority >= 'a' as u8 && priority <= 'z' as u8
    {
        return (priority - 'a' as u8) as u32 + 1;
    }
    else
    {
        return (priority - 'A' as u8) as u32 + 27;
    }
}

fn day03_1(print_outcome: bool, content: &str)
{
    let mut total_score: u32 = 0;
    for line in content.lines()
    {
        let priority = get_priority(line);
        total_score += get_priority_score(priority);
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
    let mut lines = content.lines().peekable();

    while lines.peek() != None
    {
        let line0 = get_str(lines.next()).as_bytes();
        let line1 = get_str(lines.next()).as_bytes();
        let line2 = get_str(lines.next()).as_bytes();

        for priority0 in line0
        {
            if line1.contains(priority0) && line2.contains(priority0)
            {
                total_score += get_priority_score(*priority0);
                break;
            }
        }
    }
    if print_outcome
    {
        println!("Day 03-2: Sum of priorities: {}", total_score);
    }
}
