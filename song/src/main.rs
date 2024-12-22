fn main() {
    let text = [
        "A Partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", 
        "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];
    for (day_index, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas, \nMy true love send to me:", day);
        for text_index in (0..=day_index).rev() {
            if text_index == 0 && day_index > 0 {
                print!("And ");
            }
            println!("{}", text[text_index]);
        }
        println!("");
    }
}
