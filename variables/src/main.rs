fn main() {
    println!("Simple Variables");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // You aren’t allowed to use mut with constants. Constants are always immutable. 
    // The type of the value must be annotated.
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;
    println!("Constants\n3 hours in seconds is {THREE_HOURS_IN_SECONDS}");
    println!("Shadowing");
    let y = 5;
    let y = y + 1;
    {
        // Shadowing is different from marking a variable as mut 
        // because we’ll get a compile-time error if we accidentally 
        // try to reassign to this variable without using the let keyword.
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }
    println!("The value of y is {y}");
    // We’re effectively creating a new variable when we use the let keyword again
    // we can change the type of the value but reuse the same name
    let spaces = "     ";
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");
}
