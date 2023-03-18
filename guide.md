# lens-rs guide

Several days ago, I've written a library [`lens-rs`](https://github.com/TOETOE55/lens-rs) for Rust. 

Before, I have never imagined that the lens could be implemented in Rust and would be so useful in Rust.Compare with `lens` in Haskell, there are no weird operators, grotesque symbols and confusing types in this case. 

let me show you how to play with `lens-rs`!

## Access nested structure is easy

* `optics!()` is a macro to compose optics which 
  could be seen as a *path* to access the nested structure.
  For instance, `optics!(a._0.Ok)` is used to peek the data like
  `Foo { a: (Ok(/* look at me! */), "I don't care"), .. }`.
* `optic.view()`, `.view_mut()` and `.view_ref()` 
  can visit the substructure which **must** exist (exactly one) like nested structs and tuples.
* `.preview()`, `preview_mut()` and `.preview_ref()`
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
*x.view_mut(optics!(_0)) += 1; // x.0 += 1: 1 -> 2
```
But `x.1` may be `Ok` or `Err`. We should set it by `.preview_mut()`.
```rust
*x.preview_mut(optics!(_1.Ok._1))? *= 2 // x.1.Ok.1? *= 2: 4 -> 8
*x.pm_mut(optics!(_1.Err))? *= 2   // do nothing
```
Then `x.1.Ok.0` is a vec. We should traverse it by `.traverse_xxx()` or access it by index optics.

```rust
x
    .traverse_mut(optics!(_1.Ok._0._mapped.Some._0))
    .into_iter()
    .for_each(|s| *s = s.to_uppercase());       // "a" -> "A", "b" -> "B"

x.traverse_ref(optics!(_1.Ok._0._mapped.Some._1)); // vec![&2, &3]
x.preview_ref(optics!(_1.Ok._0.[2].Some._1))?; // &3
```


Besides, `.review()` is used to construct a single value.

```rust
let y: _ = optic!(Ok.Some.Err).review((1, 2));
// Ok(Some(Err((1, 2))))
```

## Generate your own optics

`lens-rs` will scan your source code, and generate optics into `lens_rs::optics` for fields and variants where you marked it with `#[optic]`.

the derive-macro
* `Optic` is used to impl `Optic` for optics
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
fn may_have_i32<Pm, T: PrismMut<T, i32>>(t: &mut T, pm: Pm) {
//                       ^ `T` may have a value of `i32`
    t.preview_mut(pm).map(|x| *x+=1);
}
// the above x
may_have_i32(&mut x, optics!(_0));       // 2 -> 3
may_have_i32(&mut x, optics!(_1.Ok._1)); // 8 -> 9
may_have_i32(&mut x, optics!(_1.Err));   // do nothing
```

or a type has some fields:
```rust
fn with_field_a<T>(t: &T) -> &str
where
    T: LensRef<Optics![a], String>, // T must have field a
{
    t.view_ref(optics!(a))
}


let foo = Foo {
    a: "this is Foo".to_string(),
    b: (),
};
let bar = Bar {
    a: "this is Bar".to_string(),
    c: 0,
};

assert_eq!(with_field_a(&foo), "this is Foo");
assert_eq!(with_field_a(&bar), "this is Bar");
```

or a type *may* have some fields:

```rust
fn may_has_c<T>(t: T) -> Option<i32>
where
    T: Prism<Optics![c], i32>,
{
    t.preview(optics!(c))
}

let foo = Foo {
    a: "this is Foo".to_string(),
    b: (),
};
let bar = Bar {
    a: "this is Bar".to_string(),
    c: 0,
};
let left: Either<i32, i32> = Left(0);

assert_eq!(may_has_c(foo), None);
assert_eq!(may_has_c(bar), Some(0));
assert_eq!(may_has_c(left), None);
assert_eq!(may_has_c((1, 2, 3)), None);
```

## An optic is an onion

the traits behind are:

```rust
/// base optic
pub trait Optic<Opt> {
    type Image: ?Sized;
}

/// Review
pub trait Review<Opt>: Optic<Opt> {
    fn review(optics: Opt, from: Self::Image) -> Self
    where
        Self::Image: Sized;
}

