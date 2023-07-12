// 0 1 1 2 3 5 8 13 21 34 55
const N:u32 = 127;

/**
 * I'm offline and I don't remember whether to count from 0 
 */
fn main() {
    let mut count = 0;
    let mut fib:u64 = 0;
    let mut prev:u64 = 0;

    while count < N {
        if count != 0 {
            let temp:u64 = fib + prev;
            prev = fib;
            fib = temp;
        } else {
            fib = 0;
            prev = 1;
        }
        println!("{count} - Current: {fib}");
        count += 1;
    }
    println!("The {N} number in the Fibonacci sequence is {fib}");
}