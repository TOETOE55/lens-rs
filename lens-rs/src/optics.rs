
/// the basic optic, behave as the identity functor
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct __;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ok<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Err<Optic>(pub Optic);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Some<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct None<Optic>(pub Optic);

/// behaves as `x.0`, implemented `Lens`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _0<Optic>(pub Optic);
/// behaves as `x.1`, implemented `Lens`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _1<Optic>(pub Optic);
/// behaves as `x.2`, implemented `Lens`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _2<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _3<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _4<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _5<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _6<Optic>(pub Optic);

/// to traverse all fields of tuples(0~6), implemented `Traversal`
///
/// ```rust
/// assert_eq!((1, 2).view(optics!(_both)), vec![1, 2])
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _both<Optic>(pub Optic);

/// to traverse items of collections, implemented `Traversal`
///
/// ```rust
/// assert_eq!(vec![vec![1,2], vec![3,4]].traverse(_mapped._mapped), vec![1, 2, 3, 4])
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _mapped<Optic>(pub Optic);

/// to create `Box`, `Rc` or `Arc` pointers, implemented `Review`
///
/// ```rust
/// assert_eq!(Review::review(optics!(_box), 0), Box::new(0));
/// assert_eq!(Review::review(optics!(_box), 0), Rc::new(0));
/// assert_eq!(Review::review(optics!(_box), 0), Arc::new(0));
/// ```
///
/// or to visit the data in `Box`, implemented `Lens`
///
/// ```rust
/// assert_eq!(Box::new(0).view(optic!(_box)), 0);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _box<Optic>(pub Optic);

/// to visit the data in pointers, implemented `LensRef`
///
/// ```rust
/// assert_eq!(Box::new(0).view_ref(optic!(_ref)), &0);
/// assert_eq!(Rc::new(0).view_ref(optic!(_ref)), &0);
///
/// let x = Foo {
///     a: &(1, 2),
///     b: ()
/// };
/// assert_eq!(x.view_ref(optics!(a._ref._1)), &2)
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _ref<Optic>(pub Optic);
/// to visit the data in pointers, implemented `LensMut`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _mut<Optic>(pub Optic);

/// behave as `xs[index]`
///
/// ```rust
/// assert_eq!(vec![1,2,3].view_ref(optics!([1])), &2);
/// assert_eq!(vec![1,2,3].view_ref(optics!([1..])), &[2, 3]);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _ix<I, Optic>(pub Optic, pub I);

//impls
mod impl__ {
    /***********************************************************
     * impl for __
     ************************************************************/
    use crate::*;

    impl<T> Review<__, T> for T {
        fn review(_optics: __, from: T) -> Self {
            from
        }
    }

    impl<T: ?Sized> TraversalRef<__, T> for T {
        fn traverse_ref(&self, _optics: __) -> Vec<&T> {
            vec![self]
        }
    }

    impl<T: ?Sized> TraversalMut<__, T> for T {
        fn traverse_mut(&mut self, _optics: __) -> Vec<&mut T> {
            vec![self]
        }
    }

    impl<T> Traversal<__, T> for T {
        fn traverse(self, _optics: __) -> Vec<T>
        where
            Self: Sized,
        {
            vec![self]
        }
    }

    impl<T: ?Sized> PrismRef<__, T> for T {
        fn preview_ref(&self, _optics: __) -> Option<&T> {
            Option::Some(self)
        }
    }

    impl<T: ?Sized> PrismMut<__, T> for T {
        fn preview_mut(&mut self, _optics: __) -> Option<&mut T> {
            Option::Some(self)
        }
    }

    impl<T> Prism<__, T> for T {
        fn preview(self, _: __) -> Option<T>
        where
            Self: Sized,
        {
            Option::Some(self)
        }
    }

    impl<T: ?Sized> LensRef<__, T> for T {
        fn view_ref(&self, _optics: __) -> &T {
            self
        }
    }

    impl<T: ?Sized> LensMut<__, T> for T {
        fn view_mut(&mut self, _optics: __) -> &mut T {
            self
        }
    }

    impl<T> Lens<__, T> for T {
        fn view(self, _optics: __) -> T
        where
            Self: Sized,
        {
            self
        }
    }
}

mod impl_result {
    /***********************************************************
     * impl for Result
     ************************************************************/
    use crate::*;

    impl<Rv, Image, T, E> Review<optics::Ok<Rv>, Image> for Result<T, E>
    where
        T: Review<Rv, Image>,
    {
        fn review(optics: optics::Ok<Rv>, from: Image) -> Self {
            Result::Ok(Review::review(optics.0, from))
        }
    }

    impl<Tr, Image: ?Sized, T, E> TraversalRef<optics::Ok<Tr>, Image> for Result<T, E>
    where
        T: TraversalRef<Tr, Image>,
    {
        fn traverse_ref(&self, optics: optics::Ok<Tr>) -> Vec<&Image> {
            match self {
                Result::Ok(t) => t.traverse_ref(optics.0),
                _ => vec![],
            }
        }
    }

