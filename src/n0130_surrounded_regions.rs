/**
 * [130] Surrounded Regions
 *
 * Given a 2D board containing 'X' and 'O' (the letter O), capture all regions surrounded by 'X'.
 * 
 * A region is captured by flipping all 'O's into 'X's in that surrounded region.
 * 
 * Example:
 * 
 * 
 * X X X X
 * X O O X
 * X X O X
 * X O X X
 * 
 * 
 * After running your function, the board should be:
 * 
 * 
 * X X X X
 * X X X X
 * X X X X
 * X O X X
 * 
 * 
 * Explanation:
 * 
 * Surrounded regions shouldn&rsquo;t be on the border, which means that any 'O' on the border of the board are not flipped to 'X'. Any 'O' that is not on the border and it is not connected to an 'O' on the border will be flipped to 'X'. Two cells are connected if they are adjacent cells connected horizontally or vertically.
 * 
 */
/// Find all 'O' in the border layer, and mark all adjacent O in the board.
/// For example:
/// X X X X         1 1 1 1
/// X O O X    =>   1 0 0 1
/// X X O X         1 1 0 1
/// X O X X         1 2 1 1
/// Here 0 present unreachable location, 1 present X and 2 present not surrounded O.
///
/// Then set all location to X if not 2.
///
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.len() <= 2 || board[0].len() <= 2 {
            return;
        }

        let rows = board.len();
        let columns = board[0].len();

        // 0 for unchecked, 1 for 'X', 2 for not surrounded 'O'
        let mut mask = vec![vec![0; columns]; rows];
        let border_layer = get_board_layer((rows, columns), 0);

        // check border
        for loc in &border_layer {
            if board[loc.0][loc.1] == 'O' {
                mask[loc.0][loc.1] = 2;
            } else {
                mask[loc.0][loc.1] = 1;
            }
        }

        // fill mask
        for loc in &border_layer {
            // skip corner
            if !(loc.0 > 0 && loc.0 < rows - 1 || loc.1 > 0 && loc.1 < columns - 1) {
                continue;
            }

            if mask[loc.0][loc.1] == 2 {
                let adjacent = match *loc {
                    (0, c) => (1usize, c),
                    (r, c) if r == rows - 1 => (r - 1, c),
                    (r, 0) => (r, 1),
                    (r, c) if c == columns - 1 => (r, c - 1),
                    _ => panic!("cannot find adjacent")
                };
                connect_not_surrounded(board, &mut mask, &adjacent);
            }
        }

        // apply mask
        for r in 0..board.len() {
            for c in 0..board[r].len() {
                if mask[r][c] != 2 {
                    board[r][c] = 'X';
                }
            }
        }
    }
}

fn connect_not_surrounded(board: &mut Vec<Vec<char>>, mask: &mut Vec<Vec<i32>>, loc: &(usize, usize)) {
    if mask[loc.0][loc.1] != 0 {
        return;
    }

    if board[loc.0][loc.1] == 'O' {
        mask[loc.0][loc.1] = 2;

        // check adjacent location
        connect_not_surrounded(board, mask, &(loc.0 - 1, loc.1));
        connect_not_surrounded(board, mask, &(loc.0 + 1, loc.1));
        connect_not_surrounded(board, mask, &(loc.0, loc.1 - 1));
        connect_not_surrounded(board, mask, &(loc.0, loc.1 + 1));
    } else {
        mask[loc.0][loc.1] = 1;
    }
}

/// get locations of specific layer of board with size rows * columns
/// layer start from 0
fn get_board_layer(size: (usize, usize), layer: usize) -> Vec<(usize, usize)> {
    if size.0 == 0 || size.1 == 0 {
        panic!("rows and columns cannot be 0");
    }
    let (rows, columns) = size;

    if layer > (rows.min(columns) - 1) / 2 {
        panic!("layer cannot larger than {}", (rows.min(columns) + 1) / 2);
    }


    let size = (rows - 2 * layer) * (columns - 2 * layer);
    let mut result = Vec::with_capacity(size);

    if size == 1 {
        result.push((layer, layer));
        return result;
    }

    // LT -> RT
    for i in layer..columns - layer - 1 {
        result.push((layer, i));
    }

    // RT -> RB
    for i in layer..rows - layer - 1 {
        result.push((i, columns - layer - 1));
    }

    // RB -> LB
    for i in (layer + 1..columns - layer).rev() {
        result.push((rows - layer - 1, i));
    }

    // LB -> LT
    for i in (layer + 1..rows - layer).rev() {
        result.push((i, layer));
    }

    result
}

// submission codes end

#[cfg(test)]
mod tests {
    use crate::*;

    use super::*;

    #[test]
    fn test_130() {
        let mut board = vec2![
            ['X', 'X', 'X', 'X'],
            ['X', 'O', 'O', 'X'],
            ['X', 'X', 'O', 'X'],
            ['X', 'O', 'X', 'X']
        ];
        Solution::solve(&mut board);
        assert_eq!(board, vec2![
            ['X', 'X', 'X', 'X'],
            ['X', 'X', 'X', 'X'],
            ['X', 'X', 'X', 'X'],
            ['X', 'O', 'X', 'X']
        ]);

        board = vec2![
            ['X', 'X', 'X', 'X'],
            ['X', 'O', 'O', 'X'],
            ['X', 'O', 'X', 'X'],
            ['X', 'O', 'X', 'X']
        ];
        Solution::solve(&mut board);
        assert_eq!(board, vec2![
            ['X', 'X', 'X', 'X'],
            ['X', 'O', 'O', 'X'],
            ['X', 'O', 'X', 'X'],
            ['X', 'O', 'X', 'X']
        ]);
    }
}
