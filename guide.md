# lens-rs guide

Several days ago, I've written a library [`lens-rs`](https://github.com/TOETOE55/lens-rs) for Rust accidentally . 

Before, I have never imagined that the lens could be implemented in Rust and would be so useful in Rust.Compare with `lens` in Haskell, there are no weird operators, grotesque symbols and confusing types in this case. 

let me show you how to play with `lens-rs`!

## Access nested structure is easy

* `optics!()` is a macro to compose optics which 
  could be seen as a *path* to access the nested structure.
  For instance, `optics!(a._0.Ok)` is used to peek the data like
  `Foo { a: (Ok(/* look at me! */), "I don't care"), .. }`.
* `optic.view()`, `.view_mut()` and `.view_ref()` 
  can visit the substructure which **must** exist (exactly one) like nested structs and tuples.
* `pm()`, `pm_mut()` and `.pm_ref()`
  can visit the substructure which **may** exist (one or none) like nested enum variants.
* `.traverse()`, `.traverse_mut()` and `traverse_ref()`
  can visit **multiple** substructures (zero or more) like vec or vec in vec.
  
Given suuuuuper complex value named `x` as an example:

```rust
let mut x = (
    1,
    Ok((
        vec![
            Some(("a".to_string(), 2)),
            None,
            Some(("b".to_string(), 3)),
        ],
        4,
    )),
);
```

And `x.0` must exist. We could modify it by `.view_mut()`.

```rust
*optics!(_0).view_mut(&mut x) += 1; // x.0 += 1: 1 -> 2
```
But `x.1` may be `Ok` or `Err`. We should set it by `.pm_mut()`.
```rust
*optics!(_1.Ok._1).pm_mut(&mut x)? *= 2 // x.1.Ok.1? *= 2: 4 -> 8
*optics!(_1.Err).pm_mut(&mut x)? *= 2   // do nothing
```
Then `x.1.Ok.0` is a vec. We should traverse it by `.traverse_xxx()`.

```rust
optics!(_1.Ok._0._mapped.Some._0)
        .traverse_mut(&mut x)
        .into_iter()
        .for_each(|s| *s = s.to_uppercase());       // "a" -> "A", "b" -> "B"

optics!(_1.Ok._0._mapped.Some._1).traverse_ref(&x); // vec![&2, &3]
```

Besides, `.review()` is used to construct a single value.

```rust
let y: _ = optic!(Ok.Some.Err).review((1, 2));
// Ok(Some(Err((1, 2))))
```

## Generate your own optics

`lens-rs` will scan your source code, and generate optics into `lens_rs::optics` for fields and variants where you marked it with `#[optic]`.

the derive-macro
* `Review` is used to impl `Review` for optics
* `Prism` is used to impl `Prism` for optics
* `Lens` is used to impl `Lens` for optics

```rust
#[derive(Copy, Clone, Debug, Review, Prism)]
enum Either<L, R> {
    #[optic]
    Left(L), // generate optics::Left
    #[optic]
    Right(R), // generate optics::Right
}

#[derive(Copy, Clone, Debug, Lens)]
struct Tuple<A, B>(#[optic] A, #[optic] B); 
// use optics::_0 or optics::_1 to access it

#[derive(Copy, Clone, Debug, Lens)]
struct Foo<A, B> {
    #[optic]
    a: A, // generate optics::a
    #[optic]
    b: B, // generate optics::b
}

#[derive(Clone, Debug, Lens)]
struct Bar {
    #[optic]
    a: String, // generate optics::a, same as above
    #[optic]
    c: i32,    // generate optics::c
}
```

the attribute
* `#[optic(ref)]` is to mark a field/variant which its optic impl only `LensRef`/`PrismRef` trait
* `#[optic(mut)]` is to mark a field/variant which its optic impl `LensMut`/`PrismMut` trait
* `#[optic]` is to mark a field/variant which its optic impl `Lens`/`Prism` trait

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
```

Limitations:
* can only derive `Lens` for struct
* can only derive `Review` and `Prism` for enum variant with *single*, *unnamed* argument.

## A little row-polymorphism

You can represent a type has values of some types

```rust
fn may_have_i32<T, Pm: PrismMut<T, To=i32>>(t: &mut T, pm: Pm) {
//                           ^ `T` may have a value of `i32`
    pm.pm_mut(t).map(|x| *x+=1);
}
// the above x
may_have_i32(&mut x, optics!(_0));       // 2 -> 3
may_have_i32(&mut x, optics!(_1.Ok._1)); // 8 -> 9
may_have_i32(&mut x, optics!(_1.Err));   // do nothing
```

or a type has some fields:
```rust
fn with_field_a<T>(t: T) -> String
where
    field![a]: Lens<T, To=String> // `T` must have field `.a`
{
    optics!(a).view(t)
}

