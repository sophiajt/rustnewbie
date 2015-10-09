fn main() {
    let mut sum = 0;
    const SIZE: usize = 2000000;
    let mut slots: [bool; SIZE] = [true; SIZE];
    slots[0] = false;
    slots[1] = false;
    
    // We calculate the primes using a simple stride and marking off multiples 
    for stride in 2..(SIZE/2) {
        let mut pos = stride;
        while pos < (SIZE - stride) {
            pos += stride;
            slots[pos] = false;
        }
    }
    
    for (idx, pr) in slots.into_iter().enumerate() {
        if *pr { sum += idx; }
    }

    println!("Sum: {}", sum);
}
