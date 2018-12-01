fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");
    let x = 5 + 5;
    println!("Value of x is {}", x);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
}

