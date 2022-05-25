fn main() {
    println!("The fizz buzz challenge");

    // using while loops

    let mut count = 0;

    while count < 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 3 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        count += 1;
    }

    // using range

    for i in 0..100 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 3 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }

    }
}
