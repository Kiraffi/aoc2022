use std::mem::swap;

const FILE_PATH:&str = "day1.txt";

fn main()
{
    let now = std::time::Instant::now();
    day1();
    println!("Day1 duration: {}us", now.elapsed().as_micros());
}

fn day1()
{
    const NUMBER_AMOUNT:usize = 3;
    let mut max_amounts: [i32; NUMBER_AMOUNT] = [0; NUMBER_AMOUNT];
    let mut current_amount = 0;

    common_lib::read_line_by_line(FILE_PATH,
        |line: &str|
        {
            if line.len() != 0
            {
                current_amount += line.parse::<i32>().unwrap();
            }
            else
            {
                for n in 0..NUMBER_AMOUNT
                {
                    if max_amounts[n] < current_amount
                    {
                        swap(&mut max_amounts[n], &mut current_amount);
                    }
                }
                current_amount = 0;
            }
            return true;
        }
    );

    let mut sum_of_maxes = 0;
    for value in max_amounts
    {
        sum_of_maxes += value;
    }
    println!("Day 1-1: Highest: {}", max_amounts[0]);
    println!("Day 1-2: Sum of highest {} values: {}", NUMBER_AMOUNT, sum_of_maxes);
}
