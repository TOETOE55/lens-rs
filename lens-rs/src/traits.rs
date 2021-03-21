/**
A trait representing the optics describes how to construct a single value.
## Example
```
use lens_rs::*;
let optic = optics!(_Ok._Err._Some);
let nested: Result<Result<(), _>, ()> = optic.review((1,2,3));
assert_eq!(nested, Ok(Err(Some((1,2,3)))));
```
*/
pub trait Review<T> {
    type From;
    fn review(&self, from: Self::From) -> T;
}

/**
A trait representing the optics allows you to traverse over a structure and change out its contents.
A `Traversal` can access the multiple substructures.
## Example
```
let mut x = (1, vec![Some((2, 3)), None]);
optics!(_1.Mapped._Some._0)
    .traverse_mut(&mut x)
    .into_iter()
    .for_each(|i| *i += 1);
assert_eq!(optics!(_1.Mapped._Some._0).traverse(x), vec![3]);
```
*/
pub trait TraversalRef<T> {
    type To;
    fn traverse_ref<'a>(&self, source: &'a T) -> Vec<&'a Self::To>;
}

pub trait TraversalMut<T> {
    type To;
    fn traverse_mut<'a>(&self, source: &'a mut T) -> Vec<&'a mut Self::To>;
}

pub trait Traversal<T> {
    type To;
    fn traverse(&self, source: T) -> Vec<Self::To>;
}

/**
A trait representing the optics behaves as the first-class pattern.
A `Prism` can access the substructure may exist.
```
let mut x: (_, Result<_, ()>) = (1, Ok((2, 3)));
*optics!(_1._Ok._1).pm_mut(&mut x)? *= 2;
assert_eq!(optics!(_1._Ok._1).pm(x)?, 6);
```
*/
pub trait PrismRef<T> {
    type To;
    fn pm_ref<'a>(&self, source: &'a T) -> Option<&'a Self::To>;
}

pub trait PrismMut<T> {
    type To;
    fn pm_mut<'a>(&self, source: &'a mut T) -> Option<&'a mut Self::To>;
}

pub trait Prism<T> {
    type To;
    fn pm(&self, source: T) -> Option<Self::To>;
}

/**
A trait representing the optics allows you to access the field.
A `Lens` can access the substructure must exist.
## Example
```
let mut x = (1, (2, (3, 4)));
*optics!(_1._1._1).view_mut(&mut x) *= 2;
assert_eq!(optics!(_1._1._1).view(x), 8);
```
*/
pub trait LensRef<T> {
    type To;
    fn view_ref<'a>(&self, source: &'a T) -> &'a Self::To;
}

pub trait LensMut<T> {
    type To;
    fn view_mut<'a>(&self, source: &'a mut T) -> &'a mut Self::To;
}

pub trait Lens<T> {
    type To;
    fn view(&self, source: T) -> Self::To;
}

// pub trait LensH<T> {
//     type To;
//     type ToRef<'a>;
//     type ToMut<'a>;
//     fn view(&self, source: T) -> Self::To;
//     fn view_ref<'a>(&self, source: &'a T) -> Self::ToRef<'a>;
//     fn view_mut<'a>(&self, source: &'a mut T) -> Self::ToMut<'a>;
// }
