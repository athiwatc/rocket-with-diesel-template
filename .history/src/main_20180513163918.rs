#[warn(unknown_lints)]
#![warn(clippy)]

#![feature(plugin)]
#![plugin(rocket_codegen)]

fn main() {
    let a = 234230490324;
    println!("Hello, world! {}", a);
}
