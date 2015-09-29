fn main() {
    let mut sum_of_squares:u64 = 0;
    let mut square_of_sums:u64 = 0;
    for i in 1..101u64 {
        sum_of_squares += i.pow(2);;
        square_of_sums += i;
    }
    square_of_sums = square_of_sums.pow(2);
    
    println!("{}", square_of_sums - sum_of_squares);
}