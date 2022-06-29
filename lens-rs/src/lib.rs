//! # Overview
//!
//! `lens-rs` is a lens library for Rust, just like the package [`lens`](https://hackage.haskell.org/package/lens) for Haskell,
//! but here are no weird operators, grotesque symbols and confusing types.
//! Not only can `lens-rs` get/set for fields of struct, but also variants of enum and items of collection.
//! It unifies the way to access to/construct with different nested substructures, providing Rusty-APIs.
//!
//!
//!
//! ## Before use
//!
//! Add the following in your Cargo.toml
//!
//! ```toml
//! [dependencies]
//! lens-rs = "0.3"
//!
//! [package.metadata.inwelling]
//! lens-rs_generator = true
//! ```
//!
//! Add the following in your .rs files
//!
//! ```ignore
//! use lens_rs::*;
//! ```
//!
//!
//! ## Usage
//!
//! ## Access substructures
//!
//! visit field in struct:
//!
//! ```ignore
//! let mut x = Foo { a: String::from(".a in Foo"), b: 0 };
//! assert_eq!(x.view_ref(optics!(a)), ".a in Foo");
//!
//! *x.view_mut(optics!(b)) += 1;
//! assert_eq!(x.view_ref(optics!(b)), &1)
//! ```
//!
//! visit variant in enum:
//!
//! ```ignore
//! let mut x = Ok(0);
//! *x.preview_ref(optics!(Ok))? += 1;
//! assert_eq!(x.preview_ref(optics!(Ok))?, &1);
//! assert_eq!(x.preview_ref(optics!(Err)), None);
//! ```
//!
//! visit items in collection:
//!
//! ```ignore
//! let mut x = vec![1, 2, 3];
//! x.traverse_mut(optics!(_mapped)).into_iter().for_each(|x| *x += 1);
//! assert_eq!(x.traverse_ref(optics!(_mapped)), vec![&2, &3, &4]);
//! assert_eq!(x.view_ref(optics!([1])), &3);
//! ```
//!
//! build a structure:
//!
//! ```ignore
//! let x = Review::review(optics!(Ok.Some), 1);
//! assert_eq!(x, Ok(Some(1)));
//! ```
//!
//! ## Compose optics
//!
//! macro `optics!()` and `Optics![]` is to compose optics:
//!
//! ```ignore
//! let optics: Optics![_1.a.Some._mapped.[1]] = optics!(_1.a.Some._mapped.[1]);
//!
//! let x = (0, Foo {
//!     a: Some(vec![vec![1,2], vec![3,4]]),
//!     b: ()
//! });
//!
//! assert_eq!(x.traverse(optics), vec![2, 4]);
//! ```
//!
//! ## Derive Optics
//!
//! Derive Lens for fields to use `.view_xx()`.
//!
//! ```ignore
//! #[derive(Lens)]
//! struct Foo<A, B> {
//!     #[optic]
//!     a: A, // generate optics::a
//!     #[optic]
//!     b: B, // generate optics::b
//! }
//!
//! #[derive(Lens)]
//! struct Tuple<A, B>(#[optic] A, #[optic] B);
//! // use optics::_0 or optics::_1 to access it
//! ```
//!
//! Derive Review/Prism for variants to use `Review::review`/`.preview_xx()`:
//!
//! ```ignore
//! #[derive(Review, Prism)]
//! enum Either<L, R> {
//!     #[optic]
//!     Left(L), // generate optics::Left
//!     #[optic]
//!     Right(R), // generate optics::Right
//! }
//! ```
//!
//! Control the mutability:
//!
//! ```ignore
//! #[derive(Debug, Lens)]
//! struct Bar<C>{
//!     #[optic(ref)]
//!     a: String,    // can only take the immutable ref of .a by optics::a
//!     #[optic(mut)]
//!     b: i32,       // can take the mutable ref of .b by optics::b
//!     #[optic]
//!     c: C          // can move .c out by by optics::c
//! }
//! ```
//!
//! ## A little row polymorphism
//!
//! restrict a type has some fields:
//!
//! ```ignore
//! fn with_field_a<T>(t: &T) -> &str
//! where
//!     T: LensRef<Optics![a], String>, // T must have field a
//! {
//!     t.view_ref(optics!(a))
//! }
//!
//! fn may_has_c<T>(t: T) -> Option<i32>
//! where
//!     T: PrismRef<Optics![c], i32>,  // T may have field c
//! {
//!     Some(*t.preview_ref(optics!(c))?)
//! }
//!
//! let foo = Foo {
//!     a: "this is Foo".to_string(),
//!     b: (),
//! };
//! let bar = Bar {
//!     a: "this is Bar".to_string(),
//!     c: 0,
//! };
//!
//! assert_eq!(with_field_a(&foo), "this is Foo");
//! assert_eq!(with_field_a(&bar), "this is Bar");
//!
//! assert_eq!(may_has_c(foo), None);
//! assert_eq!(may_has_c(bar), Some(0));
//! assert_eq!(may_has_c(Left(0)), None);
//! assert_eq!(may_has_c((1, 2, 3)), None);
//! ```
//!
//! ## Play with structx
//!
//! Now, `Lens` has implemented for [`structx`](https://crates.io/crates/structx)
//!
//! Add the following in your Cargo.toml
//!
//! ```toml
//! [dependencies]
//! lens-rs = { version = "0.3", features = [ "structx" ] }
//! structx = { version = "0.1", features = [ "lens-rs" ] }
//!
//! [package.metadata.inwelling]
//! lens-rs_generator = true
//! structx = true
//! ```
//!
//! Enjoy it!
//!
//! ```ignore
//! use structx::*;
//! use lens_rs::*;
//! let s1 = structx! { height: 1, width: 2 };
//! let s2 = structx! { height: 3, length: 4 };
//! assert_eq!(s1.view_ref(optics!(height)), &1);
//! assert_eq!(s2.view_ref(optics!(height)), &3);
//!
//! assert_eq!(s1.preview_ref(optics!(width)), Some(&2));
//! assert_eq!(s2.preview_ref(optics!(width)), None);
//! ```
//!
//! # Limitations
//!
//! * can't derive `Lens` for enum.
//! * can't derive `Prism` and `Review` for the variant has more than one argument or has named field.
//!
//! # License
//!
//! Under Apache License 2.0 or MIT License, at your will.

/// definitions of optics (including generated optic)
pub mod optics;

/// definitions of optics traits
pub mod traits;

pub use traits::{lens::*, prism::*, review::*, traversal::*};

/// build-in optics
pub use optics::{
    _both, _box, _ix, _mapped, _mut, _ref, _0, _1, _10, _11, _12, _13, _14, _15, _16, _2, _3, _4,
    _5, _6, _7, _8, _9, __,
};

/// derive macro
pub use lens_rs_derive::{Lens, Prism, Review};

/// macro to compose optics
///
/// ```ignore
/// let optics: Optics![a.Some.[0]._0] = optics!(a.Some.[0]._0);
/// // equivalent to optics!(lens_rs::optics::a.lens_rs::optics::Some.[0].lens_rs::optics::_0)
/// // the default optics path is `lens_rs::optics`.
/// ```
pub use lens_rs_derive::{optics, Optics};
