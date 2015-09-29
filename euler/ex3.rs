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

/*
fn highest_prime_divisor(num: u64) -> u64 {
    let mut highest:u64 = 0;
    for i in 2..(num/2 + 1) {
        if (num % i == 0) && is_prime(i) {
            println!("highest so far: {}", i);
            highest = i;
        }
    }
    
    return highest;    
}

fn main() {
    let num:u64 = 600851475143;
    let top_bound = num / 2 + 1;
    let mut highest:u64 = 0;
    
    //for i in primes().take_while(|&x| x <= (num / 2)) {
    for i in 2..top_bound{
        if i % 1000000000 == 0 {
            println!("at: {}", i);
        }
        if (num % i == 0) && is_prime(i) {
            println!("highest so far: {}", i);
            highest = i;
        }
    }
    println!("{}", highest);
}
*/
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