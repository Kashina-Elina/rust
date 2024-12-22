use std::io;

fn main() {
    println!("Введите число в градуах цельсия");
    let mut c_degrees = String::new();

    io::stdin()
        .read_line(&mut c_degrees)
        .expect("Failed to read line");

    let c_degrees: f32 =  c_degrees.trim().parse().expect("Введите другое число");

    let f_degrees: f32 = c_degrees*1.8+32.0;

    println!("Значение в градусах по Фаренгейту {}", f_degrees)
}
