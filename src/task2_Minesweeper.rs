// 题目链接：https://exercism.org/tracks/rust/exercises/minesweeper

fn minesweeper(r: usize, c: usize, mine_index: Vec<[i32; 2]>, result_board: &mut Vec<Vec<i32>>) {
    let mut tmp: [i32; 2];
    for i in 0..r {
        for j in 0..c {
            tmp = [i as i32, j as i32];
            if mine_index.iter().any(|&x| x == tmp) {
                result_board[i][j] = -1;
            } else {
                // left
                if j > 0 {
                    tmp = [i as i32, (j - 1) as i32];
                    if mine_index.iter().any(|&x| x == tmp) {
                        result_board[i][j] = result_board[i][j] + 1;
                    }
                    // left above
                    if i > 0 {
                        tmp = [(i - 1) as i32, (j - 1) as i32];
                        if mine_index.iter().any(|&x| x == tmp) {
                        result_board[i][j] = result_board[i][j] + 1;
                        }
                    }
                    // left below
                    if i < r - 1 {
                        tmp = [(i + 1) as i32, (j - 1) as i32];
                        if mine_index.iter().any(|&x| x == tmp) {
                            result_board[i][j] = result_board[i][j] + 1;
                        }
                    }
                }
                // right
                if j < c - 1 {
                    tmp = [i as i32, (j + 1) as i32];
                    if mine_index.iter().any(|&x| x == tmp) {
                        result_board[i][j] = result_board[i][j] + 1;
                    }
                    // right above
                    if i > 0 {
                        tmp = [(i - 1) as i32, (j + 1) as i32];
                        if mine_index.iter().any(|&x| x == tmp) {
                        result_board[i][j] = result_board[i][j] + 1;
                        }
                    }
                    // right below
                    if i < r - 1 {
                        tmp = [(i + 1) as i32, (j + 1) as i32];
                        if mine_index.iter().any(|&x| x == tmp) {
                            result_board[i][j] = result_board[i][j] + 1;
                        }
                    }
                }
                // above
                if i > 0 {
                    tmp = [(i - 1) as i32, j as i32];
                    if mine_index.iter().any(|&x| x == tmp) {
                    result_board[i][j] = result_board[i][j] + 1;
                    }
                }
                // below
                if i < r - 1 {
                    tmp = [(i + 1) as i32, j as i32];
                    if mine_index.iter().any(|&x| x == tmp) {
                        result_board[i][j] = result_board[i][j] + 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minesweeper() {
        // the index of mine in the borad
        let mine_index: Vec<[i32; 2]> = vec![
            [0, 1], [0, 3],
            [1, 2],
            [2, 2]
            ];
        
        // use -1 to represent the mine
        let board: Vec<Vec<i32>> = vec![
            vec![1, -1, 3, -1, 1],
            vec![1, 3, -1, 3, 1],
            vec![0, 2, -1, 2, 0],
            vec![0, 1, 1, 1, 0]
        ];
        let mut result_board: Vec<Vec<i32>> = vec![
            vec![0; 5],
            vec![0; 5],
            vec![0; 5],
            vec![0; 5]
        ];
        minesweeper(4, 5, mine_index, &mut result_board);
        assert_eq!(board, result_board);
    }
}