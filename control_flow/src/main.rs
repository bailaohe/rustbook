fn main() {
    let mut number = 3;

    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for (i, element) in a.iter().enumerate() {
        println!("the {}-value is: {}", i, element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
