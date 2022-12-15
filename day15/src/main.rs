use std::collections::HashSet;


const RUN_AMOUNT:u32 = 1;
const DAY_STR: &'static str = "15";
const DATA: &'static str = include_str!("../../data/day15.txt");
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
    println!("Day {}-1: The beacon cannot be in this many locations: {}", DAY_STR, part_a(&DATA, 2000000));
    println!("Day {}-2: Beacon is at: {}", DAY_STR, part_b(&DATA, 4_000_000, 4_000_000));
    println!("Day {} duration: {}us", DAY_STR, now.elapsed().as_micros() as f32 / RUN_AMOUNT as f32);
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point
{
    x: i64,
    y: i64
}

impl Point
{
    fn min(a: &Point, b: &Point) -> Point
    {
        Point{x: std::cmp::min(a.x, b.x), y: std::cmp::min(a.y, b.y)}
    }
    fn max(a: &Point, b: &Point) -> Point
    {
        Point{x: std::cmp::max(a.x, b.x), y: std::cmp::max(a.y, b.y)}
    }
}

struct Sensor
{
    pos: Point,
    distance: i64
}

#[test]
fn part_a_test()
{
    let value = part_a(&_TEST_DATA, 10);
    assert_eq!(value, 26);
}

fn get_number(s: &str) -> i64
{
    let s2 = s.split_once('=').unwrap().1;
    let s3 = s2.split_once(',').unwrap_or_else(||{(s2, "")}).0;
    let s4 = s3.split_once(':').unwrap_or_else(||{(s3, "")}).0;
    return s4.parse::<i64>().unwrap();
}

fn parse_data(content: &'static str) -> (Vec<Sensor>, HashSet<Point>, Point, Point)
{
    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacons: HashSet<Point> = HashSet::new();
    let mut min = Point{x: i64::MAX, y: i64::MAX};
    let mut max = Point{x: i64::MIN, y: i64::MIN};
    for line in content.lines()
    {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let x = get_number(parts[2]);
        let y = get_number(parts[3]);
        let sensor = Point{x: x, y: y};

        let beacon_x = get_number(parts[8]);
        let beacon_y = get_number(parts[9]);
        let beacon = Point{x: beacon_x, y: beacon_y};

        min = Point::min(&min, &sensor);
        min = Point::min(&min, &beacon);
        max = Point::max(&max, &sensor);
        max = Point::max(&max, &beacon);

        let diff_x = i64::abs(beacon_x - x);
        let diff_y = i64::abs(beacon_y - y);
        let distance = diff_x + diff_y;
        let p1 = Point{x: sensor.x + distance, y: sensor.y + distance};
        let p2 = Point{x: sensor.x - distance, y: sensor.y - distance};

        min = Point::min(&min, &p1);
        min = Point::min(&min, &p2);

        max = Point::max(&max, &p1);
        max = Point::max(&max, &p2);
        //println!("p1 {}:{}, p2 {}:{}", p1.x, p1.y, p2.x, p2.y);

        beacons.insert(beacon);
        sensors.push(Sensor {pos: sensor, distance: distance});
    }
    return (sensors, beacons, min, max);
}

fn part_a(content: &'static str, row: i64) -> i64
{
    let (sensors, beacons, min, max)
        = parse_data(content);

    let mut visible_points = 0;
    for x in min.x ..= max.x
    {
        for sensor in &sensors
        {
            if beacons.contains(&Point{x: x, y: row})
            {
                continue;
            }
            let diff_x = i64::abs(sensor.pos.x - x);
            let diff_y = i64::abs(sensor.pos.y - row);
            let distance = diff_x + diff_y;
            if sensor.distance >= distance
            {
                visible_points += 1;
                break;
            }
        }
    }
    println!("min {}:{}, max {}:{}", min.x, min.y, max.x, max.y);
    return visible_points;
}

#[test]
fn part_b_test()
{
    let value = part_b(&_TEST_DATA, 20, 20);
    assert_eq!(value, 14 * 4000000 + 11);
}

fn part_b(content: &'static str, search_x :i64, search_y: i64) -> i64
{

    let (sensors, _, _, _)
        = parse_data(content);
/*
    let mut map: Vec<Vec<char>> = Vec::new();
    map.resize((search_y + 1) as usize, Vec::new());
    for y in 0..search_y + 1
    {
        map[y as usize].resize((search_x + 1) as usize, ' ');
    }
    println!("Resized");
    for sensor in &sensors
    {
        println!("Distance: {}", sensor.distance);
        let y_s = i64::max(sensor.pos.y - sensor.distance, 0);
        let y_e = i64::min(sensor.pos.y + sensor.distance, search_y);
        for y in y_s..=y_e
        {
            let diff_y = i64::abs(y - sensor.pos.y);
            let x_diff = sensor.distance - diff_y;
            let x_s = i64::max(0, sensor.pos.x - x_diff);
            let x_e = i64::min(search_y, sensor.pos.x + x_diff);
            for x in x_s..=x_e
            {
                map[y as usize][x as usize] = '#';
            }
        }
        println!("Filled sensor");
    }
    println!("Filled");

    for y in 0..=search_y
    {
        for x in 0..=search_x
        {
            if map[y as usize][x as usize] == ' '
            {
                return (x, y);
            }
        }
    }
    */

    for y in 0 ..= search_y
    {
        let mut x = 0;
        while x <= search_x
        {
            let mut visible = false;
            for sensor in &sensors
            {
                let diff_x = i64::abs(sensor.pos.x - x);
                let diff_y = i64::abs(sensor.pos.y - y);
                let distance = diff_x + diff_y;
                if sensor.distance >= distance
                {
                    let warp = sensor.distance - diff_y;
                    x = sensor.pos.x + warp;
                    visible = true;
                    break;
                }
            }
            if !visible
            {
                return x * 4000000 + y;
            }
            x += 1;
        }
    }

    println!("Didn't find any point");
    return 0;
}



