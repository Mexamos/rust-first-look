fn main() {
    let tup: (i32, f64, u8) = (500, 3.5, 1);

    let (x, y, z) = tup;

    let a = tup.0;
    let b = tup.1;

    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("a = {}, b = {}", a, b);

    // Arrays should have elements with the same type
    // they have fix length
    // use it to allocate data on the stack

    let month = ["Jan", "Feb", "Mar", "Apr"];

    println!("First month = {}", month[0]);

    println!("Ten month = {}", month[10]);
}

// Rust immediately exits program insteaad of allowing the memory access