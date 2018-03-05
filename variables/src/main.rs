fn main() {
    const MAX_POINTS: u32 = 100_000;
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    println!("The value of MAX_POINTS is {}", MAX_POINTS);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is {}", spaces);

    //let mut spaces = "   ";
    //spaces = spaces.len();

    let spaces = "   ";
    println!("The value of spaces is {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess)
}
