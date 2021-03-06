# Overview
deriving lens for custom data types.

see [the guide](https://github.com/TOETOE55/lens-rs/blob/master/guide.md)

## Example
```rust
#[derive(Debug, Lens)]
struct Baz<'a, A, B, C>{
    #[optic(ref)] 
    a: &'a A,     // can only take the immutable ref by optics::a
    #[optic(mut)] 
    b: &'a mut B, // can take the mutable ref by optics::b
    #[optic]
    c: C          // can mv it out by by optics::c
}

#[derive(Review, Prism, Debug)]
enum AnEnum<T> {
    A(T, i32), // couldn't derive Prism or Review
    #[optic] 
    B(T),
    #[optic]
    C,
    #[optic]
    D(),
    #[optic]
    E {},
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

## Limitations
* can't derive `Lens` for enum.
* can't derive `Prism` and `Review` for the variant has more than one argument or has named field.