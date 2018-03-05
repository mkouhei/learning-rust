fn main() {
    //let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    // NG
    //println!("tup: {}", tup);

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_handred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("0: {}", five_handred);
    println!("0: {}", tup.0);
    println!("1: {}", six_point_four);
    println!("1: {}", tup.1);
    println!("2: {}", one);
    println!("2: {}", tup.2);
}
