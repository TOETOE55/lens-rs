/// the basic Optic trait.
pub trait Optic<Opt> {
    type Image: ?Sized;
}

/// # Review
///
/// A trait representing the optics describes how to construct a single value.
///
/// ## Example
/// ```
/// use lens_rs::*;
/// let nested: Result<Result<(), _>, ()> = Review::review(optics!(Ok.Err.Some), (1,2,3));
/// assert_eq!(nested, Ok(Err(Some((1,2,3)))));
/// ```
///
pub mod review {
    use super::*;
    pub trait Review<Opt>: Optic<Opt> {
        fn review(optics: Opt, from: Self::Image) -> Self
        where
            Self::Image: Sized;
    }
}

/// # Traversal
///
/// A trait representing the optics allows you to traverse over a structure and change out its contents.
/// A `TraversalXX` can access the multiple substructures.
///
/// ## Example
/// ```
/// use lens_rs::*;
/// let mut x = (1, vec![Some((2, 3)), None]);
/// x.traverse_mut(optics!(_1.Mapped._Some._0))
///     .into_iter()
///     .for_each(|i| *i += 1);
/// assert_eq!(x.traverse(optics!(_1._mapped.Some._0)), vec![3]);
/// ```
pub mod traversal {
    use super::*;

    /// the immutable version of Traversal
    pub trait TraversalRef<Opt>: Optic<Opt> {
        fn traverse_ref(&self, optics: Opt) -> Vec<&Self::Image>;
    }

    /// the mutable version of Traversal
    pub trait TraversalMut<Optics>: TraversalRef<Optics> {
        fn traverse_mut(&mut self, optics: Optics) -> Vec<&mut Self::Image>;
    }

    /// the movable version of Traversal
    pub trait Traversal<Optics>: TraversalMut<Optics> {
        fn traverse(self, optics: Optics) -> Vec<Self::Image>
        where
            Self: Sized,
            Self::Image: Sized;
    }
}

/// # Prism
///
/// Traits representing the optics behaves as the first-class pattern.
/// A `PrismXX` can access the substructure may exist.
///
/// ```
/// use lens_rs::*;
/// let mut x: (_, Result<_, ()>) = (1, Ok((2, 3)));
/// *x.preview_mut(optics!(_1.Ok._1))? *= 2;
/// assert_eq!(x.preview(optics!(_1.Ok._1))?, 6);
/// ```
///
pub mod prism {
    use crate::*;
    /// the immutable version of Prism
    pub trait PrismRef<Optics>: TraversalRef<Optics> {
        fn preview_ref(&self, optics: Optics) -> Option<&Self::Image>;
    }

    /// the mutable version of Prism
    pub trait PrismMut<Optics>: PrismRef<Optics> + TraversalMut<Optics> {
        fn preview_mut(&mut self, optics: Optics) -> Option<&mut Self::Image>;
    }

    /// the movable version of Prism
    pub trait Prism<Optics>: PrismMut<Optics> + Traversal<Optics> {
        fn preview(self, optics: Optics) -> Option<Self::Image>
        where
            Self: Sized,
            Self::Image: Sized;
    }
}

/// # Lens
///
/// Traits representing the optics allows you to access the field.
/// A `LensXX` can access the substructure must exist.
///
/// ## Example
/// ```
/// use lens_rs::*;
/// let mut x = (1, (2, (3, 4)));
/// *x.view_mut(optics!(_1._1._1)) *= 2;
/// assert_eq!(x.view(optics!(_1._1._1)), 8);
/// ```
///
pub(crate) mod lens {
    use crate::*;

    /// the immutable version of Lens
    pub trait LensRef<Optics>: PrismRef<Optics> {
        fn view_ref(&self, optics: Optics) -> &Self::Image;
    }

    /// the mutable version of Lens
    pub trait LensMut<Optics>: LensRef<Optics> + PrismMut<Optics> {
        fn view_mut(&mut self, optics: Optics) -> &mut Self::Image;
    }

    /// the movable version of Lens
    pub trait Lens<Optics>: LensMut<Optics> + Prism<Optics> {
        fn view(self, optics: Optics) -> Self::Image
        where
            Self: Sized,
            Self::Image: Sized;
    }
}
