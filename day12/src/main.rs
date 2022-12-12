use std::collections::VecDeque;



const RUN_AMOUNT:u32 = 1;
const DAY_STR: &'static str = "12";
const DATA: &'static str = include_str!("../../data/day12.txt");
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
    println!("Day {}-1: Lowest amount of steps: {}", DAY_STR, part_a(&DATA));
    println!("Day {}-2: Lowest amount from any a-height: {}", DAY_STR, part_b(&DATA));
    println!("Day {} duration: {}us", DAY_STR, now.elapsed().as_micros() as f32 / RUN_AMOUNT as f32);
}
struct Pos
{
    x: i32,
    y: i32,
    distance: usize
}
struct Tile
{
    distance: usize,
    height: u8,
}

struct HeightMap
{
    map: Vec<Vec<Tile>>,
    pos_end_x: i32,
    pos_end_y: i32
}

fn is_valid_tile(map: & HeightMap, pos_x: i32, pos_y: i32) -> bool
{
    if pos_x < 0 || pos_x >= map.map[0].len() as i32
        || pos_y < 0 || pos_y >= map.map.len() as i32
    {
        return false;
    }
    return true;
}

fn get_height_helper(map: & HeightMap, pos_x: i32, pos_y: i32) -> u8
{
    if !is_valid_tile(map, pos_x, pos_y)
    {
        return !0u8;
    }
    return map.map[pos_y as usize][pos_x as usize].height;
}

fn find_exit(map: &mut HeightMap, pos_x: i32, pos_y: i32) -> usize
{
    let mut posses: VecDeque<Pos> = VecDeque::new();
    posses.push_back(Pos{x: pos_x, y: pos_y, distance: 0 });

    while posses.len() > 0
    {
        let pos = posses.pop_front().unwrap();
        let pos_x = pos.x;
        let pos_y = pos.y;
        let distance = pos.distance;
        if !is_valid_tile(map, pos_x, pos_y)
        {
            continue;
        }
        let tile = &mut map.map[pos_y as usize][pos_x as usize];
        if tile.distance <= distance
        {
            continue;
        }
        tile.distance = distance;
        if pos_x == map.pos_end_x && pos_y == map.pos_end_y
        {
            return distance;
        }

        let height = get_height_helper(map, pos_x, pos_y);

        if get_height_helper(map, pos_x - 1, pos_y) <= height + 1
        {
            posses.push_back(Pos{x: pos_x - 1, y: pos_y, distance: distance + 1});
        }

        if get_height_helper(map, pos_x + 1, pos_y) <= height + 1
        {
            posses.push_back(Pos{x: pos_x + 1, y: pos_y, distance: distance + 1});
        }

        if get_height_helper(map, pos_x, pos_y - 1) <= height + 1
        {
            posses.push_back(Pos{x: pos_x, y: pos_y - 1, distance: distance + 1});
        }

        if get_height_helper(map, pos_x, pos_y + 1) <= height + 1
        {
            posses.push_back(Pos{x: pos_x, y: pos_y + 1, distance: distance + 1});
        }
    }
    return !0usize;
}

fn parse(content: &'static str) -> (HeightMap, i32, i32)
{
    let mut map = HeightMap {map: Vec::new(), pos_end_x: 0, pos_end_y: 0};
    for line in content.lines()
    {
        map.map.push(line
            .chars()
            .map(|s| {Tile{distance: !0usize, height: s as u8}}).collect());
    }
    let mut pos_start_x = 0;
    let mut pos_start_y = 0;

    for (y, tiles) in map.map.iter_mut().enumerate()
    {
        for (x, tile) in tiles.iter_mut().enumerate()
        {
            match tile.height as char
            {
                'S' => { tile.height = 'a' as u8; pos_start_x = x as i32; pos_start_y = y as i32; }
                'E' => { tile.height = 'z' as u8; map.pos_end_x = x as i32; map.pos_end_y = y as i32; },
                _ => ()
            }
        }
    }

    return (map, pos_start_x, pos_start_y);
}

#[test]
fn part_a_test()
{
    let value = part_a(&_TEST_DATA);
    assert_eq!(value, 31);
}

fn part_a(content: &'static str) -> usize
{
    let (mut map, pos_start_x, pos_start_y) = parse(content);
    return find_exit(&mut map, pos_start_x, pos_start_y);
}

#[test]
fn part_b_test()
{
    let value = part_b(&_TEST_DATA);
    assert_eq!(value, 29);
}

fn part_b(content: &'static str) -> usize
{
    let mut smallest = !0usize;
    let (mut map, _, _) = parse(content);

    let mut start_xs: Vec<i32> = Vec::new();
    let mut start_ys: Vec<i32> = Vec::new();

    for (y, tiles) in map.map.iter_mut().enumerate()
    {
        for (x, tile) in tiles.iter_mut().enumerate()
        {
            match tile.height as char
            {
                'a' => { start_xs.push(x as i32); start_ys.push(y as i32); }
                _ => ()
            }
        }
    }
    for i in 0..start_xs.len()
    {
        let x = start_xs[i];
        let y = start_ys[i];
        for tiles in map.map.iter_mut()
        {
            for tile in tiles.iter_mut()
            {
                tile.distance = !0usize;
            }
        }
        smallest = std::cmp::min(smallest,
            find_exit(&mut map, x, y));
    }
    return smallest;
}
