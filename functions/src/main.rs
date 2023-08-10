// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

fn main() {
    println!("Hello, world!");

    another_function(5, 'h');
    
    let x = five();
    println!("The value of x is {x}");
    
    let x = plus_one(five());
    println!("The value of x is {x}");
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}


fn five() -> i32 {
    5   // 5 is an expression and therefore no ; after it
}

fn plus_one(x: i32) -> i32 {
    x + 1
}