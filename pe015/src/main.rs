use std::time::SystemTime;

const WIDTH: usize = 20;
const HEIGHT: usize = 20;

fn main() {
    let time = SystemTime::now();

    let answer = nroutes(WIDTH, HEIGHT);

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

/// Calculates the number of possible routes from top-left to bottom-right in a `w` x `h` grid
fn nroutes(mut w: usize, mut h: usize) -> usize {
    // each node in the grid stores the number of routes going from it
    let mut grid = vec![vec![1; w + 1]; h + 1];

    // last col and last row all have 1 possible route per node
    // then fill last unfilled col and last unfilled row starting with the corner
    // number of routes in a node is the sum of routes from bottom and right nodes
    while w != 0 && h != 0 {
        // update current last unfilled corner
        if h > 0 {
            h -= 1;
        }

        if w > 0 {
            w -= 1;
        }

        // fill last unfilled col, from the node on top of the corner to the top
        for row in (0..=h).rev() {
            grid[row][w] = grid[row + 1][w] + grid[row][w + 1];
        }

        // fill last unfilled row, from the corner to the left
        if w > 0 {
            for col in (0..=w - 1).rev() {
                grid[h][col] = grid[h + 1][col] + grid[h][col + 1];
            }
        }
    }

    grid[0][0]
}

#[cfg(test)]
mod tests {
    use crate::nroutes;

    #[test]
    fn test_nroutes() {
        assert_eq!(nroutes(2, 2), 6);
        assert_eq!(nroutes(4, 3), 35);
    }
}
