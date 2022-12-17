
const RUN_AMOUNT:u32 = 1;
const DAY_STR: &'static str = "22";
const DATA: &'static str = include_str!("../../data/day22.txt");
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
    println!("Day {}-1: CHANGE THIS: {}", DAY_STR, part_a(&DATA));
    println!("Day {}-2: CHANGE THIS: {}", DAY_STR, part_b(&DATA));
    println!("Day {} duration: {}us", DAY_STR, now.elapsed().as_micros() as f32 / RUN_AMOUNT as f32);
}

#[derive(Clone)]
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

#[test]
fn part_a_test()
{
    let value = part_a(&_TEST_DATA);
    assert_eq!(value, 0);
}

fn part_a(content: &'static str) -> i64
{
    return 0;
}

#[test]
fn part_b_test()
{
    let value = part_b(&_TEST_DATA);
    assert_eq!(value, 0);
}

fn part_b(content: &'static str) -> i64
{
    return 0;
}



