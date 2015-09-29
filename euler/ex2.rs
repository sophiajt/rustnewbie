struct Fibonacci {
    curr: u64,
    next: u64,
}

// Implement 'Iterator' for 'Fibonacci'
impl Iterator for Fibonacci {
    type Item = u64;
    // The 'Iterator' trait only requires the 'next' method to be defined. The
    // return type is 'Option<T>', 'None' is returned when the 'Iterator' is
    // over, otherwise the next value is returned wrapped in 'Some'
    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // 'Some' is always returned, this is an infinite value generator
        Some(self.curr)
    }
}

// Returns a fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    let mut sum:u64 = 0;
    for i in fibonacci().take_while(|&x| x < 4000000).filter(|&x| x % 2 == 0) {
        sum += i;
    }
    println!("sum: {}", sum);
}