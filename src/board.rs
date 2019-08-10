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

#[cfg(test)]
mod tests {
    use crate::*;

    use super::*;

    #[test]
    fn test_get_board_layer() {
        assert_eq!(get_board_layer((2,2), 0), vec![
            (0,0),
            (0,1),
            (1,1),
            (1,0)
        ]);

        assert_eq!(get_board_layer((1,1), 0), vec![(0,0)]);
        assert_eq!(get_board_layer((4,3), 1), vec![(1,1),(2,1)]);

    }
}