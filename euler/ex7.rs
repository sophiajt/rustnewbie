fn is_prime(num:u64) -> bool {
    for i in 2..(num / 2 + 1) {
        if num % i == 0 {
            return false;
        }
    }
    return true; 
}    

struct Prime {
    curr: u64,
}

// Implement 'Iterator' for 'Fibonacci'
impl Iterator for Prime {
    type Item = u64;
    // The 'Iterator' trait only requires the 'next' method to be defined. The
    // return type is 'Option<T>', 'None' is returned when the 'Iterator' is
    // over, otherwise the next value is returned wrapped in 'Some'
    fn next(&mut self) -> Option<u64> {
        //let new_next = self.curr + self.next;
        let mut new_next = self.curr + 1;
        while !is_prime(new_next) {
            new_next += 1;
        }

        self.curr = new_next;

        // 'Some' is always returned, this is an infinite value generator
        Some(self.curr)
    }
}


// Returns the primes
fn primes() -> Prime {
    Prime { curr: 1 }
}

fn main() {
    let mut pos = 0;
    for num in primes() {
        pos += 1;
        if pos == 10001 {
            println!("prime: {}", num);
            return;
        }
    }
}