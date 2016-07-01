#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]

//extern crate rustproof;

fn main() {
    let mut x = 3;
    x = add_five(x);
    x = add_three(x);
    println!("{:?}", x);
}

#[condition(pre="x > 0", post="x >= 5")]
fn add_five(mut x: i32) -> i32 {
    x = x + 5;
    return x;
}

fn add_three(mut x: i32) -> i32 {
    x = x + 5;
    return x;
}

struct Foo;

struct Bar {
    id: u32,
}