    impl<Tr, Image: ?Sized, T, E> TraversalMut<optics::Ok<Tr>, Image> for Result<T, E>
    where
        T: TraversalMut<Tr, Image>,
    {
        fn traverse_mut(&mut self, optics: optics::Ok<Tr>) -> Vec<&mut Image> {
            match self {
                Result::Ok(t) => t.traverse_mut(optics.0),
                _ => vec![],
            }
        }
    }

    impl<Tr, Image, T, E> Traversal<optics::Ok<Tr>, Image> for Result<T, E>
    where
        T: Traversal<Tr, Image>,
    {
        fn traverse(self, optics: optics::Ok<Tr>) -> Vec<Image>
        where
            Self: Sized,
        {
            match self {
                Result::Ok(t) => t.traverse(optics.0),
                _ => vec![],
            }
        }
    }

    impl<Tr, Image, T, E> Traversal<optics::Err<Tr>, Image> for Result<T, E>
    where
        E: Traversal<Tr, Image>,
    {
        fn traverse(self, optics: optics::Err<Tr>) -> Vec<Image>
        where
            Self: Sized,
        {
            match self {
                Result::Err(e) => e.traverse(optics.0),
                _ => vec![],
            }
        }
    }

    impl<Tr, Image: ?Sized, T, E> TraversalRef<optics::Err<Tr>, Image> for Result<T, E>
    where
        E: TraversalRef<Tr, Image>,
    {
        fn traverse_ref(&self, optics: optics::Err<Tr>) -> Vec<&Image> {
            match self {
                Result::Err(e) => e.traverse_ref(optics.0),
                _ => vec![],
            }
        }
    }

    impl<Tr, Image: ?Sized, T, E> TraversalMut<optics::Err<Tr>, Image> for Result<T, E>
    where
        E: TraversalMut<Tr, Image>,
    {
        fn traverse_mut(&mut self, optics: optics::Err<Tr>) -> Vec<&mut Image> {
            match self {
                Result::Err(e) => e.traverse_mut(optics.0),
                _ => vec![],
            }
        }
    }

    impl<Pm, Image: ?Sized, T, E> PrismRef<optics::Ok<Pm>, Image> for Result<T, E>
    where
        T: PrismRef<Pm, Image>,
    {
        fn preview_ref(&self, optics: optics::Ok<Pm>) -> Option<&Image> {
            let t = self.as_ref().ok()?;
            t.preview_ref(optics.0)
        }
    }

    impl<Pm, Image: ?Sized, T, E> PrismMut<optics::Ok<Pm>, Image> for Result<T, E>
    where
        T: PrismMut<Pm, Image>,
    {
        fn preview_mut(&mut self, optics: optics::Ok<Pm>) -> Option<&mut Image> {
            let t = self.as_mut().ok()?;
            t.preview_mut(optics.0)
        }
    }

    impl<Pm, Image, T, E> Prism<optics::Ok<Pm>, Image> for Result<T, E>
    where
        T: Prism<Pm, Image>,
    {
        fn preview(self, optics: optics::Ok<Pm>) -> Option<Image>
        where
            Self: Sized,
        {
            let t = self.ok()?;
            t.preview(optics.0)
        }
    }

    impl<Rv, Image, T, E> Review<optics::Err<Rv>, Image> for Result<T, E>
    where
        E: Review<Rv, Image>,
    {
        fn review(optics: optics::Err<Rv>, from: Image) -> Self {
            Result::Err(Review::review(optics.0, from))
        }
    }

    impl<Pm, Image: ?Sized, T, E> PrismRef<optics::Err<Pm>, Image> for Result<T, E>
    where
        E: PrismRef<Pm, Image>,
    {
        fn preview_ref(&self, optics: optics::Err<Pm>) -> Option<&Image> {
            let t = self.as_ref().err()?;
            t.preview_ref(optics.0)
        }
    }

    impl<Pm, Image: ?Sized, T, E> PrismMut<optics::Err<Pm>, Image> for Result<T, E>
    where
        E: PrismMut<Pm, Image>,
    {
        fn preview_mut(&mut self, optics: optics::Err<Pm>) -> Option<&mut Image> {
            let t = self.as_mut().err()?;
            t.preview_mut(optics.0)
        }
    }

    impl<Pm, Image, T, E> Prism<optics::Err<Pm>, Image> for Result<T, E>
    where
        E: Prism<Pm, Image>,
    {
        fn preview(self, optics: optics::Err<Pm>) -> Option<Image>
        where
            Self: Sized,
        {
            let t = self.err()?;
            t.preview(optics.0)
        }
    }
}

mod impl_some {
    /***********************************************************
     * impl for Option
     ************************************************************/

    use crate::*;

    impl<Rv, Image, T> Review<optics::Some<Rv>, Image> for Option<T>
    where
        T: Review<Rv, Image>,
    {
        fn review(optics: optics::Some<Rv>, from: Image) -> Self {
            Some(Review::review(optics.0, from))
        }
    }

