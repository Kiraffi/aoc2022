
const RUN_AMOUNT:u32 = 1;
const DAY_STR: &'static str = "17";
const DATA: &'static str = include_str!("../../data/day17.txt");
const _TEST_DATA: &'static str = include_str!("../test_data.txt");

const BLOCKS: usize = 5;
const SHAPES: [[&'static str; 4]; BLOCKS] = [
    [
        "....",
        "....",
        "....",
        "####"
    ],

    [
        "....",
        ".#..",
        "###.",
        ".#..",
    ],

    [
        "....",
        "..#.",
        "..#.",
        "###.",
    ],

    [
        "#....",
        "#....",
        "#....",
        "#....",
    ],

    [
        "....",
        "....",
        "##..",
        "##..",
    ],
];

// Rust not allowing for in const fn....
const fn get_bit_row(row: &'static str) -> u32
{
    let mut row_value = 0u32;
    let bytes = row.as_bytes();
    if bytes[0] == '#' as u8 { row_value |= 1; }
    if bytes[1] == '#' as u8 { row_value |= 2; }
    if bytes[2] == '#' as u8 { row_value |= 4; }
    if bytes[3] == '#' as u8 { row_value |= 8; }
    return row_value;
}

const fn get_bit_shape(shape: &[&'static str; 4]) -> u32
{
    let mut block_value = 0u32;
    block_value |= get_bit_row(shape[3]) << (8 * 0 + 2);
    block_value |= get_bit_row(shape[2]) << (8 * 1 + 2);
    block_value |= get_bit_row(shape[1]) << (8 * 2 + 2);
    block_value |= get_bit_row(shape[0]) << (8 * 3 + 2);
    return block_value;
}

const fn get_bit_shapes(shapes: &[[&'static str; 4]; BLOCKS]) -> [u32; BLOCKS]
{
    return [
        get_bit_shape(&shapes[0]),
        get_bit_shape(&shapes[1]),
        get_bit_shape(&shapes[2]),
        get_bit_shape(&shapes[3]),
        get_bit_shape(&shapes[4]),
    ];
}

const BIT_SHAPES: [u32; BLOCKS] = get_bit_shapes(&SHAPES);

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
    println!("Day {}-1: Row height after 2022 blocks: {}", DAY_STR, part_a(&DATA));
    //println!("Day {}-2: Row height after 1 000 000 000 000 blocks: {}", DAY_STR, part_b(&DATA));
    println!("Day {} duration: {}us", DAY_STR, now.elapsed().as_micros() as f32 / RUN_AMOUNT as f32);
}

fn check_collide(board: &Vec<u8>, block: u32, block_y: i64) -> bool
{
    let coll: u32 = unsafe { *board.as_ptr().offset(block_y as isize ).cast::<u32>() };
    return coll & block != 0;
}

fn add_block(board: &mut Vec<u8>, block: u32, block_y: i64) -> i64
{
    if block_y < 0
    {
        return 0;
    }

    unsafe
    {
        let b32 = board.as_mut_ptr().offset(block_y as isize ).cast::<u32>();
        *b32 |= block;
    }
    let coll: u32 = unsafe { *board.as_ptr().offset(block_y as isize ).cast::<u32>() };

    let height = (32 - coll.leading_zeros()) / 8;
    return block_y + height as i64 + 1;
}


fn move_block(board: &Vec<u8>, block: u32, block_y: i64, command: i8) -> u32
{
    // Lift block up one row, to not overflow with shifts.
    let mut new_pos = (block as u64) << (8 + command);

    // Bottom row
    if new_pos & (255
        // Every 8th bit
        | 0x8080_8080_8080
        // Nothing after 5th row
        | (!((1 << 40) - 1))) != 0
    {
        return block;
    }
    // Lower it back
    new_pos = new_pos >> 8;
    if check_collide(board, new_pos as u32, block_y)
    {
        return block;
    }

    return new_pos as u32;
}

fn move_block_no_check(block: u32, command: i8) -> u32
{
    return block << command;
}


fn _print_line(v: u8)
{
    print!("|");
    for i in 0..7
    {
        print!("{}", if ((v >> i) & 1) != 0 {'#'} else {'.'} );
    }
    assert_eq!((v >> 7), 0);
    print!("|");
    println!("");
}
fn _print_block(block: u32)
{
    for i in 0..4
    {
        _print_line(((block >> ((3 - i) * 8)) & 255) as u8);
    }
    println!("");
}

fn _print_board(board: &Vec<u8>, from: usize, lines: usize)
{
    println!("from: {} to {}", from, lines + from);
    for i in 0..lines
    {
        _print_line(board[(from + lines) - 1 - i]);
    }
}

fn _print_shapes()
{
    for c in BIT_SHAPES
    {
        for i in 0..4
        {
            _print_line(((c >> ((3 - i) * 8)) & 255) as u8);
        }
        println!("");
    }
}

fn get_new_shape(precalculated: &Vec<Vec<u32>>, block_count: i64, command_count: usize) -> u32
{
    let new_block
        = precalculated[(block_count as usize) % BLOCKS][command_count];
    return new_block;
}

fn get_precalculated_starts(commands: &Vec<i8>) -> Vec<Vec<u32>>
{
    let mut result: Vec<Vec<u32>> = Vec::new();
    let tmp_board: Vec<u8> = vec![0u8, 0u8, 0u8, 0u8];
    for i in 0..BLOCKS
    {
        let mut v: Vec<u32> = Vec::new();

        for j in 0..commands.len()
        {
            let mut block = BIT_SHAPES[i];
            for k in 0..4
            {
                let index = (j + k) % commands.len();
                block = move_block(&tmp_board, block, 0, commands[index]);
            }
            v.push(block);
        }
        result.push(v);
    }

    return result;
}

fn parse_commands(content: &'static str) -> Vec<i8>
{
    let mut commands: Vec<i8> = Vec::new();
    for &c in content.as_bytes()
    {
        match c
        {
            60 => commands.push(-1),
            62 => commands.push(1),
            _ => ()
        }
    }
    return commands;
}
/*
fn calculate_move(state: u128, mut block: u128, block_index: usize, command: &Command)
{
    if block & state != 0
    {
        return;
    }
    if *command == Command::Left { block <<= 9; }
    else { block = 1; }


}

fn calculate_moves(content: &'static str)
{
    let commands = parse_commands(content);
    let mut dirs: Vec<Command> = Vec::new();
}


fn something(mut commands: u32) -> (usize, i64)
{
    let mut board: Vec<u8> = Vec::new();
    board.resize(32, 0);
    let mut height = 0;
    let mut command_amount = 0;

    for i in 0 .. BLOCKS
    {
        let mut block = BIT_SHAPES[i % BLOCKS];
        let mut posy = height + 3;

        while posy >= 0 && !check_collide(&board, block, posy)
        {
            block = match commands & 1
            {
                // Right
                0 => move_block(&board, block, posy, &Command::Left),
                1 => move_block(&board, block, posy, &Command::Right),
                _ => block
            };
            command_amount += 1;
            commands >>= 1;
            //assert!(command_amount < 32);
            posy -= 1;
        }
        if posy < 0 && (i % BLOCKS) != 0
        {
            return (0, 0);
        }
        height = std::cmp::max(height, add_block(&mut board, block, posy + 1));
    }
    return (command_amount, height);
}

*/
fn get_row_count(content: &'static str, block_count_to: i64) -> (i64, i64)
{
    let commands = parse_commands(content);
    println!("amount: {}", commands.len());

    let precalculated = get_precalculated_starts(&commands);
    //_print_shapes();

    let mut thirty = 0;

    let mut board: Vec<u8> = Vec::new();
    board.resize(1024 * 4, 0);
    let mut block_count = 0;
    let mut command_count = 0;
    let mut row_height = 0i64;
    let mut row_offset = 0i64;
    while block_count < block_count_to
    {
        let start = row_height + 3 - row_offset - 4;
        let mut block_y = start;
        let mut new_block = get_new_shape(&precalculated, block_count, command_count % commands.len());
        //let mut new_block = BIT_SHAPES[(block_count as usize) % BLOCKS];
        //new_block= move_block_no_check(new_block, &commands[(command_count + 0) % commands.len()]);
        //new_block= move_block_no_check(new_block, &commands[(command_count + 1) % commands.len()]);
        command_count += 4;
        while block_y >= 0 && !check_collide(&board, new_block, block_y)
        {
            new_block = move_block(&board, new_block, block_y, commands[command_count % commands.len()]);
            block_y -= 1;
            command_count += 1;
        }

        row_height = std::cmp::max(row_height, add_block(&mut board, new_block, block_y + 1) + row_offset);
        block_count += 1;

        if row_height - row_offset > 4000
        {
            unsafe
            {
                let mut b1 = board.as_mut_ptr().cast::<u64>();
                let mut b2 = board.as_ptr().add(2000).cast::<u64>();

                for i in 0..(board.len() - 2000) / 8
                {
                    *b1 = *b2;
                    b1 = b1.add(1);
                    b2 = b2.add(1);
                }

                for _ in 0..2000 / 8
                {
                    *b1 = 0;
                    b1 = b1.add(1);
                }
            }
            row_offset += 2000;
        }

        if row_height % 1000000 == 0
        {
            println!("Blocks: {}", block_count);
        }
    }
    //println!("");
    println!("command: {} vs commands: {}, commands %: {}", command_count, commands.len(), command_count % commands.len());
    //_print_board(&board,0, 16);
    //_print_board(&board, (row_height - row_offset) as usize - 15, 16);
    //println!("rowoffset: {}", row_offset);
    //println!("thirty {}", thirty);
    println!("Rowheight: {}", row_height);
    return (row_height, block_count);
}

#[test]
fn part_a_test()
{
    let value = part_a(&_TEST_DATA);
    assert_eq!(value, 3068);
}

fn part_a(content: &'static str) -> i64
{
    return get_row_count(content, 2022).0;
}

#[test]
fn part_b_test()
{
    let value = part_b(&_TEST_DATA);
    assert_eq!(value, 1514285714288);
}

fn part_b(content: &'static str) -> i64
{
    return get_row_count(content, 1_000_000_000i64).0;

}

