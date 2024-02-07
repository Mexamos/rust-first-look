fn main() {
    let refers_notiong = dangle();
    println!("refers_notiong = {}", refers_notiong);
}

fn dangle() -> String {
    let s = String::from("Hello");

    s
}
