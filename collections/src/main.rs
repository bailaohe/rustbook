fn main() {
    let v = vec![1, 2, 3];

    println!("{:?}", &v[1]);

    let data = "initial contents";

    let mut s: String = data.to_string();

    s.push_str("bar");

    s.push('1')
}
