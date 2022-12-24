use std::collections::{VecDeque, HashSet};


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

struct Blizzard
{
    x: u8,
    y: u8,
    dir: u8,
    _padding: u8
}

fn _print_map(map: &Vec<Vec<char>>)
{
    for l in map
    {
        for c in l
        {
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Stamp
{
    time: u16,
    x: u8,
    y: u8
}

fn simulate(start: (u8, u8), start_time: u16, blizzards: &Vec<Blizzard>, end: (u8, u8), map: &Vec<Vec<char>>) -> u16
{
    let mut posses: VecDeque<Stamp> = VecDeque::new();
    let mut seen: HashSet<Stamp> = HashSet::new();
    let map_size = (map[0].len() as u8, map.len() as u8);
    posses.push_back(Stamp{time: start_time, x: start.0, y: start.1});
    'outer: while posses.len() > 0
    {
        let stamp = posses.pop_front().unwrap();
        if stamp.x >= map_size.0 || stamp.y >= map_size.1
        {
            continue;
        }
        if map[stamp.y as usize][stamp.x as usize] == '#'
        {
            continue;
        }
        if seen.contains(&stamp)
        {
            continue;
        }
        seen.insert(stamp);

        if stamp.x == end.0 && stamp.y == end.1
        {
            return stamp.time;
        }
        //let mut new_map = map.clone();
        for b in blizzards
        {
            let mut x = b.x as i64;
            let mut y = b.y as i64;
            let _c = match b.dir
            {
                0 => { x += stamp.time as i64; '>'},
                1 => { y += stamp.time as i64; 'v'},
                2 => { x -= stamp.time as i64; '<'},
                3 => { y -= stamp.time as i64; '^'},
                _ => {'.'}
            };
            let x = (x.rem_euclid(map_size.0 as i64 - 2) + 1) as u8;
            let y = (y.rem_euclid(map_size.1 as i64 - 2) + 1) as u8;
            //assert!(x > 0 && y > 0 && x < map_size.0 - 1 && y < map_size.1 - 1);
            if x == stamp.x && y == stamp.y
            {
                continue 'outer;
            }
            //new_map[y as usize][x as usize] = _c;
        }
        //new_map[pos.1 as usize][pos.0 as usize] = '@';
        //println!("Time: {}", time);
        //_print_map(&new_map);
        if stamp.x > 0
        {
            posses.push_back(Stamp{time: stamp.time + 1, x: stamp.x - 1, y: stamp.y});
        }
        if stamp.y > 0
        {
            posses.push_back(Stamp{time: stamp.time + 1, x: stamp.x, y: stamp.y - 1});
        }
        if stamp.x < map_size.0
        {
            posses.push_back(Stamp{time: stamp.time + 1, x: stamp.x + 1, y: stamp.y});
        }
        if stamp.y < map_size.1
        {
            posses.push_back(Stamp{time: stamp.time + 1, x: stamp.x, y: stamp.y + 1});
        }
        posses.push_back(Stamp{time: stamp.time + 1, x: stamp.x, y: stamp.y});
    }
    return !0;
}

fn parse(content: &'static str) -> (Vec<Blizzard>, (u8, u8), (u8, u8), Vec<Vec<char>>)
{
    let mut blizzards: Vec<Blizzard> = Vec::new();
    let lines = content.lines().collect::<Vec<&str>>();
    let start = lines[0].find('.').unwrap();
    let end = lines[lines.len() - 1].find('.').unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();

    for row in 0..lines.len() as u8
    {
        let mut line: Vec<char> = Vec::new();
        for (col, &c) in lines[row as usize].as_bytes().iter().enumerate()
        {
            line.push('.');
            match c as char
            {
                '>' => { blizzards.push(Blizzard {x: col as u8 - 1, y: row - 1, dir: 0, _padding: 0} )},
                'v' => { blizzards.push(Blizzard {x: col as u8 - 1, y: row - 1, dir: 1, _padding: 0} )},
                '<' => { blizzards.push(Blizzard {x: col as u8 - 1, y: row - 1, dir: 2, _padding: 0} )},
                '^' => { blizzards.push(Blizzard {x: col as u8 - 1, y: row - 1, dir: 3, _padding: 0} )},
                '#' => { line[col] = '#' },
                _ => {}
            }
        }
        map.push(line);
    }
    let start = (start as u8, 0u8);
    let end = (end as u8, lines.len() as u8 - 1);
    return (blizzards, start, end, map);
}

#[test]
fn part_a_test()
{
    let value = part_a(&_TEST_DATA);
    assert_eq!(value, 18);
}

fn part_a(content: &'static str) -> i64
{
    let (blizzards, start, end, map) = parse(content);

    let fastest = simulate(
        start,
        0,
        &blizzards,
        end,
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
    let (blizzards, start, end, map) = parse(content);

    let fastest = simulate(start,0, &blizzards, end, &map);
    let fastest = simulate(end, fastest as u16, &blizzards, start, &map);
    let fastest = simulate(start, fastest as u16, &blizzards, end, &map);
    return fastest as i64;
}



