# lens-rs_derive
deriving lens for custom data types.

## Example
```rust
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

## Limitation
* can't derive `Lens` for enum.
* can't derive `Prism` and `Review` for the variant has more than one argument or has named field.