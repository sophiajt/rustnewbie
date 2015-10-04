//Wow, finding divisors this way is soooo slow
fn num_divisors(num: u64) -> u64 {
    let mut total = 1u64;
    
    for x in 2..(num+1) {
        if (num % x) == 0 { total += 1; }
    }
    
    return total;
}

fn main() {
    let mut num = 0u64;
    for i in 1..1000000u64 {
        num += i;
        let num_div = num_divisors(num);
        if num_div > 100 {
            println!("num_div: {} (from {}:{})", num_div, i, num);
            if num_div > 500 {
                println!("num: {}", num);
                return;
            }
        }
    }
}