let foo = Foo {
    a: "this is Foo".to_string(),
    b: ()
};
let bar = Bar {
    a: "this is Bar".to_string(),
    c: 0
};

println!("{}", with_field_a(foo)); // this is Foo
println!("{}", with_field_a(bar)); // this is Bar
```

## An optic is an onion

the traits behind are:

```rust
/// Review
pub trait Review<T> {
    type From;
    fn review(&self, from: Self::From) -> T;
}

/// Traversal
pub trait TraversalRef<T> {
    type To;
    fn traverse_ref<'a>(&self, source: &'a T) -> Vec<&'a Self::To>;
}

pub trait TraversalMut<T>: TraversalRef<T> {
    fn traverse_mut<'a>(&self, source: &'a mut T) -> Vec<&'a mut Self::To>;
}

pub trait Traversal<T>: TraversalMut<T> {
    fn traverse(&self, source: T) -> Vec<Self::To>;
}

/// Prism
pub trait PrismRef<T>: TraversalRef<T> {
    fn pm_ref<'a>(&self, source: &'a T) -> Option<&'a Self::To>;
}

pub trait PrismMut<T>: PrismRef<T> + TraversalMut<T> {
    fn pm_mut<'a>(&self, source: &'a mut T) -> Option<&'a mut Self::To>;
}

pub trait Prism<T>: PrismMut<T> + Traversal<T> {
    fn pm(&self, source: T) -> Option<Self::To>;
    // ^ pattern match
}

/// Lens
pub trait LensRef<T>: PrismRef<T> {
    fn view_ref<'a>(&self, source: &'a T) -> &'a Self::To;
}

pub trait LensMut<T>: LensRef<T> + PrismMut<T> {
    fn view_mut<'a>(&self, source: &'a mut T) -> &'a mut Self::To;
}

pub trait Lens<T>: LensMut<T> + Prism<T> {
    fn view(&self, source: T) -> Self::To;
}
```

the optics are the types like:
```rust
/// optics for `Result`
struct Ok<Optic>(Optic);
struct Err<Optic>(Optic);

/// optics for tuples
struct _0<Optic>(Optic);
struct _1<Optic>(Optic);
// 0 to 6

/// the basic optic
struct __;
```

the optic implementation like:
```rust
impl<Pm, T, E> Prism<Result<T, E>> for optics::Ok<Pm>
where
    Pm: Prism<T>,
{
    // type To = Pm::To;
    fn pm(&self, source: Result<T, E>) -> Option<Self::To> {
        source.ok().and_then(|t| self.0.pm(t))
    }
}
```

so, just like an onion
* the `optics!(_0.Ok.Some)` will convert to `optics::_0(optics::Ok(optics::Some(optics::__)))`
* the `field![_0.Ok.Some]` will convert to `optics::_0<optics::Ok<optics::Some<optics::__>>>`

## It's just the van Laarhoven style optic

it's easy to see that in a little bit of generation(may not correspond exactly):
```rust
trait Traversal<T> {
    fn traverse<F: FromIterator<Self::To>>(&self, data: T) -> F;
    // forall F. Traversable F => T -> F Self::To 
}

trait Prism<T> {
    fn pm<F: From<Option<Self::To>>>(&self, source: T) -> F;
    // forall F. Pointed F => T -> F Self::To
}

trait Lens<T> {
    fn view<F: From<Self::To>>(&self, source: T) -> F;
    // forall F. Functor F => T -> F Self::To
}
```

Hence
* `optics::Ok` (the value constructor) can be treat as `forall f. (T -> f a) -> (Result<T, E> -> f b)`
* `optics::_1` (the value constructor) can be treat as `forall f. (A -> f a) -> ((A, B) -> f b)`
* `optics::_1(optics::Ok(optics::_mapped(optics::__)))` (the value) can be treat as `forall f. (A, Result<Vec<T>, E>) -> f T`

***

The library I've uploaded to [crates.io](https://crates.io/crates/lens-rs). 
I would be glad someone is using this library!

oh, don't forget to add this in `Cargo.toml`
```toml
[package.metadata.inwelling]
lens-rs = true
```

Finally, I must thank @oooutlk, he helped me a lot in developing `lens-rs_derive`(generating optics in `lens_rs::optics` using `inwelling`).