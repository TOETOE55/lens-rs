#![allow(non_camel_case_types)]
use lens::*;
use lens_derive::*;

#[derive(Optic, Review, Prism, Debug)]
enum AnEnum {
    A(Foo, i32),
    B(String),
}


#[derive(Optic, Lens, Debug)]
struct AStruct {
    a: i32,
    b: String,
}

#[derive(Optic, Lens, Debug)]
struct Foo(i32, i32);

fn main() {
    let x = optics!(_Some._A).review((Foo(3,1), 2));

    println!("{:?}", optics!(_Some._A._0).pm(x));
}
