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
    let mut num:u64 = 600851475143;
    let mut highest_prime_factor = 0;
    
    for i in primes() {
        if num % i == 0 {
            highest_prime_factor = i;
        }
        while (num % i == 0) && (num >= 2) {
            num /= i;
        }
        if num == 1 {
            break;
        }
    }
    println!("num: {}", highest_prime_factor);
}
