fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("a: {}", a[0]);
    let first = a[0];
    println!("first: {}", first);

    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    // NG
    //println!("months: {}", months);

    println!("Jan: {}", months[0]);


    let valid_index = 1;
    let invalid_index = 6;

    println!("valid: {}", a[valid_index]);
    println!("invalid: {}", a[invalid_index]);
}
