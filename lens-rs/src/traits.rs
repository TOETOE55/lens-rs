pub trait Optic<Opt> {
    type Image: ?Sized;
}

/**
* A trait representing the optics describes how to construct a single value.
* ## Example
* ```
* use lens_rs::*;
* let nested: Result<Result<(), _>, ()> = Review::review(optics!(Ok.Err.Some), (1,2,3));
* assert_eq!(nested, Ok(Err(Some((1,2,3)))));
* ```
*/
pub trait Review<Opt>: Optic<Opt> {
    fn review(optics: Opt, from: Self::Image) -> Self
    where
        Self::Image: Sized;
}

/**
A trait representing the optics allows you to traverse over a structure and change out its contents.
A `Traversal` can access the multiple substructures.
## Example
```
use lens_rs::*;
let mut x = (1, vec![Some((2, 3)), None]);
x.traverse_mut(optics!(_1.Mapped._Some._0))
    .into_iter()
    .for_each(|i| *i += 1);
assert_eq!(x.traverse(optics!(_1._mapped.Some._0)), vec![3]);
```
*/
pub trait TraversalRef<Opt>: Optic<Opt> {
    fn traverse_ref(&self, optics: Opt) -> Vec<&Self::Image>;
}

pub trait TraversalMut<Optics>: TraversalRef<Optics> {
    fn traverse_mut(&mut self, optics: Optics) -> Vec<&mut Self::Image>;
}

pub trait Traversal<Optics>: TraversalMut<Optics> {
    fn traverse(self, optics: Optics) -> Vec<Self::Image>
    where
        Self: Sized,
        Self::Image: Sized;
}

/**
A trait representing the optics behaves as the first-class pattern.
A `Prism` can access the substructure may exist.
```
use lens_rs::*;
let mut x: (_, Result<_, ()>) = (1, Ok((2, 3)));
*x.preview_mut(optics!(_1.Ok._1))? *= 2;
assert_eq!(x.preview(optics!(_1.Ok._1))?, 6);
```
*/
pub trait PrismRef<Optics>: TraversalRef<Optics> {
    fn preview_ref(&self, optics: Optics) -> Option<&Self::Image>;
}

pub trait PrismMut<Optics>: PrismRef<Optics> + TraversalMut<Optics> {
    fn preview_mut(&mut self, optics: Optics) -> Option<&mut Self::Image>;
}

pub trait Prism<Optics>: PrismMut<Optics> + Traversal<Optics> {
    fn preview(self, optics: Optics) -> Option<Self::Image>
    where
        Self: Sized,
        Self::Image: Sized;
}

/**
A trait representing the optics allows you to access the field.
A `Lens` can access the substructure must exist.
## Example
```
use lens_rs::*;
let mut x = (1, (2, (3, 4)));
*x.view_mut(optics!(_1._1._1)) *= 2;
assert_eq!(x.view(optics!(_1._1._1)), 8);
```
*/
pub trait LensRef<Optics>: PrismRef<Optics> {
    fn view_ref(&self, optics: Optics) -> &Self::Image;
}

pub trait LensMut<Optics>: LensRef<Optics> + PrismMut<Optics> {
    fn view_mut(&mut self, optics: Optics) -> &mut Self::Image;
}

pub trait Lens<Optics>: LensMut<Optics> + Prism<Optics> {
    fn view(self, optics: Optics) -> Self::Image
    where
        Self: Sized,
        Self::Image: Sized;
}
