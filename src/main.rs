use std::{thread, time};

fn main() {
    let s = time::Duration::from_secs(1);
    let mut board: [[bool; 100]; 50] = [[false; 100]; 50];
    let mut new_board: [[bool; 100]; 50] = [[false; 100]; 50];
    //Starting position
    board[30][50] = true;
    board[29][50] = true;
    board[31][50] = true;
    board[31][51] = true;
    board[30][49] = true;

    //main loop
    loop {
        //Checking loop
        for i in 0..50 {
            for j in 0..100 {
                let mut alife: u32 = 0;
                for x in i - 1..=i + 1 {
                    for y in j - 1..=j + 1 {
                        //Don't count the same square and square not in the board.
                        if x == i && y == j {
                            continue
                        }

                        if x < 0 || y < 0 || x > 49 || y > 99 {
                            continue;
                        }

                        if board[x as usize][y as usize] {
                            alife += 1;
                        }
                    }
                }
                if board[i as usize][j as usize] && (alife == 2 || alife == 3) {
                    new_board[i as usize][j as usize] = true;
                }

                if !board[i as usize][j as usize] && alife == 3 {
                    new_board[i as usize][j as usize] = true;
                }
            }
        }

        //Printing module
        thread::sleep(s);
        print!("\x1B[2J\x1B[1;1H");
        for i in 0..50 {
            for j in 0..100 {
                if new_board[i as usize][j as usize]{
                    print!("⬜");
                }else {
                    print!("⬛");
                }

            }
            println!();
        }
        board = new_board;
        new_board = [[false; 100]; 50];

    }
}

