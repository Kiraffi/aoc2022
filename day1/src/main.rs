
use std::fs::File;
use std::io::{self, BufRead};
use std::mem::swap;
use std::path::Path;

const FILE_PATH:&str = "day1.txt";

fn main()
{
    const NUMBER_AMOUNT:usize = 3;
    let mut max_amounts: [i32; NUMBER_AMOUNT] = [0; NUMBER_AMOUNT];
    let mut current_amount = 0;
    if let Ok(lines) = read_lines(FILE_PATH)
    {
        // Consumes the iterator, returns an (Optional) String
        for line in lines
        {
            if let Ok(line_string) = line
            {
                if line_string == ""
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
                else
                {
                    current_amount += line_string.parse::<i32>().unwrap();
                }
            }
        }
        let mut max_three = 0;
        for value in max_amounts
        {
            max_three += value;
        }
        println!("Highest: {}", max_amounts[0]);
        println!("Sum of highest 3: {}", max_three);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}