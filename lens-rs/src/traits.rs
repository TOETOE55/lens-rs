use std::iter::FromIterator;

/**
* A trait representing the optics describes how to construct a single value.
* ## Example
* ```
* use lens_rs::*;
* let optic = optics!(Ok.Err.Some);
* let nested: Result<Result<(), _>, ()> = optic.review((1,2,3));
* assert_eq!(nested, Ok(Err(Some((1,2,3)))));
* ```
*/
pub trait Review<T> {
    type From;
    fn review<F: Into<Self::From>>(&self, from: F) -> T;
}

pub trait Fold<T>: Review<T> {
    type From;
    fn fold<F: IntoIterator<Item=Self::From>>(&self, from: F) -> T;
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
assert_eq!(optics!(_1._mapped.Some._0).traverse(x), vec![3]);
```
*/
pub trait TraversalRef<T> {
    type To;
    fn traverse_ref<'a, F: FromIterator<&'a Self::To>>(&self, source: &'a T) -> F
    where
        Self::To: 'a;
}

pub trait TraversalMut<T>: TraversalRef<T> {
    fn traverse_mut<'a, F: FromIterator<&'a mut Self::To>>(&self, source: &'a mut T) -> F
    where
        Self::To: 'a;
}

pub trait Traversal<T>: TraversalMut<T> {
    fn traverse(&self, source: T) -> Vec<Self::To>;
}

/**
A trait representing the optics behaves as the first-class pattern.
A `Prism` can access the substructure may exist.
```
let mut x: (_, Result<_, ()>) = (1, Ok((2, 3)));
*optics!(_1.Ok._1).pm_mut(&mut x)? *= 2;
assert_eq!(optics!(_1.Ok._1).pm(x)?, 6);
```
*/
pub trait PrismRef<T>: TraversalRef<T> {
    fn pm_ref<'a, F: From<Option<&'a Self::To>>>(&self, source: &'a T) -> F
    where
        Self::To: 'a;
}

pub trait PrismMut<T>: PrismRef<T> + TraversalMut<T> {
    fn pm_mut<'a, F: From<Option<&'a mut Self::To>>>(&self, source: &'a mut T) -> F
    where
        Self::To: 'a;
}

pub trait Prism<T>: PrismMut<T> + Traversal<T> {
    fn pm<F: From<Option<Self::To>>>(&self, source: T) -> F;
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
pub trait LensRef<T>: PrismRef<T> {
    fn view_ref<'a, F: From<&'a Self::To>>(&self, source: &'a T) -> F
    where
        Self::To: 'a;
}

pub trait LensMut<T>: LensRef<T> + PrismMut<T> {
    fn view_mut<'a, F: From<&'a mut Self::To>>(&self, source: &'a mut T) -> F
    where
        Self::To: 'a;
}

pub trait Lens<T>: LensMut<T> + Prism<T> {
    fn view<F: From<Self::To>>(&self, source: T) -> F;
}
