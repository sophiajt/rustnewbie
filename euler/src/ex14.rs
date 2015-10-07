fn collatz(x: u64) -> u64 {
    let mut total:u64 = x;
    let mut num_steps:u64 = 0;
    
    while total != 1 {
        if total % 2 == 0 {
            total /= 2;
        }
        else {
            total = total * 3 + 1;
        }
        num_steps += 1;
    }

    return num_steps;
}

fn main() {
    println!("Imperative");
    let mut top_collatz:u64 = 0;
    let mut top_steps:u64 = 0;
    
    for i in 1..1000001 {
        let num_steps = collatz(i);
        
        if num_steps > top_steps {
            top_collatz = i;
            top_steps = num_steps;
        }
    }
        
    println!("{}", top_collatz);
    
    println!("Compositional:");
    
    let max = (1..1000001).map(collatz).zip(1..1000001).max().unwrap().1;

    println!("{}", max);
}