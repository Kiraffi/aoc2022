use std::collections::HashSet;

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
    x: i8,
    y: i8,
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
    x: i8,
    y: i8
}

fn try_add(stamp: &Stamp, map: &Vec<Vec<char>>, seen: &mut HashSet<Stamp>, posses: &mut Vec<Stamp>)
{
    if stamp.x < 0
        || stamp.y < 0
        || stamp.x as usize >= map[0].len()
        || stamp.y as usize >= map.len()
    {
        return;
    }
    if map[stamp.y as usize][stamp.x as usize] != '.'
    {
        return;
    }
    if seen.contains(stamp)
    {
        return;
    }
    seen.insert(*stamp);
    posses.push(*stamp);
}

fn simulate(start: (i8, i8), start_time: u16, blizzards: &Vec<Blizzard>, end: (i8, i8), map: &Vec<Vec<char>>) -> u16
{
    let mut posses: Vec<Stamp> = Vec::new();
    let map_size = (map[0].len() as i8, map.len() as i8);
    posses.push(Stamp{x: start.0, y: start.1});
    let mut time = start_time;
    while posses.len() > 0
    {
        let mut new_posses: Vec<Stamp> = Vec::new();

        let mut new_map = map.clone();
        for b in blizzards
        {
            let mut x = b.x as i64;
            let mut y = b.y as i64;
            let c = match b.dir
            {
                0 => { x += time as i64 + 1; '>'},
                1 => { y += time as i64 + 1; 'v'},
                2 => { x -= time as i64 + 1; '<'},
                3 => { y -= time as i64 + 1; '^'},
                _ => { assert!(false); '.'}
            };
            let x = (x.rem_euclid(map_size.0 as i64 - 2) + 1) as usize;
            let y = (y.rem_euclid(map_size.1 as i64 - 2) + 1) as usize;
            //assert!(x > 0 && y > 0 && x < map_size.0 - 1 && y < map_size.1 - 1);
            new_map[y][x] = c;
        }


        //new_map[pos.1 as usize][pos.0 as usize] = '@';
        //println!("Time: {}", time);
        //_print_map(&new_map);
        let mut seen: HashSet<Stamp> = HashSet::new();
        //let mut new_map_tmp = new_map.clone();

        while posses.len() > 0
        {
            let stamp = posses.pop().unwrap();
            //println!{"{}, {}", stamp.x, stamp.y};
            //new_map_tmp[stamp.y as usize][stamp.x as usize] = '@';
            if stamp.x == end.0 && stamp.y == end.1
            {
                return time;
            }

            try_add(&Stamp{x: stamp.x + 0, y: stamp.y + 0}, &new_map, &mut seen, &mut new_posses);
            try_add(&Stamp{x: stamp.x + 1, y: stamp.y + 0}, &new_map, &mut seen, &mut new_posses);
            try_add(&Stamp{x: stamp.x - 1, y: stamp.y + 0}, &new_map, &mut seen, &mut new_posses);
            try_add(&Stamp{x: stamp.x + 0, y: stamp.y + 1}, &new_map, &mut seen, &mut new_posses);
            try_add(&Stamp{x: stamp.x + 0, y: stamp.y - 1}, &new_map, &mut seen, &mut new_posses);
        }
        //_print_map(&new_map_tmp);

        posses = new_posses;
        time += 1;
    }
    return !0;
}

fn parse(content: &'static str) -> (Vec<Blizzard>, (i8, i8), (i8, i8), Vec<Vec<char>>)
{
    let mut blizzards: Vec<Blizzard> = Vec::new();
    let lines = content.lines().collect::<Vec<&str>>();
    let start = lines[0].find('.').unwrap();
    let end = lines[lines.len() - 1].find('.').unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();

    for row in 0..lines.len() as i8
    {
        let mut line: Vec<char> = Vec::new();
        for (col, &c) in lines[row as usize].as_bytes().iter().enumerate()
        {
            line.push('.');
            match c as char
            {
                '>' => { blizzards.push(Blizzard {x: col as i8 - 1, y: row - 1, dir: 0, _padding: 0} )},
                'v' => { blizzards.push(Blizzard {x: col as i8 - 1, y: row - 1, dir: 1, _padding: 0} )},
                '<' => { blizzards.push(Blizzard {x: col as i8 - 1, y: row - 1, dir: 2, _padding: 0} )},
                '^' => { blizzards.push(Blizzard {x: col as i8 - 1, y: row - 1, dir: 3, _padding: 0} )},
                '#' => { line[col] = '#' },
                _ => {}
            }
        }
        map.push(line);
    }
    let start = (start as i8, 0i8);
    let end = (end as i8, lines.len() as i8 - 1);
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



