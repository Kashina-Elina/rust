use std::fs::File;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut content = String::new();
    let mut file = File::open("file.txt")?;
    file.read_to_string(&mut content)?;
    let mut lines: Vec<Vec<String>> = Vec::new();
    for line in content.lines() {
        let line = line;
        let split_line: Vec<String> = line.split_whitespace().map(String::from).collect();
        lines.push(split_line);
    }
    let row_count = lines.len();
    let col_count = lines[0].len();
    for col in 0..col_count {
        let transposed_row: Vec<&str> = (0..row_count).map(|row| &lines[row][col][..]).collect();
        println!("{}", transposed_row.join(" "));
        }
    Ok(())
}