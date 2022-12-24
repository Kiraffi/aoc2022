
const RUN_AMOUNT:u32 = 1;
const DAY_STR: &'static str = "24";
const DATA: &'static str = include_str!("../../data/day24.txt");
const _TEST_DATA: &'static str = include_str!("../test_data.txt");

fn main()
{
    let now = std::time::Instant::now();
    /*
    for _ in 0..RUN_AMOUNT - 1
    {
        _ = part_a(&_data);
        _ = part_b(&_data);
    }
    */
    println!("Day {}-1: Through the blizzard time: {}", DAY_STR, part_a(&DATA));
    println!("Day {}-2: Through the blizzard time and back and through again: {}", DAY_STR, part_b(&DATA));
    println!("Day {} duration: {}us", DAY_STR, now.elapsed().as_micros() as f32 / RUN_AMOUNT as f32);
}

pub struct Blizzard
{
    up: Vec<u128>,
    down: Vec<u128>,
    left: Vec<u128>,
    right: Vec<u128>
}

fn _print_map(map: &Vec<u128>)
{
    for l in map
    {
        for i in 0..128
        {
            let c = if ((l >> i) & 1) == 1 {'#'} else {'.'};
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Stamp
{
    x: i8,
    y: i8
}

pub fn simulate_step(
    blizzards: &mut Blizzard,
    map: &Vec<u128>,
    map_size: (i8, i8),
    _time: u16) -> Vec<u128>
{
    let rows = (map_size.1 - 1) as usize;
    let cols = (map_size.0 - 1) as usize;

    for i in 0..rows
    {
        let l = &mut blizzards.right[i];
        *l <<= 1;
        *l &= !3;
        *l |= (*l >> (map_size.0 - 2)) & 2;
        *l &= 0x7fff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;

        let r = &mut blizzards.left[i];
        *r >>= 1;
        *r |= (*r & 1) << (cols - 1);

        blizzards.up[i] = blizzards.up[i + 1];

        blizzards.down[rows - i] = blizzards.down[rows - i - 1];
    }

    blizzards.down[1] = blizzards.down[rows];
    blizzards.up[rows - 1] = blizzards.up[0];

    let mut new_map: Vec<u128> = map.clone();

    for i in 0..map.len()
    {
        new_map[i] |= blizzards.left[i];
        new_map[i] |= blizzards.right[i];
        new_map[i] |= blizzards.up[i];
        new_map[i] |= blizzards.down[i];
    }
    return new_map;
}

pub fn simulate(
    start: (i8, i8),
    end: (i8, i8),
    map_size: (i8, i8),
    start_time: u16,
    blizzards: &mut Blizzard,
    map: &Vec<u128>) -> u16
{
    let mut posses: Vec<u128> = Vec::new();
    posses.resize(map_size.1 as usize, 0);
    posses[start.1 as usize] |= 1 << start.0;

    let mut time = start_time;

    while ((posses[end.1 as usize] >> end.0) & 1) == 0
    {
        let new_map = simulate_step(blizzards, map, map_size, time + 1);
        let mut new_posses = posses.clone();

        for i in 0..map.len() - 1
        {
            new_posses[i] |= posses[i + 1];
            new_posses[i + 1] |= posses[i];
            new_posses[i] |= posses[i] << 1;
            new_posses[i] |= posses[i] >> 1;
        }
        for i in 0..map.len() - 1
        {
            new_posses[i] &= !new_map[i];
        }
        //new_map[pos.1 as usize][pos.0 as usize] = '@';
        //println!("Time: {}", time);
        //_print_map(&new_map);
        //let mut new_map_tmp = new_map.clone();
        posses = new_posses;
        time += 1;
    }
    return time;
}

fn parse(content: &'static str) -> (Blizzard, (i8, i8), (i8, i8), (i8, i8), Vec<u128>)
{
    let mut blizzards: Blizzard = Blizzard{ up: Vec::new(), down: Vec::new(), left: Vec::new(), right: Vec::new() };
    let lines = content.lines().collect::<Vec<&str>>();
    let start = lines[0].find('.').unwrap();
    let end = lines[lines.len() - 1].find('.').unwrap();
    let mut map: Vec<u128> = Vec::new();

    for row in 0..lines.len() as i8
    {
        let mut line = 0u128;
        let mut right = 0u128;
        let mut left = 0u128;
        let mut up = 0u128;
        let mut down = 0u128;
        for (col, &c) in lines[row as usize].as_bytes().iter().enumerate()
        {
            match c as char
            {
                '>' => right |= 1 << col,
                'v' => down |= 1 << col,
                '<' => left |= 1 << col,
                '^' => up |= 1 << col,
                '#' => { line |= 1 << col },
                _ => {}
            }
        }
        blizzards.up.push(up);
        blizzards.down.push(down);
        blizzards.left.push(left);
        blizzards.right.push(right);
        map.push(line);
    }
    let map_size = (lines[0].len() as i8, lines.len() as i8);
    let start = (start as i8, 0i8);
    let end = (end as i8, lines.len() as i8 - 1);
    return (blizzards, start, end, map_size, map);
}

#[test]
fn part_a_test()
{
    let value = part_a(&_TEST_DATA);
    assert_eq!(value, 18);
}

fn part_a(content: &'static str) -> i64
{
    let (mut blizzards, start, end, map_size, map) = parse(content);

    let fastest = simulate(
        start,
        end,
        map_size,
        0,
        &mut blizzards,
        &map);
    return fastest as i64;
}

#[test]
fn part_b_test()
{
    let value = part_b(&_TEST_DATA);
    assert_eq!(value, 54);
}

fn part_b(content: &'static str) -> i64
{
    let (mut blizzards, start, end, map_size, map) = parse(content);

    let fastest = simulate(start, end, map_size, 0, &mut blizzards, &map);
    let fastest = simulate(end, start, map_size, fastest, &mut blizzards, &map);
    let fastest = simulate(start, end, map_size, fastest, &mut blizzards, &map);
    return fastest as i64;
}



