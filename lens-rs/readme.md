# lens-rs
lens implemented in rust

* the `Review` optics describes how to construct a single value.
* A `Traversal` can access the multiple substructures.
* A `Prism` can access the substructure may exist.
* A `Lens` can access the substructure must exist.

## Example
access the substructure
```rust
fn test() -> Option<()> {
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
```

derive lens for data types
```rust
use lens_rs_derive::*;

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


fn test() -> Option<()> {
    let x = optics!(_Some._B).review(Foo {
        a: 3,
        b: 2,
    });
    assert_eq!(optics!(_Some._B._b).pm(x)?, 2);
    
    Some(())
}
```

assume a type T may have field that the type is `i32`.
```rust
fn bar<T, Pm: Prism<T, To=i32>>(t: &mut T, pm: Pm) {
    pm.pm_mut(t).map(|x| *x += 2);
}

fn test() {
    let mut complex = (1, Ok((Err(2), 3)));
    bar(&mut complex, optics!(_0));
    bar(&mut complex, optics!(_1._Err)); // do nothing
    bar(&mut complex, optics!(_1._Ok._0._Ok)); // do nothing
    bar(&mut complex, optics!(_1._Ok._0._Err));
    bar(&mut complex, optics!(_1._Ok._1));
    assert_eq!(complex, (3, Ok((Err(4), 5))));
}
```