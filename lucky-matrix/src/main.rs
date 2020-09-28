fn main() {
    println!("Hello, world!");
}
pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut min_in_rows: Vec<i32> = Vec::new();
        for row in matrix.iter() {
            let mut current_min: i32 = row[0];
            for n in row.iter() {
                if *n < current_min { current_min = *n; }
            }
            min_in_rows.push(current_min);
        }
        
        let mut max_in_cols: Vec<i32> = Vec::new();
        for i in 0..matrix[0].len() {
            let mut current_max: i32 = matrix[0][i];
            for j in 0..matrix.len() {
                if matrix[j][i] > current_max { current_max = matrix[j][i]; }
            }
            max_in_cols.push(current_max);
        }
        
        min_in_rows.into_iter().filter(|x| max_in_cols.contains(x)).collect()
    }
