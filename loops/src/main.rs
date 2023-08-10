fn main() {
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            // you can add the value you want returned 
            // after the break expression you use to stop the loop; 
            // that value will be returned out of the loop so you can use it
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    //  You can optionally specify a loop label on a loop that you can then 
    // use with break or continue to specify that those keywords apply to the 
    // labeled loop instead of the innermost loop.
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
