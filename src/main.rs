fn next_epoch(mut board : Vec<Vec<usize>>, size : usize)->Vec<Vec<usize>> {
    let mut count = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            if i != 0 {
                count[i][j] += board[i-1][j];
            }
            if j != 0 {
                count[i][j] += board[i][j-1];
            }
            if i != size-1 {
                count[i][j] += board[i+1][j];
            }
            if i != size-1 {
                count[i][j] += board[i+1][j];
            }
            if i != 0 && j != 0 {
                count[i][j] += board[i-1][j-1];
            }
            if i != 0 && j != size-1 {
                count[i][j] += board[i-1][j+1];
            }
            if i != size-1 && j != size-1 {
                count[i][j] += board[i+1][j+1];
            }
            if i != size-1 && j != 0 {
                count[i][j] += board[i+1][j-1];
            }
        }
    }

    for i in 0..size {
        for j in 0..size {
            if board[i][j] == 0 && count[i][j] == 3 {
                board[i][j] = 1;
            }
            if board[i][j] == 1 && !(count[i][j] == 2 || count[i][j] == 3) {
                board[i][j] = 0;
            }
        }
    }

    board
}

fn print_board(board : &Vec<Vec<usize>>, size : usize, epoch : u32) {
    print!("\n{} generation\n", epoch);
    for i in 0..size {
        for j in 0..size {
            print!("{} ", board[i][j]);
        }
        print!("\n");
    }
}

fn main() {
    let size : usize = 5;
    let mut board = vec![vec![0; size]; size];
    board[0][0] = 1;
    board[2][2] = 1;
    board[1][2] = 1;
    board[3][2] = 1;
    print_board(&board, size, 0);
    for i in 1..5 {
        board = next_epoch(board, size);
        print_board(&board, size, i);
    }
}