/// Traversal
pub trait TraversalRef<Opt>: Optic<Opt> {
    fn traverse_ref(&self, optics: Opt) -> Vec<&Self::Image>;
}

pub trait TraversalMut<Optics>: TraversalRef<Optics> {
    fn traverse_mut(&mut self, optics: Optics) -> Vec<&mut Self::Image>;
}

pub trait Traversal<Optics>: TraversalMut<Optics> {
    fn traverse(self, optics: Optics) -> Vec<Self::Image>
    where
        Self::Image: Sized;
}

/// Prism
pub trait PrismRef<Optics>: TraversalRef<Optics> {
    fn preview_ref(&self, optics: Optics) -> Option<&Self::Image>;
}

pub trait PrismMut<Optics>: PrismRef<Optics> + TraversalMut<Optics> {
    fn preview_mut(&mut self, optics: Optics) -> Option<&mut Self::Image>;
}

pub trait Prism<Optics>: PrismMut<Optics> + Traversal<Optics> {
    fn preview(self, optics: Optics) -> Option<Self::Image>
    where
        Self::Image: Sized;
}

/// Lens
pub trait LensRef<Optics>: PrismRef<Optics> {
    fn view_ref(&self, optics: Optics) -> &Self::Image;
}

pub trait LensMut<Optics>: LensRef<Optics> + PrismMut<Optics> {
    fn view_mut(&mut self, optics: Optics) -> &mut Self::Image;
}

pub trait Lens<Optics>: LensMut<Optics> + Prism<Optics> {
    fn view(self, optics: Optics) -> Self::Image
    where
        Self::Image: Sized;
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
impl<Pm, Image, T, E> Prism<optics::Ok<Pm>, Image> for Result<T, E>
where
    T: Prism<Pm, Image>,
{
    fn preview(self, optics: optics::Ok<Pm>) -> Option<Image> {
        self.ok().and_then(|t| optics.0.pm(t))
    }
}
```

so, just like an onion
* the `optics!(_0.Ok.Some)` will convert to `optics::_0(optics::Ok(optics::Some(optics::__)))`
* the `Optics![_0.Ok.Some]` will convert to `optics::_0<optics::Ok<optics::Some<optics::__>>>`

## It's just the van Laarhoven style optic

it's easy to see that in a little bit of generation(may not correspond exactly):
```rust
trait Traversal<Optics, Image> {
    fn traverse<F: FromIterator<Image>>(&self, optics: Optics) -> F;
    // forall F. Traversable F => Self -> F Self::To 
}

trait Prism<Optics, Image> {
    fn pm<F: From<Option<Image>>>(&self, optics: Optics) -> F;
    // forall F. Pointed F => Self -> F Self::To
}

trait Lens<T, Image> {
    fn view<F: From<Image>>(&self, source: Optics) -> F;
    // forall F. Functor F => Self -> F Self::To
}
```

Hence
* `optics::Ok` (the value constructor) can be treat as `forall f. (T -> f a) -> (Result<T, E> -> f b)`
* `optics::_1` (the value constructor) can be treat as `forall f. (A -> f a) -> ((A, B) -> f b)`
* `optics::_1(optics::Ok(optics::_mapped(optics::__)))` (the value) can be treat as `forall f. (A, Result<Vec<T>, E>) -> f T`

> In fact, `Traversal`, `Prism` and `Lens` in `lens-rs` actually correspond to `Fold`, `AffineFold` and `Getter` in Haskell's `lens`.
But with the mutable trait, they actually do the same as `Traversal`, `Prism` and `Lens` in `lens`. Then I named them so.

***

The library I've uploaded to [crates.io](https://crates.io/crates/lens-rs). 
I would be glad someone is using this library!

oh, don't forget to add this in `Cargo.toml`
```toml
[build-dependencies]
inwelling = "0.4"

[package.metadata.inwelling]
lens-rs_generator = true
```

and add this in `build.rs`
```rust
fn main() {
  inwelling::register();
}
```

Finally, I must thank @oooutlk, he helped me a lot in developing `lens-rs_derive`(generating optics in `lens_rs_generator::*` using [`inwelling`](https://github.com/oooutlk/inwelling)).
