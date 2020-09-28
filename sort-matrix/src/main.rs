fn main() {
    println!("Hello, world!");
}


fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    
    let mut count = 0;
    
    for row in 0..rows {
        // check if remainder of matrix is all-negative
        if grid[row][0] < 0 {
            count += (rows - row) * cols;
            break;
        }
        
        for col in 0..cols {
            // check if remainder of row is all-negative
            if grid[row][col] < 0 {
                count += cols - col;
                break;
            }
        }
    }
    
    count as i32
}
