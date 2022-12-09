
const RUN_AMOUNT:u32 = 1;
const DAY_STR: &'static str = "09";

fn main()
{
    let _test_data = include_str!("../test_data.txt");
    let _data = include_str!("../../data/day09.txt");

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
    part_b(true, &_data);
    println!("Day {} duration: {}us", DAY_STR, now.elapsed().as_micros() as f32 / RUN_AMOUNT as f32);
}

#[derive(Clone, Copy)]
struct Pos
{
    x: i32,
    y: i32
}

fn simulate_movement(content: &'static str, knots: usize) -> usize
{
    let mut visited_squares = 0;
    let mut rope: Vec<Pos> = Vec::new();
    rope.resize(knots, Pos{x: 0, y: 0});

    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut map: Vec<Vec<u8>> = Vec::new();
    map.push(Vec::new());
    for line in content.lines()
    {
        let mut parsed = line.split(' ');
        let l = parsed.next().unwrap();
        let l = l.as_bytes();
        let amount = parsed.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..amount
        {
            match l[0] as char
            {
                'L' => rope[0].x -= 1,
                'R' => rope[0].x += 1,
                'U' => rope[0].y -= 1,
                'D' => rope[0].y += 1,
                _ => ()
            }
            for i in 1..knots
            {
                let prev = rope[i - 1];
                let mut curr = &mut rope[i];
                let diff_x = prev.x - curr.x;
                let diff_y = prev.y - curr.y;
                let abs_diff_x = i32::abs(diff_x);
                let abs_diff_y = i32::abs(diff_y);

                if (abs_diff_x >= 1 && abs_diff_y >= 1)
                    && (abs_diff_x >= 2 || abs_diff_y >= 2)
                {
                    curr.x += diff_x / abs_diff_x;
                    curr.y += diff_y / abs_diff_y;
                }
                else if abs_diff_x >= 2
                {
                    curr.x += diff_x / abs_diff_x;
                }
                else if abs_diff_y >= 2
                {
                    curr.y += diff_y / abs_diff_y;
                }
            }
            let tail = rope[knots - 1];
            if tail.x < 0
            {
                for j in 0..height
                {
                    map[j as usize].insert(0, 0);
                }
                for knot in &mut rope
                {
                    knot.x += 1;
                }
                width += 1
            }
            if tail.x >= width
            {
                for j in 0..height
                {
                    map[j as usize].push(0);
                }
                width += 1
            }

            if tail.y < 0
            {
                map.insert(0, Vec::new());
                map[0].resize(width as usize, 0);
                height += 1;

                for knot in &mut rope
                {
                    knot.y += 1;
                }
            }

            if tail.y >= height
            {
                map.push(Vec::new());
                map[height as usize].resize(width as usize, 0);
                height += 1;
            }
            let tail = rope[knots - 1];
            map[tail.y as usize][tail.x as usize] = 1;
        }
    }
    for m in map
    {
        for n in m
        {
            if n == 1
            {
                visited_squares += 1;
            }
        }
    }
/*
    for m in map
    {
        for n in m
        {
            if n == 0
            {
                print!("{}", ' ');
            }
            else
            {
                print!("{}", '#');
            }
        }
        println!("");
    }
*/

    return visited_squares;
}

fn part_a(print_outcome: bool, content: &'static str)
{
    let visited_tiles = simulate_movement(content, 2);
    if print_outcome
    {
        println!("Day {}-1: Tail visiting tile amount: {}", DAY_STR, visited_tiles);
    }
}

fn part_b(print_outcome: bool, content: &'static str)
{
    let visited_tiles = simulate_movement(content, 10);
    if print_outcome
    {
        println!("Day {}-2: Tail visited tile amount: {}", DAY_STR, visited_tiles);
    }
}
