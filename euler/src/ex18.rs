use std::cmp;

fn main() {
    let input = 
       "75\n\
        95 64\n\
        17 47 82\n\
        18 35 87 10\n\
        20 04 82 47 65\n\
        19 01 23 75 03 34\n\
        88 02 77 73 07 63 67\n\
        99 65 04 28 06 16 70 92\n\
        41 41 26 56 83 40 80 70 33\n\
        41 48 72 33 47 32 37 16 94 29\n\
        53 71 44 65 25 43 91 52 97 51 14\n\
        70 11 33 28 77 73 17 78 39 68 17 57\n\
        91 71 52 38 17 14 91 43 58 50 27 29 48\n\
        63 66 04 68 89 53 67 30 73 16 69 87 40 31\n\
        04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";
        
     let mut triangle = input.split('\n')
        .map(|s| s.split_whitespace().map(|x| u64::from_str_radix(x, 10).unwrap())
        .collect::<Vec<u64>>()).collect::<Vec<Vec<u64>>>();
     
     let mut length = triangle.len();
     let mut cursor = length - 2;
     
     loop {
        for i in 0..(length-1) {
            triangle[cursor][i] += cmp::max(triangle[cursor+1][i], triangle[cursor+1][i+1]);
        }
        if cursor == 0 {
            break;
        } else {
            cursor -= 1;
        }
        length -= 1;
     }
     
     println!("{}", triangle[0][0]);
}
