fn main() {
    // let mut s = String::from("Hello");
    // s.push_str(", world!");
    // println!("s = {}", s);

    let mut x = String::from("Hello");
    takes_ownership(&mut x);
    println!("x = {}", x);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}", s1); // Here will be error
}

fn takes_ownership(some_str: &mut String) {
    println!("some string {}", some_str);
    some_str.push_str(" World!");
}
