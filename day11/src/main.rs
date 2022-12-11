

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
    let mut monkeys: Vec<Monkey> = Vec::new();
    for lines in content.lines()
        .collect::<Vec<&str>>()
        .chunks(7)
    {
        monkeys.push(Monkey{
            items: lines[1]
                .split_once(':').unwrap()
                .1
                .split(',')
                .map(|x| { x.trim().parse::<u64>().unwrap_or_default() })
                .collect(),
            op:
            {
                let value = lines[2][25..].parse::<u64>().unwrap_or(!0u64);
                match lines[2].as_bytes()[23] as char
                {
                    '+' => Op::Add(value),
                    _ => Op::Mul(value)
                }
            },
            test: lines[3][21..].parse::<u64>().unwrap(),
            true_cond: lines[4][29..].parse::<usize>().unwrap(),
            false_cond: lines[5][30..].parse::<usize>().unwrap(),
            inspects: 0
        });
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
            assert!(i != monkeys[i].false_cond
                && i != monkeys[i].true_cond
                && monkeys[i].false_cond != monkeys[i].true_cond);

            // Fight the borrow checeker...
            let m = unsafe { &mut *monkeys.as_mut_ptr().add(i) };
            let m_false = unsafe { &mut *monkeys.as_mut_ptr().add(m.false_cond) };
            let m_true = unsafe { &mut *monkeys.as_mut_ptr().add(m.true_cond) };

            m.inspects += m.items.len();
            for &i in m.items.iter()
            {
                let i = match m.op
                {
                    Op::Add(x) => if x != !0u64 { i + x } else { i + i },
                    Op::Mul(x) => if x != !0u64 { i * x } else { i * i },
                };
                let i = f(i);
                if i % m.test == 0
                {
                    m_true.items.push(i);
                }
                else
                {
                    m_false.items.push(i);
                }
            }
            m.items.clear();
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
    for m in &monkeys
    {
            divisable *= m.test;
    }
    throw_packets(&mut monkeys, 10000, |x| { x % divisable});
    let inspects = get_inspects(&monkeys);
    if print_outcome
    {
        println!("Day {}-2: Inspects: {}", DAY_STR, inspects);
    }
}
