# Overview
deriving lens for custom data types.

## Example
```rust
#[derive(Debug, Optic, Lens)]
struct Baz<'a, A, B, C>{
    #[optic(ref)] 
    a: &'a A,     // can only take the immutable ref by optics::a
    #[optic(mut)] 
    b: &'a mut B, // can take the mutable ref by optics::b
    #[optic]
    c: C          // can mv it out by by optics::c
}

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
    let x = Review::review(optics!(Some.B), Foo {
        a: 3,
        b: 2,
    });
    assert_eq!(x.preview(optics!(Some.B.b))?, 2);
    
    Some(())
}
```

## Limitation
* can't derive `Lens` for enum.
* can't derive `Prism` and `Review` for the variant has more than one argument or has named field.