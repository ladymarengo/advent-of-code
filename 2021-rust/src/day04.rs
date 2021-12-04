use std::fs::read_to_string;

fn parse_board() -> (Vec<String>, Vec<Vec<Vec<String>>>) {
    let input = read_to_string("input/04").unwrap().to_string();
    let parts: Vec<String> = input.split("\n\n").map(|p| p.to_string()).collect();
    let numbers: Vec<String> = parts[0].split(',').map(|i| i.to_string()).collect();
    let mut boards: Vec<Vec<Vec<String>>> = vec![];
    for i in 1..parts.len() {
        if parts[i].len() == 0 {
            continue;
        }
        let rows: Vec<String> = parts[i].lines().map(|l| l.to_string()).collect();
        let mut board: Vec<Vec<String>> = vec![];
        for row in rows {
            let mut row_arr: Vec<String> = vec![];
            let temp: Vec<String> = row.split(' ').map(|r| r.to_string()).collect(); 
            for number in temp {
                if number != "".to_string() {
                    row_arr.push(number);
                }
            }
            board.push(row_arr);
        }
        boards.push(board);
    }
    (numbers, boards)
}

fn part_one(numbers: &Vec<String>, boards: &mut Vec<Vec<Vec<String>>>) -> i32 {
    for number in numbers {
        for board in &mut *boards {
            let result = mark(board, number.to_string());
            if result != 0 {
                return result;
            }
        }
    }
    0
}

fn part_two(numbers: &Vec<String>, boards: &mut Vec<Vec<Vec<String>>>) -> i32 {
    for number in numbers {
        let mut new_boards: Vec<Vec<Vec<String>>> = vec![];
        for i in 0..boards.len() {
            let result = mark(&mut boards[i], number.to_string());
            if result != 0 {
                if boards.len() == 1 {
                    return calculate_win(&boards[0]) * number.parse::<i32>().unwrap();
                }
            } else {
                new_boards.push(boards[i].clone());
            }
        }
        *boards = new_boards;
    }
    0
}

fn mark(board: &mut Vec<Vec<String>>, number: String) -> i32{
    for row in 0..board.len() {
        for column in 0..board.len() {
            if board[row][column] == number {
                board[row][column] = "*".to_string();
                if check_win(&board, row, column) != 0 {
                    return calculate_win(&board) * number.parse::<i32>().unwrap();
                }
            }
        }
    }
    0
}

fn check_win(board: &Vec<Vec<String>>, row: usize, column: usize) -> i32 {
    let mut win_row: i32 = 1;
    let mut win_column: i32 = 1;
    for c in 0..board[0].len() {
        if board[row][c] != "*".to_string() {
            win_row = 0;
            break;
        }
    }
    for r in 0..board[0].len() {
        if board[r][column] != "*".to_string() {
            win_column = 0;
            break;
        }
    }
    if win_row == 1 || win_column == 1 {
        1
    } else {
        0
    }
}

fn calculate_win(board: &Vec<Vec<String>>) -> i32 {
    let mut sum: i32 = 0;
    for row in board {
        for number in row {
            if number != "*" {
                sum += number.parse::<i32>().unwrap();
            }
        }
    }
    sum
}

fn main() {
    let (numbers,  mut boards) = parse_board();
    let mut boards_two = boards.clone();
    println!("First answer is {}, second answer is {}", part_one(&numbers, &mut boards), part_two(&numbers, &mut boards_two));
}
