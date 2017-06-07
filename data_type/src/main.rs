fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{}, {}, {}", five_hundred, six_point_four, one);

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("first: {}, second: {}", first, second);
    println!("The value of elem: {}", a[10]);
}
