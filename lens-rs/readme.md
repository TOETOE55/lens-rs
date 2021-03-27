# lens-rs
lens implemented in rust

* the `Review` optics describes how to construct a single value.
* A `Traversal` can access the multiple substructures.
* A `Prism` can access the substructure may exist.
* A `Lens` can access the substructure must exist.

## Example
access the substructure
```rust
use lens_rs::*;
fn test() -> Option<()> {
    let mut nested: Result<Result<_, ()>, ()> = Review::review(optics!(Ok.Ok), (1,2));
    *x.preview_mut(optics!(Ok.Ok._0))? += 1;
    assert_eq!(nested.preview(optics!(Ok.Ok._0))?, 2);

    let mut x = (1, (2, (3, 4)));
    *x.view_mut(optics!(_1._1._1)) *= 2;
    assert_eq!(x.view(optics!(_1._1._1)), 8);

    let mut x: (_, Result<_, ()>) = (1, Ok((2, 3)));
    *x.preview_mut(optics!(_1.Ok._1))? *= 2;
    assert_eq!(x.preview(optics!(_1.Ok._1))?, 6);

    let mut x = (1, vec![Some((2, 3)), None]);
        x
        .traverse_mut(optics!(_1._mapped.Some._0))
        .into_iter()
        .for_each(|i| *i += 1);
    assert_eq!(x.traverse(optics!(_1._mapped.Some._0)), vec![3]);

    Some(())
}
```

derive lens for data types
```rust
use lens_rs::*;

#[derive(Review, Prism, Debug)]
enum AnEnum<T> {
    A(T, i32),
    #[optic] B(T),
}

#[derive(Lens, Debug)]
struct Foo {
    #[optic] a: i32,
    #[optic] b: i32,
}


fn test() -> Option<()> {
    let x = Review::review(optics!(Some.B), Foo {
        a: 3,
        b: 2,
    });
    assert_eq!(x.preview(optics!(Some.B.b))?, 2);
    
    Some(())
}
```

assume a type T may have substructure that the type is `i32`.
```rust
fn bar<T, Pm: Prism<T, To=i32>>(t: &mut T, pm: Pm) {
    t.preview_mut(pm).map(|x| *x += 2);
}

fn test() {
    let mut complex = (1, Ok((Err(2), 3)));
    bar(&mut complex, optics!(_0));
    bar(&mut complex, optics!(_1.Err)); // do nothing
    bar(&mut complex, optics!(_1.Ok._0.Ok)); // do nothing
    bar(&mut complex, optics!(_1.Ok._0.Err));
    bar(&mut complex, optics!(_1.Ok._1));
    assert_eq!(complex, (3, Ok((Err(4), 5))));
}
```

assume a type T must have a field `.a`.
```rust
fn with_field_a<T>(t: &T) -> &str
where
    T: LensRef<field![a], To=String>
{
    t.view_ref(optics!(a))
}

let foo = Foo {
    a: "this is Foo".to_string(),
    b: ()
};
let bar = Bar {
    a: "this is Bar".to_string(),
    c: 0
};

println!("{}", with_field_a(foo));
println!("{}", with_field_a(bar));
```

## Cargo.toml

add it in Cargo.toml

```toml
[package.metadata.inwelling]
lens-rs = true
```

