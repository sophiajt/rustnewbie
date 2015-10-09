fn main() {
    let mut digits: Vec<u8> = vec![1];

    for _ in 0..1000 {
        let mut carry = 0;
        for digit_index in 0..digits.len() {
            let new_digit = digits[digit_index] * 2 + carry;
            digits[digit_index] = new_digit % 10;
            carry = new_digit / 10;
        }
        if carry > 0 {
            digits.push(carry);
        }
    }

    let mut sum = 0u64;
    for digit in digits {
        sum += digit as u64;
    }

    println!("{}", sum);
}
