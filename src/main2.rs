use std::io::Write;
use std::fmt;
use std::str::FromStr;

fn add<T>(var1: T, var2: T) -> T
    where T: std::ops::Add<Output = T> {
    var1 + var2
}

fn sub<T>(var1: T, var2: T) -> T
    where T: std::ops::Sub<Output = T> {
    var1 - var2
}

fn mul<T>(var1: T, var2: T) -> T
    where T: std::ops::Mul<Output = T> {
    var1 * var2
}

fn div<T>(var1: T, var2: T) -> T
    where T: std::ops::Div<Output = T> {
    var1 / var2
}

fn read_input<T>(msg: &str) -> T
  where T: std::str::FromStr,
        <T as FromStr>::Err: fmt::Debug {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut buf = String::new();

    print!("{}", msg);
    stdout.flush().unwrap();

    stdin.read_line(&mut buf).unwrap();
    buf.trim().parse::<T>().unwrap()
}

fn main() {
    let mut num1: i32;
    let mut num2: i32;
    let mut operator: char;

    loop {
        num1 = read_input("Number 1: ");
        num2 = read_input("Number 2: ");
        operator = read_input("Operator: ");

        let result = match operator {
            '+' => Some(add::<i32>(num1, num2)),
            '-' => Some(sub::<i32>(num1, num2)),
            '*' => Some(mul::<i32>(num1, num2)),
            '/' => Some(div::<i32>(num1, num2)),
            _   => None,
        };

        match result {
            Some(r) => println!("Result: {}", r),
            None    => {
                eprintln!("Wrong Operator!!!");
                continue;
            },
        }

        println!("\n-------------------------------------\n");
    }
}
