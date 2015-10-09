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

impl Iterator for Prime {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let mut new_next = self.curr + 1;
        while !is_prime(new_next) {
            new_next += 1;
        }

        self.curr = new_next;

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
