

fn greet() -> i32 {
    42
}

fn main() {
    let a = 1;
    println!("{}", greet());
    // place holder for didChange test.
}
// place holder for didChange test.

mod lib;
use lib::yo;
fn yo_in_main() -> i32 {
    yo() + 1
}