fn main() {
    println!("Hello, world!");
}


fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] < 0 {
                count += 1;
            }
        }
    }
    
    count 
}