    impl<Tr, Image: ?Sized, T> TraversalRef<optics::Some<Tr>, Image> for Option<T>
    where
        T: TraversalRef<Tr, Image>,
    {
        fn traverse_ref(&self, optics: optics::Some<Tr>) -> Vec<&Image> {
            match self {
                Some(t) => t.traverse_ref(optics.0),
                _ => vec![],
            }
        }
    }

    impl<Tr, Image: ?Sized, T> TraversalMut<optics::Some<Tr>, Image> for Option<T>
    where
        T: TraversalMut<Tr, Image>,
    {
        fn traverse_mut(&mut self, optics: optics::Some<Tr>) -> Vec<&mut Image> {
            match self {
                Some(t) => t.traverse_mut(optics.0),
                _ => vec![],
            }
        }
    }

    impl<Tr, Image, T> Traversal<optics::Some<Tr>, Image> for Option<T>
    where
        T: Traversal<Tr, Image>,
    {
        fn traverse(self, optics: optics::Some<Tr>) -> Vec<Image>
        where
            Self: Sized,
        {
            match self {
                Some(t) => t.traverse(optics.0),
                _ => vec![],
            }
        }
    }

    impl<Pm, Image: ?Sized, T> PrismRef<optics::Some<Pm>, Image> for Option<T>
    where
        T: PrismRef<Pm, Image>,
    {
        fn preview_ref(&self, optics: optics::Some<Pm>) -> Option<&Image> {
            self.as_ref()?.preview_ref(optics.0)
        }
    }

    impl<Pm, Image: ?Sized, T> PrismMut<optics::Some<Pm>, Image> for Option<T>
    where
        T: PrismMut<Pm, Image>,
    {
        fn preview_mut(&mut self, optics: optics::Some<Pm>) -> Option<&mut Image> {
            self.as_mut()?.preview_mut(optics.0)
        }
    }

    impl<Pm, Image, T> Prism<optics::Some<Pm>, Image> for Option<T>
    where
        T: Prism<Pm, Image>,
    {
        fn preview(self, optics: optics::Some<Pm>) -> Option<Image>
        where
            Self: Sized,
        {
            self?.preview(optics.0)
        }
    }

    impl<Tr, Image: ?Sized, T> TraversalRef<optics::None<Tr>, Image> for Option<T> {
        fn traverse_ref(&self, _optics: optics::None<Tr>) -> Vec<&Image> {
            vec![]
        }
    }

    impl<Tr, Image: ?Sized, T> TraversalMut<optics::None<Tr>, Image> for Option<T> {
        fn traverse_mut(&mut self, _optics: optics::None<Tr>) -> Vec<&mut Image> {
            vec![]
        }
    }

    impl<Tr, Image, T> Traversal<optics::None<Tr>, Image> for Option<T> {
        fn traverse(self, _optics: optics::None<Tr>) -> Vec<Image> where
            Self: Sized {
            vec![]
        }
    }

    impl<Pm, Image: ?Sized, T> PrismRef<optics::None<Pm>, Image> for Option<T> {
        fn preview_ref(&self, _optics: optics::None<Pm>) -> Option<&Image> {
            None
        }
    }

    impl<Pm, Image: ?Sized, T> PrismMut<optics::None<Pm>, Image> for Option<T> {
        fn preview_mut(&mut self, _optics: optics::None<Pm>) -> Option<&mut Image> {
            None
        }
    }

    impl<Pm, Image, T> Prism<optics::None<Pm>, Image> for Option<T> {
        fn preview(self, _optics: optics::None<Pm>) -> Option<Image> where
            Self: Sized {
            None
        }
    }

    impl<Rv, Image, T> Review<optics::None<Rv>, Image> for Option<T> {
        fn review(_optics: optics::None<Rv>, _from: Image) -> Self {
            None
        }
    }
}

