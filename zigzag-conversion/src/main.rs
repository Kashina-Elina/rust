struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let max_of_set = (num_rows-1)*2;
        let max_two = num_rows-1;
        let mut string = String::new();
        if num_rows == 1 {
            return s;
        }
        else {
            for i in (0..s.len() as i32).step_by(max_of_set as usize){
                string.push(s.chars().nth(i as usize).expect("error"));
            }
            for j in (1..max_two as usize) {
                let mut down = true;
                let mut i=j;
                while i < s.len() {
                    string.push(s.chars().nth(i as usize).expect("error"));
                    if down {
                        i+= ((num_rows -(j as i32)-1)*2) as usize;
                    }
                    else {
                        i+=((num_rows-1)*2 - (num_rows - (j as i32)-1)*2) as usize;
                    }
                    down = !down;
                }
            }
            println!("s: {s}");
            for k in ((num_rows-1)..s.len() as i32).step_by(((num_rows-1)*2) as usize){
                string.push(s.chars().nth(k as usize).expect("error"));
            }
            string
    }
}
}

fn main() {
    let s1 = String::from("PAYPALISHIRING");
    let result = Solution::convert(s1, 4);
    println!("{}", result); 
}
