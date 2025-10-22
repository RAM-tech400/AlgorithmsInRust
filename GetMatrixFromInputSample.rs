use std::io;

macro_rules! parse_input {
    ($x: expr, $t: ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let matrix = get_matrix_from_input();
    println!("{:#?}", matrix);
}

fn get_matrix_from_input() -> Vec<Vec<i32>> {
    let mut input_line = String::new();
    println!("Enter the matrix rows and columns count (Separated by space):\t");
    io::stdin().read_line(&mut input_line).unwrap();
    let given_rows_cols : Vec<_> = input_line.split_whitespace().collect();
    let max_rows = parse_input!(given_rows_cols[0], usize);
    let max_cols = parse_input!(given_rows_cols[1], usize);
    let mut matrix : Vec<Vec<i32>> = vec![vec![0; max_cols]; max_rows];
    println!("Enter your matrix (columns separated by one space and rows separated by newline):");
    for i in 0..max_rows {
        input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        matrix[i] = input_line.split_whitespace().map(|s| parse_input!(s, i32)).collect::<Vec<i32>>();
    }
    return matrix;
}

