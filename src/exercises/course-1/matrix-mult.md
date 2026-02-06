# Matrix multiplication

Your task is to write a function that performs matrix multiplication in Rust. The function signature should look like this:

```rs
fn multiply_matrices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Your code
}
```

Your code should check that the matrices have the right dimensions.

There are different ways to represent 2-D arrays in Rust, here we chose the type `&Vec<Vec<i32>>`. When you finish this exercise, feel free to explore other possible representations (e.g., `ndarray::arr2`).

Usage example:
```rs
fn main() {
    let matrix_a = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];

    let matrix_b = vec![
        vec![7, 8],
        vec![9, 10],
        vec![11, 12],
    ];

    let result_matrix = multiply_matrices(&matrix_a, &matrix_b);

    // Print the result
}
```