// mod impl_tuples {
//
//     /***********************************************************
//      * impl for tuple
//      ************************************************************/
//     use crate::*;
//
//     macro_rules! impl_tuple {
//         ({$($param:ident)*}, $field:tt, $optic:ident, $to:ident) => {
//             impl<Opt, $($param,)*> Optic<$optic<Opt>> for ($($param,)*)
//             where
//                 $to: Optic<Opt>,
//             {
//                 type Image = $to::Image;
//             }
//
//             impl<Tr, $($param,)*> TraversalRef<$optic<Tr>> for ($($param,)*)
//             where
//                 $to: TraversalRef<Tr>,
//             {
//
//                 fn traverse_ref(&self, optics: $optic<Tr>) -> Vec<&Self::Image> {
//                     self.$field.traverse_ref(optics.0)
//                 }
//             }
//
//             impl<Tr, $($param,)*> TraversalMut<$optic<Tr>> for ($($param,)*)
//             where
//                 $to: TraversalMut<Tr>,
//             {
//                 fn traverse_mut(&mut self, optics: $optic<Tr>) -> Vec<&mut Self::Image> {
//                     self.$field.traverse_mut(optics.0)
//                 }
//             }
//
//             impl<Tr, $($param,)*> Traversal<$optic<Tr>> for ($($param,)*)
//             where
//                 $to: Traversal<Tr>,
//             {
//                 fn traverse(self, optics: $optic<Tr>) -> Vec<Self::Image>
//                 where Self: Sized,
//                     Self::Image: Sized,
//                 {
//                     self.$field.traverse(optics.0)
//                 }
//             }
//
//             impl<Pm, $($param,)*> PrismRef<$optic<Pm>> for ($($param,)*)
//             where
//                 $to: PrismRef<Pm>,
//             {
//                 fn preview_ref(&self, optics: $optic<Pm>) -> Option<&Self::Image> {
//                     self.$field.preview_ref(optics.0)
//                 }
//             }
//
//             impl<Pm, $($param,)*> PrismMut<$optic<Pm>> for ($($param,)*)
//             where
//                 $to: PrismMut<Pm>,
//             {
//                 fn preview_mut(&mut self, optics: $optic<Pm>) -> Option<&mut Self::Image> {
//                     self.$field.preview_mut(optics.0)
//                 }
//             }
//
//             impl<Pm, $($param,)*> Prism<$optic<Pm>> for ($($param,)*)
//             where
//                 $to: Prism<Pm>,
//             {
//                 fn preview(self, optics: $optic<Pm>) -> Option<Self::Image>
//                 where Self: Sized,
//                     Self::Image: Sized,
//                 {
//                     self.$field.preview(optics.0)
//                 }
//             }
//
//             impl<Ls, $($param,)*> LensRef<$optic<Ls>> for ($($param,)*)
//             where
//                 $to: LensRef<Ls>,
//             {
//                 fn view_ref(&self, optics: $optic<Ls>) -> &Self::Image {
//                     self.$field.view_ref(optics.0)
//                 }
//             }
//
//             impl<Ls, $($param,)*> LensMut<$optic<Ls>> for ($($param,)*)
//             where
//                 $to: LensMut<Ls>,
//             {
//                 fn view_mut(&mut self, optics: $optic<Ls>) -> &mut Self::Image {
//                     self.$field.view_mut(optics.0)
//                 }
//             }
//
//             impl<Ls, $($param,)*> Lens<$optic<Ls>> for ($($param,)*)
//             where
//                 $to: Lens<Ls>,
//             {
//                 fn view(self, optics: $optic<Ls>) -> Self::Image
//                 where Self: Sized,
//                     Self::Image: Sized,
//                 {
//                     self.$field.view(optics.0)
//                 }
//             }
//         }
//     }
//
//     impl_tuple!({ A }, 0, _0, A);
//
//     impl_tuple!({A B}, 0, _0, A);
//     impl_tuple!({A B}, 1, _1, B);
//
//     impl_tuple!({A B C}, 0, _0, A);
//     impl_tuple!({A B C}, 1, _1, B);
//     impl_tuple!({A B C}, 2, _2, C);
//
//     impl_tuple!({A B C D}, 0, _0, A);
//     impl_tuple!({A B C D}, 1, _1, B);
//     impl_tuple!({A B C D}, 2, _2, C);
//     impl_tuple!({A B C D}, 3, _3, D);
//
//     impl_tuple!({A B C D E}, 0, _0, A);
//     impl_tuple!({A B C D E}, 1, _1, B);
//     impl_tuple!({A B C D E}, 2, _2, C);
//     impl_tuple!({A B C D E}, 3, _3, D);
//     impl_tuple!({A B C D E}, 4, _4, E);
//
//     impl_tuple!({A B C D E F}, 0, _0, A);
//     impl_tuple!({A B C D E F}, 1, _1, B);
//     impl_tuple!({A B C D E F}, 2, _2, C);
//     impl_tuple!({A B C D E F}, 3, _3, D);
//     impl_tuple!({A B C D E F}, 4, _4, E);
//     impl_tuple!({A B C D E F}, 5, _5, F);
//
//     impl_tuple!({A B C D E F G}, 0, _0, A);
//     impl_tuple!({A B C D E F G}, 1, _1, B);
//     impl_tuple!({A B C D E F G}, 2, _2, C);
//     impl_tuple!({A B C D E F G}, 3, _3, D);
//     impl_tuple!({A B C D E F G}, 4, _4, E);
//     impl_tuple!({A B C D E F G}, 5, _5, F);
//     impl_tuple!({A B C D E F G}, 6, _6, G);
//
//     impl<Rv, A> Review<_0<Rv>> for (A,)
//     where
//         A: Review<Rv>,
//     {
//         fn review(optics: _0<Rv>, from: Self::Image) -> Self
//         where
//             Self::Image: Sized,
//         {
//             (Review::review(optics.0, from),)
//         }
//     }
//
//     macro_rules! impl_both {
//         (<$param:ident> $tuple:ty, $($fields:tt),*) => {
//             impl<Opt, $param> Optic<_both<Opt>> for $tuple
//             where
//                 $param: Optic<Opt>,
//             {
//                 type Image = $param::Image;
//             }
//
//             impl<Tr, $param> TraversalRef<_both<Tr>> for $tuple
//             where
//                 Tr: Clone,
//                 $param: TraversalRef<Tr>,
//             {
//
//                 fn traverse_ref(&self, optics: _both<Tr>) -> Vec<&Self::Image> {
//                     let mut vec = vec![];
//                     $(vec.append(&mut self.$fields.traverse_ref(optics.0.clone()));)*
//                     vec
//                 }
//             }
//
//             impl<Tr, $param> TraversalMut<_both<Tr>> for $tuple
//             where
//                 Tr: Clone,
//                 $param: TraversalMut<Tr>,
//             {
//                 fn traverse_mut(&mut self, optics: _both<Tr>) -> Vec<&mut Self::Image> {
//                     let mut vec = vec![];
//                     $(vec.append(&mut self.$fields.traverse_mut(optics.0.clone()));)*
//                     vec
//                 }
//             }
//
//             impl<Tr, $param> Traversal<_both<Tr>> for $tuple
//             where
//                 Tr: Clone,
//                 $param: Traversal<Tr>,
//             {
//                 fn traverse(self, optics: _both<Tr>) -> Vec<Self::Image>
//                 where Self: Sized,
//                     Self::Image: Sized,
//                 {
//                     let mut vec = vec![];
//                     $(vec.append(&mut self.$fields.traverse(optics.0.clone()));)*
//                     vec
//                 }
//             }
//         }
//     }
//
//     impl_both!(<A> (A,), 0);
//     impl_both!(<A> (A, A), 0, 1);
//     impl_both!(<A> (A, A, A), 0, 1, 2);
//     impl_both!(<A> (A, A, A, A), 0, 1, 2, 3);
//     impl_both!(<A> (A, A, A, A, A), 0, 1, 2, 3, 4);
//     impl_both!(<A> (A, A, A, A, A, A), 0, 1, 2, 3, 4, 5);
//     impl_both!(<A> (A, A, A, A, A, A, A), 0, 1, 2, 3, 4, 5, 6);
//
//     impl<Pm: Clone, A> PrismRef<_both<Pm>> for (A,)
//     where
//         A: PrismRef<Pm>,
//     {
//         fn preview_ref(&self, optics: _both<Pm>) -> Option<&Self::Image> {
//             self.0.preview_ref(optics.0)
//         }
//     }
//
//     impl<Pm: Clone, A> PrismMut<_both<Pm>> for (A,)
//     where
//         A: PrismMut<Pm>,
//     {
//         fn preview_mut(&mut self, optics: _both<Pm>) -> Option<&mut Self::Image> {
//             self.0.preview_mut(optics.0)
//         }
//     }
//
//     impl<Pm: Clone, A> Prism<_both<Pm>> for (A,)
//     where
//         A: Prism<Pm>,
//     {
//         fn preview(self, optics: _both<Pm>) -> Option<Self::Image>
//         where
//             Self: Sized,
//             Self::Image: Sized,
//         {
//             self.0.preview(optics.0)
//         }
//     }
//
//     impl<Ls: Clone, A> LensRef<_both<Ls>> for (A,)
//     where
//         A: LensRef<Ls>,
//     {
//         fn view_ref(&self, optics: _both<Ls>) -> &Self::Image {
//             self.0.view_ref(optics.0)
//         }
//     }
//
//     impl<Ls: Clone, A> LensMut<_both<Ls>> for (A,)
//     where
//         A: LensMut<Ls>,
//     {
//         fn view_mut(&mut self, optics: _both<Ls>) -> &mut Self::Image {
//             self.0.view_mut(optics.0)
//         }
//     }
//
//     impl<Ls: Clone, A> Lens<_both<Ls>> for (A,)
//     where
//         A: Lens<Ls>,
//     {
//         fn view(self, optics: _both<Ls>) -> Self::Image
//         where
//             Self: Sized,
//             Self::Image: Sized,
//         {
//             self.0.view(optics.0)
//         }
//     }
//
//     impl<Rv, A> Review<_both<Rv>> for (A,)
//     where
//         A: Review<Rv>,
//     {
//         fn review(optics: _both<Rv>, from: Self::Image) -> Self
//         where
//             Self::Image: Sized,
//         {
//             (Review::review(optics.0, from),)
//         }
//     }
// }
//
// mod impl_collect {
//     /***********************************************************
//      * impl for iter
//      ************************************************************/
//     use crate::*;
//     use std::collections::*;
//
//     macro_rules! impl_iter {
//         (<$item:ident> $collector:ty) => {
//             impl<Opt, $item> Optic<_mapped<Opt>> for $collector
//             where
//                 $item: Optic<Opt>,
//             {
//                 type Image = $item::Image;
//             }
//
//             impl<Tr: Clone, $item> TraversalRef<_mapped<Tr>> for $collector
//             where
//                 $item: TraversalRef<Tr>,
//             {
//                 fn traverse_ref(&self, optics: _mapped<Tr>) -> Vec<&Self::Image> {
//                     self.into_iter()
//                         .flat_map(|t| t.traverse_ref(optics.0.clone()))
//                         .collect()
//                 }
//             }
//
//             impl<Tr: Clone, $item> TraversalMut<_mapped<Tr>> for $collector
//             where
//                 $item: TraversalMut<Tr>,
//             {
//                 fn traverse_mut(&mut self, optics: _mapped<Tr>) -> Vec<&mut Self::Image> {
//                     self.into_iter()
//                         .flat_map(|t| t.traverse_mut(optics.0.clone()))
//                         .collect()
//                 }
//             }
//
//             impl<Tr: Clone, $item> Traversal<_mapped<Tr>> for $collector
//             where
//                 $item: Traversal<Tr>,
//             {
//                 fn traverse(self, optics: _mapped<Tr>) -> Vec<Self::Image>
//                 where
//                     Self: Sized,
//                     Self::Image: Sized,
//                 {
//                     self.into_iter()
//                         .flat_map(|t| t.traverse(optics.0.clone()))
//                         .collect()
//                 }
//             }
//         };
//     }
//
//     impl_iter!(<T> Vec<T>);
//     impl_iter!(<T> VecDeque<T>);
//     impl_iter!(<T> LinkedList<T>);
// }
//
// mod impl_ptr {
//     use crate::*;
//     use std::rc::Rc;
//     use std::sync::Arc;
//
//     macro_rules! impl_ref {
//         (<$param:ident> $ptr:ty, $optic:ident) => {
//             impl<Opt, $param: ?Sized> Optic<$optic<Opt>> for $ptr
//             where
//                 $param: Optic<Opt>,
//             {
//                 type Image = $param::Image;
//             }
//
//             impl<$param: ?Sized, Tr> TraversalRef<$optic<Tr>> for $ptr
//             where
//                 $param: TraversalRef<Tr>,
//             {
//                 fn traverse_ref(&self, optics: $optic<Tr>) -> Vec<&Self::Image> {
//                     (**self).traverse_ref(optics.0)
//                 }
//             }
//
//             impl<$param: ?Sized, Pm> PrismRef<$optic<Pm>> for $ptr
//             where
//                 $param: PrismRef<Pm>,
//             {
//                 fn preview_ref(&self, optics: $optic<Pm>) -> Option<&Self::Image> {
//                     (**self).preview_ref(optics.0)
//                 }
//             }
//
//             impl<$param: ?Sized, Ls> LensRef<$optic<Ls>> for $ptr
//             where
//                 $param: LensRef<Ls>,
//             {
//                 fn view_ref(&self, optics: $optic<Ls>) -> &Self::Image {
//                     (**self).view_ref(optics.0)
//                 }
//             }
//         };
//     }
//
//     macro_rules! impl_mut {
//         (<$param:ident> $ptr:ty, $optic:ident) => {
//             impl<$param: ?Sized, Tr> TraversalMut<$optic<Tr>> for $ptr
//             where
//                 $param: TraversalMut<Tr>,
//             {
//                 fn traverse_mut(&mut self, optics: $optic<Tr>) -> Vec<&mut Self::Image> {
//                     (**self).traverse_mut(optics.0)
//                 }
//             }
//
//             impl<$param: ?Sized, Pm> PrismMut<$optic<Pm>> for $ptr
//             where
//                 $param: PrismMut<Pm>,
//             {
//                 fn preview_mut(&mut self, optics: $optic<Pm>) -> Option<&mut Self::Image> {
//                     (**self).preview_mut(optics.0)
//                 }
//             }
//
//             impl<$param: ?Sized, Ls> LensMut<$optic<Ls>> for $ptr
//             where
//                 $param: LensMut<Ls>,
//             {
//                 fn view_mut(&mut self, optics: $optic<Ls>) -> &mut Self::Image {
//                     (**self).view_mut(optics.0)
//                 }
//             }
//         };
//     }
//
//     impl<Rv, T> Review<_box<Rv>> for Box<T>
//     where
//         T: Review<Rv>,
//     {
//         fn review(optics: _box<Rv>, from: Self::Image) -> Self
//         where
//             Self::Image: Sized,
//         {
//             Box::new(Review::review(optics.0, from))
//         }
//     }
//
//     impl<Tr, T> Traversal<_box<Tr>> for Box<T>
//     where
//         T: Traversal<Tr>,
//     {
//         fn traverse(self, optics: _box<Tr>) -> Vec<Self::Image>
//         where
//             Self: Sized,
//             Self::Image: Sized,
//         {
//             (*self).traverse(optics.0)
//         }
//     }
//
//     impl<Pm, T> Prism<_box<Pm>> for Box<T>
//     where
//         T: Prism<Pm>,
//     {
//         fn preview(self, optics: _box<Pm>) -> Option<Self::Image>
//         where
//             Self: Sized,
//             Self::Image: Sized,
//         {
//             (*self).preview(optics.0)
//         }
//     }
//
//     impl<Ls, T> Lens<_box<Ls>> for Box<T>
//     where
//         T: Lens<Ls>,
//     {
//         fn view(self, optics: _box<Ls>) -> Self::Image
//         where
//             Self: Sized,
//             Self::Image: Sized,
//         {
//             (*self).view(optics.0)
//         }
//     }
//
//     impl<Opt, T> Optic<_box<Opt>> for Rc<T>
//     where
//         T: Optic<Opt>,
//     {
//         type Image = T::Image;
//     }
//
//     impl<Rv, T> Review<_box<Rv>> for Rc<T>
//     where
//         T: Review<Rv>,
//     {
//         fn review(optics: _box<Rv>, from: Self::Image) -> Self
//         where
//             Self::Image: Sized,
//         {
//             Rc::new(Review::review(optics.0, from))
//         }
//     }
//
//     impl<Opt, T> Optic<_box<Opt>> for Arc<T>
//     where
//         T: Optic<Opt>,
//     {
//         type Image = T::Image;
//     }
//
//     impl<Rv, T> Review<_box<Rv>> for Arc<T>
//     where
//         T: Review<Rv>,
//     {
//         fn review(optics: _box<Rv>, from: Self::Image) -> Self
//         where
//             Self::Image: Sized,
//         {
//             Arc::new(Review::review(optics.0, from))
//         }
//     }
//
//     impl_ref!(<T> Box<T>, _ref);
//     impl_ref!(<T> Box<T>, _mut);
//     impl_ref!(<T> Box<T>, _box);
//     impl_ref!(<T> Rc<T>, _ref);
//     impl_ref!(<T> Arc<T>, _ref);
//     impl_ref!(<T> &'_ mut T, _ref);
//     impl_ref!(<T> &'_ mut T, _mut);
//     impl_ref!(<T> &'_ T, _ref);
//
//     impl_mut!(<T> Box<T>, _box);
//     impl_mut!(<T> Box<T>, _mut);
//     impl_mut!(<T> &'_ mut T, _mut);
// }
//
// mod impl_ix {
//     use crate::*;
//     use std::borrow::Borrow;
//     use std::collections::{BTreeMap, HashMap, VecDeque};
//     use std::hash::Hash;
//     use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
//
//     macro_rules! impl_ix {
//         (<$($param:ident,)? $(const $c:ident: $ct: ty)?> $t:ty[$ix:ty]: $o:ty) => {
//             impl<$($param,)? Opt, $(const $c: $ct)?> Optic<_ix<$ix, Opt>> for $t
//             where
//                 $o: Optic<Opt>,
//             {
//                 type Image = <$o as Optic<Opt>>::Image;
//             }
//
//             impl<$($param,)? Tr, $(const $c: $ct)?> TraversalRef<_ix<$ix, Tr>> for $t
//             where
//                 $o: TraversalRef<Tr>,
//             {
//
//                 fn traverse_ref(&self, optics: _ix<$ix, Tr>) -> Vec<&Self::Image> {
//                     self[optics.1].traverse_ref(optics.0)
//                 }
//             }
//
//             impl<$($param,)? Tr, $(const $c: $ct)?> TraversalMut<_ix<$ix, Tr>> for $t
//             where
//                 $o: TraversalMut<Tr>,
//             {
//
//                 fn traverse_mut(&mut self, optics: _ix<$ix, Tr>) -> Vec<&mut Self::Image> {
//                     self[optics.1].traverse_mut(optics.0)
//                 }
//             }
//
//             impl<$($param,)? Pm, $(const $c: $ct)?> PrismRef<_ix<$ix, Pm>> for $t
//             where
//                 $o: PrismRef<Pm>,
//             {
//                 fn preview_ref(&self, optics: _ix<$ix, Pm>) -> Option<&Self::Image> {
//                     self[optics.1].preview_ref(optics.0)
//                 }
//             }
//
//             impl<$($param,)? Pm, $(const $c: $ct)?> PrismMut<_ix<$ix, Pm>> for $t
//             where
//                 $o: PrismMut<Pm>,
//             {
//                 fn preview_mut(&mut self, optics: _ix<$ix, Pm>) -> Option<&mut Self::Image> {
//                     self[optics.1].preview_mut(optics.0)
//                 }
//             }
//
//             impl<$($param,)? Ls, $(const $c: $ct)?> LensRef<_ix<$ix, Ls>> for $t
//             where
//                 $o: LensRef<Ls>,
//             {
//                 fn view_ref(&self, optics: _ix<$ix, Ls>) -> &Self::Image {
//                     self[optics.1].view_ref(optics.0)
//                 }
//             }
//
//             impl<$($param,)? Pm, $(const $c: $ct)?> LensMut<_ix<$ix, Pm>> for $t
//             where
//                 $o: LensMut<Pm>,
//             {
//                 fn view_mut(&mut self, optics: _ix<$ix, Pm>) -> &mut Self::Image {
//                     self[optics.1].view_mut(optics.0)
//                 }
//             }
//         }
//     }
//
//     impl_ix!(<T,> Vec<T>[usize]: T);
//     impl_ix!(<T,> Vec<T>[Range<usize>]: [T]);
//     impl_ix!(<T,> Vec<T>[RangeTo<usize>]: [T]);
//     impl_ix!(<T,> Vec<T>[RangeFrom<usize>]: [T]);
//     impl_ix!(<T,> Vec<T>[RangeFull]: [T]);
//
//     impl_ix!(<T,> VecDeque<T>[usize]: T);
//
//     impl_ix!(<T, const N: usize> [T; N][usize]: T);
//     impl_ix!(<T, const N: usize> [T; N][Range<usize>]: [T]);
//     impl_ix!(<T, const N: usize> [T; N][RangeTo<usize>]: [T]);
//     impl_ix!(<T, const N: usize> [T; N][RangeFrom<usize>]: [T]);
//     impl_ix!(<T, const N: usize> [T; N][RangeFull]: [T]);
//
//     impl_ix!(<T,> [T][usize]: T);
//     impl_ix!(<T,> [T][Range<usize>]: [T]);
//     impl_ix!(<T,> [T][RangeTo<usize>]: [T]);
//     impl_ix!(<T,> [T][RangeFrom<usize>]: [T]);
//     impl_ix!(<T,> [T][RangeFull]: [T]);
//
//     impl_ix!(<> String[Range<usize>]: str);
//     impl_ix!(<> String[RangeTo<usize>]: str);
//     impl_ix!(<> String[RangeFrom<usize>]: str);
//     impl_ix!(<> String[RangeFull]: str);
//
//     impl_ix!(<> str[Range<usize>]: str);
//     impl_ix!(<> str[RangeTo<usize>]: str);
//     impl_ix!(<> str[RangeFrom<usize>]: str);
//     impl_ix!(<> str[RangeFull]: str);
//
//     impl<K, Q: ?Sized, V, Opt> Optic<_ix<&'_ Q, Opt>> for BTreeMap<K, V>
//     where
//         K: Borrow<Q> + Ord,
//         Q: Ord,
//         V: Optic<Opt>,
//     {
//         type Image = V::Image;
//     }
//
//     impl<K, Q: ?Sized, V, Opt> Optic<_ix<&'_ Q, Opt>> for HashMap<K, V>
//     where
//         K: Eq + Hash + Borrow<Q>,
//         Q: Eq + Hash,
//         V: Optic<Opt>,
//     {
//         type Image = V::Image;
//     }
//
//     impl<K, Q: ?Sized, V, Tr> TraversalRef<_ix<&'_ Q, Tr>> for BTreeMap<K, V>
//     where
//         K: Borrow<Q> + Ord,
//         Q: Ord,
//         V: TraversalRef<Tr>,
//     {
//         fn traverse_ref(&self, optics: _ix<&Q, Tr>) -> Vec<&Self::Image> {
//             self[optics.1].traverse_ref(optics.0)
//         }
//     }
//
//     impl<K, Q: ?Sized, V, Tr> TraversalRef<_ix<&'_ Q, Tr>> for HashMap<K, V>
//     where
//         K: Eq + Hash + Borrow<Q>,
//         Q: Eq + Hash,
//         V: TraversalRef<Tr>,
//     {
//         fn traverse_ref(&self, optics: _ix<&Q, Tr>) -> Vec<&Self::Image> {
//             self[optics.1].traverse_ref(optics.0)
//         }
//     }
//
//     impl<K, Q: ?Sized, V, Pm> PrismRef<_ix<&'_ Q, Pm>> for BTreeMap<K, V>
//     where
//         K: Borrow<Q> + Ord,
//         Q: Ord,
//         V: PrismRef<Pm>,
//     {
//         fn preview_ref(&self, optics: _ix<&Q, Pm>) -> Option<&Self::Image> {
//             self[optics.1].preview_ref(optics.0)
//         }
//     }
//
//     impl<K, Q: ?Sized, V, Pm> PrismRef<_ix<&'_ Q, Pm>> for HashMap<K, V>
//     where
//         K: Eq + Hash + Borrow<Q>,
//         Q: Eq + Hash,
//         V: PrismRef<Pm>,
//     {
//         fn preview_ref(&self, optics: _ix<&Q, Pm>) -> Option<&Self::Image> {
//             self[optics.1].preview_ref(optics.0)
//         }
//     }
//
//     impl<K, Q: ?Sized, V, Ls> LensRef<_ix<&'_ Q, Ls>> for BTreeMap<K, V>
//     where
//         K: Borrow<Q> + Ord,
//         Q: Ord,
//         V: LensRef<Ls>,
//     {
//         fn view_ref(&self, optics: _ix<&Q, Ls>) -> &Self::Image {
//             self[optics.1].view_ref(optics.0)
//         }
//     }
//
//     impl<K, Q: ?Sized, V, Ls> LensRef<_ix<&'_ Q, Ls>> for HashMap<K, V>
//     where
//         K: Eq + Hash + Borrow<Q>,
//         Q: Eq + Hash,
//         V: LensRef<Ls>,
//     {
//         fn view_ref(&self, optics: _ix<&Q, Ls>) -> &Self::Image {
//             self[optics.1].view_ref(optics.0)
//         }
//     }
// }

include!(concat!(env!("OUT_DIR"), "/optics.rs"));
