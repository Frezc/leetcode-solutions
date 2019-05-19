/**
 * [79] Word Search
 *
 * Given a 2D board and a word, find if the word exists in the grid.
 * 
 * The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.
 * 
 * Example:
 * 
 * 
 * board =
 * [
 *   ['A','B','C','E'],
 *   ['S','F','C','S'],
 *   ['A','D','E','E']
 * ]
 * 
 * Given word = "ABCCED", return true.
 * Given word = "SEE", return true.
 * Given word = "ABCB", return false.
 * 
 * 
 */
/// backtracing
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.len() == 0 || board.first().map_or(0, |v|v.len()) == 0 || word.len() == 0 {
            return false;
        }
        let chars: Vec<char> = word.chars().collect();
        let mut start_indices = vec![];
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == chars[0] {
                    start_indices.push(i * board[i].len() + j);
                }
            }
        }
        for start_index in start_indices {
            if exist_part(&board, &chars, &mut vec![start_index]) {
                return true;
            }
        }
        false
    }
}

fn exist_part(board: &Vec<Vec<char>>, word: &Vec<char>, steps: &mut Vec<usize>) -> bool {
    if word.len() == steps.len() {
        return true;
    }

    let rows = board.len();
    let columns = board[0].len();
    let pos = steps.last().unwrap_or(&0).to_owned();

    // check up
    if pos >= columns && steps.iter().find(|&&p| p == pos - columns).is_none() {
        let next_pos = pos - columns;
        if board[next_pos/columns][next_pos%columns] == word[steps.len()] {
            steps.push(next_pos);
            if exist_part(board, word, steps) {
                return true;
            }
            steps.pop();
        }
    }

    if pos % columns != columns - 1 && steps.iter().find(|&&p| p == pos + 1).is_none() {
        let next_pos = pos + 1;
        if board[next_pos/columns][next_pos%columns] == word[steps.len()] {
            steps.push(next_pos);
            if exist_part(board, word, steps) {
                return true;
            }
            steps.pop();
        }
    }

    if pos / columns != rows - 1 && steps.iter().find(|&&p| p == pos + columns).is_none() {
        let next_pos = pos + columns;
        if board[next_pos/columns][next_pos%columns] == word[steps.len()] {
            steps.push(next_pos);
            if exist_part(board, word, steps) {
                return true;
            }
            steps.pop();
        }
    }

    if pos % columns != 0 && steps.iter().find(|&&p| p == pos - 1).is_none() {
        let next_pos = pos - 1;
        if board[next_pos/columns][next_pos%columns] == word[steps.len()] {
            steps.push(next_pos);
            if exist_part(board, word, steps) {
                return true;
            }
            steps.pop();
        }
    }

    false
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_79() {
        assert_eq!(Solution::exist(vec2!
        [
          ['A','B','C','E'],
          ['S','F','C','S'],
          ['A','D','E','E']
        ]
        , "ABCCED".to_string()), true);
        assert_eq!(Solution::exist(vec2!
        [
          ['A','B','C','E'],
          ['S','F','C','S'],
          ['A','D','E','E']
        ]
        , "SEE".to_string()), true);
        assert_eq!(Solution::exist(vec2!
        [
          ['A','B','C','E'],
          ['S','F','C','S'],
          ['A','D','E','E']
        ]
        , "ABCB".to_string()), false);
    }
}
