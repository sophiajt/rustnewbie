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
        //let new_next = self.curr + self.next;
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
    let mut num = 0u64;
    
    // Cache the primes we'll be using
    let mut first_primes: Vec<u64> = primes().take(1000).collect();
    
    for i in 1..1000000u64 {
        num += i;
        
        let mut num_div = 1u64;
        
        let mut num_tmp = num;
        for x in &mut first_primes {
            let prime = *x;
            if (num % prime) == 0 {
                let mut exponent = 1;

                while (num_tmp % prime) == 0 { 
                    exponent += 1;
                    num_tmp /= prime;
                }
                num_div *= exponent;
            }
            if num_tmp == 1 { break; }
        }
        
        if num_div > 500 {
            println!("num: {}", num);
            return;
        }
    }
}