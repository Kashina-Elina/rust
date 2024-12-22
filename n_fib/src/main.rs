use std::io;

fn main() {
    println!("Введите число n");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let new_n: i32 =  n.trim().parse().expect("Введите другое число");
    println!("new_n {}", new_n);
    let x = fib(new_n);
    println!("Итоговое число: {}", x);
}

fn fib(n: i32) -> i32 {
    let mut m: u32 = 0;
    if n <= 0 {
        0
    }
    else if n==1 {
        1
    }
    else {
        fib(n-1)+fib(n-2)
    }
}