use std::fs::File;

fn main() {
    // let f = File::open("hello.txt");
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("We don't have the file!");

    // let foo = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("file was not found!");
    //     },
    // };
}
