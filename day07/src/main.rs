
const RUN_AMOUNT:u32 = 1;
const DAY_STR: &'static str = "07";

fn main()
{
    let _test_data = include_str!("../test_data.txt");
    let _data = include_str!("../../data/day07.txt");

    let now = std::time::Instant::now();
    /*
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

fn dir_down<F>(
    mut f: F,
    amounts: &mut Vec<usize>,
    dir_amount: &mut i32)
    where F: FnMut(usize)
{
    let amount = (*dir_amount) as usize;
    f(amounts[amount]);
    if amount > 0
    {
        amounts[amount - 1] += amounts[amount];
    }
    *dir_amount -= 1;
}


fn calculate_size<F>(content: &'static str, mut f: F) -> usize
    where F: FnMut(usize)
{
    let mut dir_amount = 0;
    let mut amounts: Vec<usize> = Vec::new();

    for line in content.lines()
    {
        let mut words = line.split(' ');

        let word0 = words.next().unwrap();
        let word1 = words.next().unwrap();
        let word2 = match words.next() { None => "", Some(x) => x};
        if word0 == "$" && word1 == "cd"
        {
            if word2 == "/"
            {
                dir_amount = 0;
                amounts = Vec::new();
                amounts.push(0);
            }
            else if word2 == ".."
            {
                dir_down(&mut f, &mut amounts, &mut dir_amount);
            }
            else
            {
                dir_amount += 1;
                if dir_amount as usize >= amounts.len()
                {
                    amounts.push(0);
                }
                amounts[dir_amount as usize] = 0;
            }
        }
        else if word0 != "$" && word0 != "dir"
        {
            let file_size = word0.parse::<usize>().unwrap();
            amounts[dir_amount as usize] += file_size;
        }
    }
    while dir_amount >= 0
    {
        dir_down(&mut f, &mut amounts, &mut dir_amount);
    }

    return amounts[0];
}

fn part_a(print_outcome: bool, content: &'static str)
{
    let mut total_under_100k = 0;
    calculate_size(content, |dir_size|
    {
        if dir_size < 100000
        {
            total_under_100k += dir_size;
        }
    });
    if print_outcome
    {
        println!("Day {}-1: Sum of size of directories with size under 100000: {}", DAY_STR, total_under_100k);
    }
}


fn part_b(print_outcome: bool, content: &'static str)
{
    let used_space = calculate_size(content, |_|{});

    let mut smallest_folder = !0;
    if 70000000 - used_space >= 30000000
    {
        smallest_folder = 0;
    }
    calculate_size(content, |dir_size|
    {
        if dir_size >= 30000000 - (70000000 - used_space)
        {
            smallest_folder = std::cmp::min(smallest_folder, dir_size);
        }
    });
    if print_outcome
    {
        println!("Day {}-2: Smallest directory to have over 30000000: {}", DAY_STR, smallest_folder);
    }
}
