# Overview
deriving lens for custom data types.

## Example
```rust
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
    let x = optics!(Some.B).review(Foo {
        a: 3,
        b: 2,
    });
    assert_eq!(optics!(Some.B.b).pm(x)?, 2);
    
    Some(())
}
```

## Limitation
* can't derive `Lens` for enum.
* can't derive `Prism` and `Review` for the variant has more than one argument or has named field.