fn multiply_matrices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Check if matrix multiplication is possible
    assert_eq!(a[0].len(), b.len(), "Matrix multiplication not possible!");

    // Dimensions of matrices
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    // Initialize result matrix C with zeros
    let mut c = vec![vec![0; cols_b]; rows_a];

    // Perform matrix multiplication
    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for &element in row {
            print!("{:4} ", element);
        }
        println!();
    }
    println!();
}

fn main() {
    let a = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];

    let b = vec![
        vec![7, 8],
        vec![9, 10],
        vec![11, 12],
    ];

    // Call the matrix multiplication function
    let c = multiply_matrices(&a, &b);

    // Print the matrices and the result
    println!("Matrix A:");
    print_matrix(&a);

    println!("Matrix B:");
    print_matrix(&b);

    println!("Result Matrix C:");
    print_matrix(&c);
}
