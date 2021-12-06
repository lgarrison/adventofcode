use std::fs;

fn make_balls(line: &str) -> Vec<i32> {
    return line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
}

fn make_board(lines: &str) -> Vec<Vec<i32>> {
    //let board = [[i32; BSZ]; BSZ];
    let rows = lines.split("\n");
    let cols: Vec<Vec<i32>> = rows.map(|r| r.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect()).collect();
    return cols;
}

fn check_board(marks: &Vec<Vec<bool>>) -> bool {
    for row in marks {
        if row.iter().all(|v| *v) {
            return true;
        }
    }
    
    for j in 0..marks[0].len() {
        let mut win = true;
        for i in 0..marks.len() {
            win &= marks[i][j];
        }
        if win {
            return true;
        }
    }
    
    return false;
}

fn score_board(board: &Vec<Vec<i32>>, marks: &Vec<Vec<bool>>, last: i32) -> i32 {
    let mut sum = 0;
    for i in 0..board.len() {
        for j in 0..board.len() {
            if !marks[i][j] {
                sum += board[i][j];
            }
        }
    }
    
    return sum*last;
}

fn main() {
    let path = "input.txt";
    let _lines = fs::read_to_string(path).unwrap();
    let mut lines = _lines.split("\n\n");
    
    let balls = make_balls(lines.next().unwrap());
    let boards: Vec<Vec<Vec<i32>>> = lines.map(make_board).collect();
    
    let mut marks = vec![vec![vec![false;5];5];boards.len()];
 
    let mut win = None;
    let mut winningball = 0;
    'outer: for ball in &balls {
        for (i,board) in boards.iter().enumerate() {
            for (j,row) in board.iter().enumerate() {
                for (k,col) in row.iter().enumerate() {
                    if ball == col {
                        marks[i][j][k] = true;
                    }
                }
            }
            
            if check_board(&marks[i]) {
                win = Some(i);
                winningball = *ball;
                break 'outer;
            }
        }
    }
    
    let score = score_board(&boards[win.unwrap()], &marks[win.unwrap()], winningball);
    
    println!("Winner: board {}, score {}, ball {}", win.unwrap(), score, winningball);
}
