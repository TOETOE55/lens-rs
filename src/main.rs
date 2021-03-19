#![allow(non_camel_case_types)]
use lens::*;
use lens_derive::*;

#[derive(Optic, Review, Prism, Debug)]
enum AnEnum<T> {
    A(T, i32),
    #[optic] B(T),
}

#[derive(Optic, Lens, Debug)]
struct Foo {
    #[optic] a: i32,
    #[optic] b: i32,
}


fn main() {
    let x = optics!(_Some._B).review(Foo {
        a: 3,
        b: 2,
    });
    println!("{:?}", optics!(_Some._B._b).pm(x));
}
