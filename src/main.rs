use std::io::{self, Write};
use std::vec::Vec;

#[derive(Debug)]
enum MatrixError {
    EmptyMatrix,
    DimensionMismatch,
    InvalidInput,
}

fn main() {
    let mut current_matrix: Option<Vec<Vec<i32>>> = None;
    
    loop {
        print_menu(&current_matrix);
        
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().to_lowercase();
        
        match choice.as_str() {
            "n" => {
                let (rows, cols) = get_matrix_dimensions();
                current_matrix = Some(get_matrix_values(rows, cols));
                println!("\nCurrent matrix:");
                print_matrix(current_matrix.as_ref().unwrap());
            },
            "a" => {
                if let Some(matrix) = &current_matrix {
                    match add_matrices(matrix) {
                        Ok(result) => {
                            println!("\nResult of addition:");
                            print_matrix(&result);
                        },
                        Err(e) => print_error("addition", e),
                    }
                } else {
                    println!("Please create a matrix first using 'n'");
                }
            },
            "s" => {
                if let Some(matrix) = &current_matrix {
                    match subtract_matrices(matrix) {
                        Ok(result) => {
                            println!("\nResult of subtraction:");
                            print_matrix(&result);
                        },
                        Err(e) => print_error("subtraction", e),
                    }
                } else {
                    println!("Please create a matrix first using 'n'");
                }
            },
            "m" => {
                if let Some(matrix) = &current_matrix {
                    match multiply_matrices(matrix) {
                        Ok(result) => {
                            println!("\nResult of multiplication:");
                            print_matrix(&result);
                        },
                        Err(e) => print_error("multiplication", e),
                    }
                } else {
                    println!("Please create a matrix first using 'n'");
                }
            },
            "q" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice. Please try again."),
        }
        
        println!("\nPress Enter to continue...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}

fn print_menu(current_matrix: &Option<Vec<Vec<i32>>>) {
    println!("\n=== Matrix Calculator ===");
    if let Some(matrix) = current_matrix {
        println!("\nCurrent matrix:");
        print_matrix(matrix);
    } else {
        println!("\nNo matrix currently loaded.");
    }
    println!("\nOperations:");
    println!("n - Create new matrix");
    println!("a - Add matrices");
    println!("s - Subtract matrices");
    println!("m - Multiply matrices");
    println!("q - Quit program");
}

fn print_error(operation: &str, error: MatrixError) {
    match error {
        MatrixError::EmptyMatrix => {
            println!("Error: Cannot perform {} with empty matrices!", operation);
        },
        MatrixError::DimensionMismatch => {
            println!("Error: Matrix dimensions don't match for {}!", operation);
        },
        MatrixError::InvalidInput => {
            println!("Error: Invalid input provided for {}!", operation);
        }
    }
}

fn get_matrix_dimensions() -> (usize, usize) {
    let mut rows = 0;
    let mut cols = 0;
    
    while rows == 0 {
        print!("Enter number of rows: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(n) = input.trim().parse() {
            if n > 0 {
                rows = n;
            } else {
                println!("Please enter a positive number");
            }
        } else {
            println!("Please enter a valid number");
        }
    }
    
    while cols == 0 {
        print!("Enter number of columns: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(n) = input.trim().parse() {
            if n > 0 {
                cols = n;
            } else {
                println!("Please enter a positive number");
            }
        } else {
            println!("Please enter a valid number");
        }
    }
    
    (rows, cols)
}

fn get_matrix_values(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut matrix = Vec::with_capacity(rows);
    
    println!("\nEnter matrix values row by row:");
    println!("(separate values with spaces, press enter after each row)");
    
    for i in 0..rows {
        let mut row = Vec::new();
        loop {
            print!("Row {} (enter {} values): ", i + 1, cols);
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            let values: Vec<i32> = input
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            
            if values.len() == cols {
                row = values;
                break;
            } else {
                println!("Please enter exactly {} numbers", cols);
            }
        }
        matrix.push(row);
    }
    
    matrix
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for val in row {
            print!("{:4}", val);
        }
        println!();
    }
}

fn add_matrices(first: &Vec<Vec<i32>>) -> Result<Vec<Vec<i32>>, MatrixError> {
    let rows = first.len();
    if rows == 0 { 
        return Err(MatrixError::EmptyMatrix); 
    }
    
    let cols = first[0].len();
    
    if !first.iter().all(|row| row.len() == cols) {
        return Err(MatrixError::DimensionMismatch);
    }
    
    println!("\nEnter the second matrix to add ({} x {}):", rows, cols);
    let second = get_matrix_values(rows, cols);
    
    if second.len() != rows || second.iter().any(|row| row.len() != cols) {
        return Err(MatrixError::DimensionMismatch);
    }
    
    let mut result = Vec::with_capacity(rows);
    for i in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for j in 0..cols {
            row.push(first[i][j] + second[i][j]);
        }
        result.push(row);
    }
    
    Ok(result)
}

fn subtract_matrices(first: &Vec<Vec<i32>>) -> Result<Vec<Vec<i32>>, MatrixError> {
    let rows = first.len();
    if rows == 0 { 
        return Err(MatrixError::EmptyMatrix); 
    }
    
    let cols = first[0].len();
    
    if !first.iter().all(|row| row.len() == cols) {
        return Err(MatrixError::DimensionMismatch);
    }
    
    println!("\nEnter the second matrix to subtract ({} x {}):", rows, cols);
    let second = get_matrix_values(rows, cols);
    
    if second.len() != rows || second.iter().any(|row| row.len() != cols) {
        return Err(MatrixError::DimensionMismatch);
    }
    
    let mut result = Vec::with_capacity(rows);
    for i in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for j in 0..cols {
            row.push(first[i][j] - second[i][j]);
        }
        result.push(row);
    }
    
    Ok(result)
}


fn multiply_matrices(first: &Vec<Vec<i32>>) -> Result<Vec<Vec<i32>>, MatrixError> {
    let rows_a = first.len();
    if rows_a == 0 { 
        return Err(MatrixError::EmptyMatrix); 
    }
    
    let cols_a = first[0].len();
    
    if !first.iter().all(|row| row.len() == cols_a) {
        return Err(MatrixError::DimensionMismatch);
    }
    
    println!("\nFor matrix multiplication:");
    println!("First matrix is {} x {}", rows_a, cols_a);
    println!("Second matrix must be {} x n", cols_a);
    print!("Enter number of columns for second matrix: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cols_b = input.trim().parse::<usize>().unwrap_or(0);
    
    if cols_b == 0 {
        return Err(MatrixError::InvalidInput);
    }
    
    println!("\nEnter the second matrix ({} x {}):", cols_a, cols_b);
    let second = get_matrix_values(cols_a, cols_b);
    
    if second.len() != cols_a || second.iter().any(|row| row.len() != cols_b) {
        return Err(MatrixError::DimensionMismatch);
    }
    
    let mut result = vec![vec![0; cols_b]; rows_a];
    
    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                result[i][j] += first[i][k] * second[k][j];
            }
        }
    }
    
    Ok(result)
}