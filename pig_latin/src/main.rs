fn main() {
    let s = String::from("first apple note");
    let new_s: Vec<_> = s.split_whitespace().collect();
    pig_latin(new_s);
}

fn pig_latin(s: Vec<&str>) {
    let vowel = String::from("ayuioe");
    let mut rezult: Vec<_>= vec![];
    for word in s{
        let s;
        let len = word.len();
        let f_ch = &word[0 .. 1];
        if f_ch != &vowel[0..1] && f_ch != &vowel[1..2] && f_ch != &vowel[2..3]&& f_ch != &vowel[3..4] && f_ch != &vowel[4..5]&& f_ch != &vowel[5..6]{
            let first_char = &word[0..1];
            let new_word = &word[1 .. len];
            let s1 =  String::from("hay");
            s = new_word.to_owned()+&first_char+&s1;

        }
        else {
            let s2 =  String::from("ay");
            s = word.to_owned()+&s2;
        }
        rezult.push(s);
        
    }
    println!("Итог: {:?}", rezult);
    
}
