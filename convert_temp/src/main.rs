use std::io;

fn main() {
    println!("Please provide a temperature in Fahrenheit");
    let mut f = String::new();
    io::stdin()
        .read_line(&mut f)
        .expect("Error reading stdin");
    let f:i32 = f
        .trim()
        .parse()
        .expect("Failed to parse number");
    println!("You entered {f}");
    let c = f_to_c(f);
    println!("It corresponds to {c}° Celsius");
}

fn f_to_c(f:i32) -> i32 {
    (f - 32) * 5 / 9
}
