use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for ref item in list.iter() {
        if item > &largest {
            largest = &item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
