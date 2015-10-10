fn number_as_name(x: usize) -> String {
    let early_nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", 
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

    let mut num = x;

    if num == 1000 {
        return early_nums[0].to_string() + "thousand";
    }

    let mut result : String = "".to_string();

    if num >= 100 {
        let hundreds_digit = num / 100;
        result.push_str(early_nums[hundreds_digit - 1]);
        result.push_str("hundred");
        num = num % 100;

        if num > 0 {
            result.push_str("and");
        }
    }

    if num >= 20 {
        let tens_digit = num / 10;
        result.push_str(tens[tens_digit - 2]);
        num = num % 10;
    }

    if num > 0 {
        result.push_str(early_nums[num - 1]);
    }

    return result;
} 
    
fn main() {
    // a few test cases
    // println!("{}", number_as_name(100));
    // println!("{}", number_as_name(4));
    // println!("{}", number_as_name(17));
    // println!("{}", number_as_name(36));
    // println!("{}", number_as_name(342));
    // println!("{}", number_as_name(1000));

    let mut sum = 0;
    for x in 1..1001 {
        sum += number_as_name(x).len();
    }

    println!("{}", sum);
}
