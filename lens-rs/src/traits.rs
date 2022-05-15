/// # Review
///
/// A trait representing the optics describes how to construct a single value.
///
/// ## Example
/// ```ignore
/// use lens_rs::*;
/// let nested: Result<Result<(), _>, ()> = Review::review(optics!(Ok.Err.Some), (1,2,3));
/// assert_eq!(nested, Ok(Err(Some((1,2,3)))));
/// ```
///
pub mod review {
    pub trait Review<Opt, Image> {
        fn review(optics: Opt, from: Image) -> Self;
    }
}

/// # Traversal
///
/// A trait representing the optics allows you to traverse over a structure and change out its contents.
/// A `TraversalXX` can access the multiple substructures.
///
/// ## Example
/// ```ignore
/// use lens_rs::*;
/// let mut x = (1, vec![Some((2, 3)), None]);
/// x.traverse_mut(optics!(_1._mapped._Some._0))
///     .into_iter()
///     .for_each(|i| *i += 1);
/// assert_eq!(x.traverse(optics!(_1._mapped.Some._0)), vec![3]);
/// ```
pub mod traversal {
    /// the immutable version of Traversal
    pub trait TraversalRef<Opt, Image: ?Sized> {
        fn traverse_ref(&self, optics: Opt) -> Vec<&Image>;
    }

    /// the mutable version of Traversal
    pub trait TraversalMut<Optics, Image: ?Sized>: TraversalRef<Optics, Image> {
        fn traverse_mut(&mut self, optics: Optics) -> Vec<&mut Image>;
    }

    /// the movable version of Traversal
    pub trait Traversal<Optics, Image>: TraversalMut<Optics, Image> {
        fn traverse(self, optics: Optics) -> Vec<Image>
        where
            Self: Sized;
    }
}

/// # Prism
///
/// Traits representing the optics behaves as the first-class pattern.
/// A `PrismXX` can access the substructure may exist.
///
/// ```ignore
/// use lens_rs::*;
/// let mut x: (_, Result<_, ()>) = (1, Ok((2, 3)));
/// *x.preview_mut(optics!(_1.Ok._1))? *= 2;
/// assert_eq!(x.preview(optics!(_1.Ok._1))?, 6);
/// ```
///
pub mod prism {
    use crate::*;
    /// the immutable version of Prism
    pub trait PrismRef<Optics, Image: ?Sized>: TraversalRef<Optics, Image> {
        fn preview_ref(&self, optics: Optics) -> Option<&Image>;
    }

    /// the mutable version of Prism
    pub trait PrismMut<Optics, Image: ?Sized>:
        PrismRef<Optics, Image> + TraversalMut<Optics, Image>
    {
        fn preview_mut(&mut self, optics: Optics) -> Option<&mut Image>;
    }

    /// the movable version of Prism
    pub trait Prism<Optics, Image>: PrismMut<Optics, Image> + Traversal<Optics, Image> {
        fn preview(self, optics: Optics) -> Option<Image>
        where
            Self: Sized;
    }
}

/// # Lens
///
/// Traits representing the optics allows you to access the field.
/// A `LensXX` can access the substructure must exist.
///
/// ## Example
/// ```ignore
/// use lens_rs::*;
/// let mut x = (1, (2, (3, 4)));
/// *x.view_mut(optics!(_1._1._1)) *= 2;
/// assert_eq!(x.view(optics!(_1._1._1)), 8);
/// ```
///
pub(crate) mod lens {
    use crate::*;

    /// the immutable version of Lens
    pub trait LensRef<Optics, Image: ?Sized>: PrismRef<Optics, Image> {
        fn view_ref(&self, optics: Optics) -> &Image;
    }

    /// the mutable version of Lens
    pub trait LensMut<Optics, Image: ?Sized>:
        LensRef<Optics, Image> + PrismMut<Optics, Image>
    {
        fn view_mut(&mut self, optics: Optics) -> &mut Image;
    }

    /// the movable version of Lens
    pub trait Lens<Optics, Image>: LensMut<Optics, Image> + Prism<Optics, Image> {
        fn view(self, optics: Optics) -> Image
        where
            Self: Sized;
    }
}
