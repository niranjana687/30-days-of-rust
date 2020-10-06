fn main() {
    let arr = vec![vec![2,3,4], vec![5, 6,7]];

    let current = &arr[1][1];

    println!("{:p}", current);
}

fn min_path_sum(grid: Vec<Vec<i32>>)  {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut current = &grid[0][0];
        let mut sum = grid[0][0];

        for i in 0..rows {
            for j in 0..cols {
                
            }
        }
}
