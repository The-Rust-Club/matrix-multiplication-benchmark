use matrix_multiplication_benchmark::Matrix;
fn main() {
    // do something like this to start, or just read from input
    let a = Matrix::random(1000,2);
    let b = Matrix::random(1000,2);
    let c = a * b;
    println!("{:?}", c);
}
