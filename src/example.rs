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

fn test1() -> Option<()> {
    let mut nested: Result<Result<_, ()>, ()> = optics!(_Ok._Ok).review((1, 2));
    *optics!(_Ok._Ok._0).pm_mut(&mut nested)? += 1;
    assert_eq!(optics!(_Ok._Ok._0).pm(nested)?, 2);

    let mut x = (1, (2, (3, 4)));
    *optics!(_1._1._1).view_mut(&mut x) *= 2;
    assert_eq!(optics!(_1._1._1).view(x), 8);

    let mut x: (_, Result<_, ()>) = (1, Ok((2, 3)));
    *optics!(_1._Ok._1).pm_mut(&mut x)? *= 2;
    assert_eq!(optics!(_1._Ok._1).pm(x)?, 6);

    let mut x = (1, vec![Some((2, 3)), None]);
    optics!(_1.Mapped._Some._0)
        .traverse_mut(&mut x)
        .into_iter()
        .for_each(|i| *i += 1);
    assert_eq!(optics!(_1.Mapped._Some._0).traverse(x), vec![3]);

    Some(())
}


fn test2() -> Option<()> {
    let x = optics!(_Some._B).review(Foo {
        a: 3,
        b: 2,
    });
    assert_eq!(optics!(_Some._B._b).pm(x)?, 2);

    Some(())
}