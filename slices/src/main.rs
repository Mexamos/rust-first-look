fn main() {
    let s = String::from("Hello, world");

    let hello = &s[0..5];

    println!("hello = {}", hello);

    let a = [1,2,3,4,5];
    let a_slice = &a[1..3];
}
