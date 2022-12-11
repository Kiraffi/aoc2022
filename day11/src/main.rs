

const RUN_AMOUNT:u32 = 1;
const DAY_STR: &'static str = "11";

fn main()
{
    let _test_data = include_str!("../test_data.txt");
    let _data = include_str!("../../data/day11.txt");

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

fn parse(content: &'static str) -> Vec<Monkey>
{
    let mut monkey_index = 0;
    let mut monkeys: Vec<Monkey> = Vec::new();
    for line in content.lines()
    {
        let line = line.trim();

        if line.starts_with("Monkey")
        {
            monkeys.push(Monkey{
                items: Vec::new(), op: Op::Add(0),
                test: 0, true_cond: 0, false_cond: 0, inspects: 0});
            monkey_index = monkeys.len() - 1;
        }
        let monkey = &mut monkeys[monkey_index];
        let words: Vec<&str> = line.split(' ').collect();

        if words[0] == "Starting"
        {
            for i in 2..words.len()
            {
                let num = words[i].split(',').next().unwrap();
                monkey.items.push(num.parse::<u64>().unwrap());
            }
        }
        else if words[0] == "Operation:"
        {
            let value = words[5].parse::<u64>().unwrap_or(!0u64);
            match words[4]
            {
                "+" => monkey.op = Op::Add(value),
                _ => monkey.op = Op::Mul(value)
            }
        }
        else if words[0] == "Test:"
        {
            monkey.test = words[3].parse::<u64>().unwrap();
        }
        else if words.len() > 1 && words[1] == "true:"
        {
            monkey.true_cond = words[5].parse::<usize>().unwrap();
        }
        else if  words.len() > 1 && words[1] == "false:"
        {
            monkey.false_cond = words[5].parse::<usize>().unwrap();
        }
    }
    return monkeys;
}

fn throw_packets<F>(monkeys: &mut Vec<Monkey>, rounds: usize, f: F)
    where F:Fn(u64) -> u64
{
    for _ in 0..rounds
    {
        for i in 0..monkeys.len()
        {
            // Borrow checeker.......................
            let mut false_throws: Vec<u64> = Vec::new();
            let mut true_throws: Vec<u64> = Vec::new();
            let m = &monkeys[i];
            for &i in m.items.iter()
            {
                let i = match m.op
                {
                    Op::Add(x) => if x != !0u64 { i + x } else { i + i },
                    Op::Mul(x) => if x != !0u64 { i * x } else { i * i },
                };
                let i = f(i); //i / 3;
                if i % m.test == 0
                {
                    true_throws.push(i);
                }
                else
                {
                    false_throws.push(i);
                }
            }
            let false_throw_index = m.false_cond;
            let true_throw_index = m.true_cond;
            monkeys[false_throw_index].items.append(&mut false_throws);
            monkeys[true_throw_index].items.append(&mut true_throws);
            monkeys[i].inspects += monkeys[i].items.len();
            monkeys[i].items.clear();
        }
    }
}

fn get_inspects(monkeys: &Vec<Monkey>) -> usize
{
    let mut inspects:Vec<usize> = Vec::new();
    for m in monkeys
    {
        inspects.push(m.inspects);
    }
    inspects.sort();
    inspects.reverse();
    return inspects[0] * inspects[1];
}

enum Op
{
    Add(u64),
    Mul(u64)
}

struct Monkey
{
    items: Vec<u64>,
    op: Op,
    test: u64,
    true_cond: usize,
    false_cond: usize,
    inspects: usize
}

fn part_a(print_outcome: bool, content: &'static str)
{
    let mut monkeys = parse(content);
    throw_packets(&mut monkeys, 20, |x| { x / 3});

    let inspects = get_inspects(&monkeys);
    if print_outcome
    {
        println!("Day {}-1: Inspects: {}", DAY_STR, inspects);
    }
}

fn part_b(print_outcome: bool, content: &'static str)
{
    let mut monkeys = parse(content);
    let mut divisable = 1;
    for i in 0..monkeys.len()
    {
            // Borrow checeker.......................
            divisable *= monkeys[i].test;
    }
    throw_packets(&mut monkeys, 10000, |x| { x % divisable});
    let inspects = get_inspects(&monkeys);
    if print_outcome
    {
        println!("Day {}-2: Inspects: {}", DAY_STR, inspects);
    }
}
