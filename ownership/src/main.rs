fn main() {
    let s = "hello"; // string literal cannot be mutated
    let mut s = String::from(s);
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}");

    let x = 5;
    let y = x;
    println!("x: {x} y: {y}");
    /*
        We now have two variables, x and y, and both equal 5. This is indeed what is 
        happening, because integers are simple values with a known, fixed size, and these 
        two 5 values are pushed onto the stack.
    */

    let s1 = String::from("hello");
    let s2 = s1;
    println!("s1: invalidated s2: {s2}");
    /*
        When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to. 
        To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer
        valid.
        If I tried to run println!("{}, world!", s1); I'd get an error because s1 is not 
        valid anymore at this point
    */

    /*
        If we want to copy the heap data as well we use the clone method
    */
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    println!("s1 = {}, s2 = {}", s1, s2);

/**********************************************************/
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    let s1 = gives_ownership();  // gives_ownership moves its return
                                    // value into s1

/**********************************************************/
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
/**********************************************************/
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
/**********************************************************/
    let mut s = String::from("hello");
    change(&mut s);
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn gives_ownership() -> String {        // gives_ownership will move its
    // return value into the function
    // that calls it
    
    let some_string = String::from("yours"); // some_string comes into scope
    
    some_string                      // some_string is returned and
    // moves out to the calling
    // function
}
        
// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                        // scope
    
    a_string  // a_string is returned and moves out to the calling function
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.