

const RUN_AMOUNT:u32 = 1;
const DAY_STR: &'static str = "10";

fn main()
{
    let _test_data = include_str!("../test_data.txt");
    let _data = include_str!("../../data/day10.txt");

    let now = std::time::Instant::now();
    /*    let mut top_visible = true;
    let mut bot_visible = true;

    for _ in 0..RUN_AMOUNT - 1
    {
        part_a(false, &_data);
        part_b(false, &_data);
    }
    */
    part_a(true, &_data);
    //part_b(true, &_data);
    println!("Day {} duration: {}us", DAY_STR, now.elapsed().as_micros() as f32 / RUN_AMOUNT as f32);
}

fn part_a(print_outcome: bool, content: &'static str)
{
    let mut clock = 1;
    let mut x: i64 = 1;
    let mut sum_checkpoints = 0;
    let mut crt = vec![vec!['.' as u8; 40]; 6];
    for line in content.lines()
    {
        let mut words = line.split(' ');
        let cmd = words.next().unwrap();
        let value = words.next().unwrap_or_default().parse::<i64>().unwrap_or_default();
        let mut add_clock = 0;
        match cmd
        {
            "noop" => add_clock = 1,
            "addx" => add_clock = 2,
            _ => ()
        }

        for i in 1..=add_clock
        {
            if i64::abs(x - ((clock - 1) % 40)) <= 1
            {
                crt[((clock - 1) / 40) as usize][((clock - 1) % 40) as usize] = '#' as u8;
            }
            clock += 1;
            if i == add_clock
            {
                x += value;
            }
            if (clock == 20) || (clock >= 60 && (clock - 20) % 40 == 0)
            {
                sum_checkpoints += clock * x;
            }
        }
    }
    if print_outcome
    {
        println!("Day {}-1: Sum of signal points: {}", DAY_STR, sum_checkpoints);
        println!("Day {}-2: Image", DAY_STR);

        for s in crt
        {
            println!("{}", std::str::from_utf8(s.as_slice()).unwrap());
        }
    }
}

