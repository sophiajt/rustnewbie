fn main() {
    let max: u64 = 20*19*18*17*16*15*14*13*12*11;
    for num in 1..max {
        if (num % 20 == 0) &&
           (num % 19 == 0) &&
           (num % 18 == 0) &&
           (num % 17 == 0) &&
           (num % 16 == 0) &&
           (num % 15 == 0) &&
           (num % 14 == 0) &&
           (num % 13 == 0) &&
           (num % 12 == 0) &&
           (num % 11 == 0) {
           
           println!("num: {}", num);
           return;
        }
    }
}
