fn is_palindrome(num: u64) -> bool {
    let s = num.to_string();
    for (i1, i2) in s.chars().zip(s.chars().rev()) {
        if i1 != i2 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut largest = 0;
    for i in 100..999 {
        for j in 100..999 {
            if is_palindrome(i * j) && ((i * j) > largest) {
                largest = i * j;
                println!("Largest: {}, {} x {}", largest, i, j);
            }
        }
    }
}