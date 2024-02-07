struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 10, height: 20
    };

    // println!("Area is {}", area(&rec1));

    println!("Area = {}", rec1.area());
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}
