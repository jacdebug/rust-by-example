fn main() {
    // println! is a macro that prints text to the console
    println!("Hello World!");
    println!("I'm a Rustacean!");

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    let x = 5 + 5;
    println!("Value of x is {}", x);

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`
    println!("{} of {:b} people know binary, the other half doesn't", 1, 3);

    // You can right-align text with a specified width
    println!("{number:width$}", number=1, width=6);
    println!("{0:1$}", 1, 6);

    println!("Pi is roughly {pi:.*}", 3, pi=22f64/7f64);
    println!("Pi is roughly {1:.*}", 3, 22f32/7f32);

    // {:?}, {:#?} marker for debugging purposes.
    #[derive(Debug)]
    struct Structure(i32);

    println!("Debug print struct `{:?}`", Structure(3));
    println!("Debug pretty print struct `{:#?}`", Structure(3));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "David";
    let age = 3;
    let peter = Person { name, age };

    println!("{:?}", peter);
    println!("{:#?}", peter);
}
