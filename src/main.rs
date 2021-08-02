use std::io::{self, Write};
use std::vec::Vec;
use rand::Rng;
fn main() {
    println!("Initializing board...");
    let mut board: [i32; 16] = [0; 16];
    add_2(&mut board);
    loop {
        print_board(&board);
        let status = board_status(&board);
        if status != 0 {
            let next = quit_or_restart();
            if next == 1 {
                board = [0;16];
                add_2(&mut board);
                continue;
            } else {
                break;
            }
        }
        print!("> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read a line");
        if input.starts_with("q") {
            println!("Quitting...");
            break;
        } else if input.starts_with("r") {
            println!("Restarting...");
            board = [0;16];
            add_2(&mut board);
            continue;
        } else if input.starts_with("w") {
            println!("Move up");
            move_up(&mut board);
        } else if input.starts_with("a") {
            println!("Move left");
            move_left(&mut board);
        } else if input.starts_with("s") {
            println!("Move down");
            move_down(&mut board);
        } else if input.starts_with("d") {
            println!("Move right");
            move_right(&mut board);
        } else {
            println!("I did not understand the input: \"{}\"", input );
            continue;
        }
        add_1(&mut board);
    }
}

fn quit_or_restart()-> i32 {
    // 0: Quitting
    // 1: Restarting
    loop {
        println!("Would you like to (q)uit or (r)estart?");
        print!("> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read a line");
        if input.starts_with("q") {
            println!("Quitting");
            return 0;
        } else if input.starts_with("r") {
            println!("Restarting");
            return 1;
        } else {
            println!("I did not understand input");
        }
    }
}

fn board_status(board: &[i32;16])-> i32 {
    // 0 is continue
    // 1 is 2048 found
    // -1 is for game lost
    for i in 0..16 {
        if board[i] == 2048 {
            println!("You won!");
            return 1;
        } else if board[i] == 0 {
            println!("Continue");
            return 0;
        }
        let idxs: [i32;4] = [i as i32 - 1, i as i32 +1, i as i32 -4, i as i32 +4];
        for j in idxs {
            if j >= 0 && j < 16 {
                if board[i] == board[j as usize] {
                    println!("Continue");
                    return 0;
                }
            }
        }
    }
    println!("You loose!");
    -1
}


fn move_left(board: &mut [i32;16]) {
    let indices = [3, 2, 1, 0];
    for i in 0..4 {
        let row = [board[indices[0]+i*4], board[indices[1]+i*4], board[indices[2]+i*4], board[indices[3]+i*4]];
        let result: [i32;4] = resolve(row);
        for j in 0..4 {
            board[indices[j]+i*4] = result[j];
        }
    }
}

fn move_right(board: &mut [i32;16]) {
    let indices = [0, 1, 2, 3];
    for i in 0..4 {
        let row = [board[indices[0]+i*4], board[indices[1]+i*4], board[indices[2]+i*4], board[indices[3]+i*4]];
        let result: [i32;4] = resolve(row);
        for j in 0..4 {
            board[indices[j]+i*4] = result[j];
        }
    }
}

fn move_down(board: &mut [i32;16]) {
    let indices = [0, 4, 8, 12];
    for i in 0..4 {
        let row = [board[indices[0]+i], board[indices[1]+i], board[indices[2]+i], board[indices[3]+i]];
        let result: [i32;4] = resolve(row);
        for j in 0..4 {
            board[indices[j]+i] = result[j];
        }
    }
}

fn move_up(board: &mut [i32;16]) {
    let indices = [12, 8, 4, 0];
    for i in 0..4 {
        let row = [board[indices[0]+i], board[indices[1]+i], board[indices[2]+i], board[indices[3]+i]];
        let result: [i32;4] = resolve(row);
        for j in 0..4 {
            board[indices[j]+i] = result[j];
        }
    }
}

fn resolve(row: [i32;4])-> [i32;4] {
    let mut stack: Vec<i32> = Vec::with_capacity(4);
    for num in 0..4 {
        if row[num] != 0 {
            stack.push(row[num]);
        }
    }

    let mut temp_stack: Vec<i32> = Vec::with_capacity(4);

    while let Some(top) = stack.pop() {
        if let Some(another) = stack.pop() {
            if top == another {
                temp_stack.push(top+another);
            } else {
                temp_stack.push(top);
                stack.push(another);
            }
        } else {
            temp_stack.push(top);
        }
    }

    while let Some(num) = temp_stack.pop() {
        stack.push(num);
    }

    let mut result: [i32;4] = [0;4];
    let mut counter: usize = 4;
    while let Some(num) = stack.pop() {
        counter -= 1;
        result[counter] = num;
    }
    result
}

fn add_1(board: &mut [i32; 16]) {
    let mut pos_zero: Vec<i32> = Vec::with_capacity(16);
    for i in 0..16 {
        if board[i] == 0 {
            pos_zero.push(i as i32);
        }
    }
    // eprintln!("Empty cells: {:?}", pos_zero);
    let num: usize = pos_zero.len();
    if num == 0 {
        // eprintln!("No empty cells in board: {:?}", pos_zero);
        return;
    }
    let mut rng = rand::thread_rng();
    let p = rng.gen_range(0..num);
    // eprintln!("position selected {:?}", pos_zero[p]);
    let val = (rng.gen_range(0..2)+1)*2;
    let i: i32 = pos_zero[p];
    board[i as usize] = val;
}

fn add_2(board: &mut [i32; 16]) {
    let mut rng = rand::thread_rng();
    let p1 = rng.gen_range(0..16);
    let mut p2 = rng.gen_range(0..16);
    while p2 == p1 {
        p2 = rng.gen_range(0..16);
    }
    let k1 = rng.gen_range(0..2);
    let k2 = rng.gen_range(0..2);
    board[p1] = (k1+1)*2;
    board[p2] = (k2+1)*2;
}

fn print_board(board: &[i32; 16]){
    println!("┌────┬────┬────┬────┐");
    println!("│{:4}│{:4}│{:4}│{:4}│", board[0], board[1], board[2], board[3]);
    println!("├────┼────┼────┼────┤");
    println!("│{:4}│{:4}│{:4}│{:4}│", board[4], board[5], board[6], board[7]);
    println!("├────┼────┼────┼────┤");
    println!("│{:4}│{:4}│{:4}│{:4}│", board[8], board[9], board[10], board[11]);
    println!("├────┼────┼────┼────┤");
    println!("│{:4}│{:4}│{:4}│{:4}│", board[12], board[13], board[14], board[15]);
    println!("└────┴────┴────┴────┘");
}