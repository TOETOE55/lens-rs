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

    impl<Image, T: From<Image>> Review<__, Image> for T {
        fn review(_optics: __, from: Image) -> Self {
            From::from(from)
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

    impl<Rv, Image, T> Review<optics::None<Rv>, Image> for Option<T> {
        fn review(_optics: optics::None<Rv>, _from: Image) -> Self {
            None
        }
    }
}

mod impl_tuples {

    /***********************************************************
     * impl for tuple
     ************************************************************/
    use crate::*;

    macro_rules! impl_tuple {
        ({$($param:ident)*}, $field:tt, $optic:ident, $to:ident) => {
            impl<Tr, Image: ?Sized, $($param,)*> TraversalRef<$optic<Tr>, Image> for ($($param,)*)
            where
                $to: TraversalRef<Tr, Image>,
            {

                fn traverse_ref(&self, optics: $optic<Tr>) -> Vec<&Image> {
                    self.$field.traverse_ref(optics.0)
                }
            }

            impl<Tr, Image: ?Sized, $($param,)*> TraversalMut<$optic<Tr>, Image> for ($($param,)*)
            where
                $to: TraversalMut<Tr, Image>,
            {
                fn traverse_mut(&mut self, optics: $optic<Tr>) -> Vec<&mut Image> {
                    self.$field.traverse_mut(optics.0)
                }
            }

            impl<Tr, Image, $($param,)*> Traversal<$optic<Tr>, Image> for ($($param,)*)
            where
                $to: Traversal<Tr, Image>,
            {
                fn traverse(self, optics: $optic<Tr>) -> Vec<Image>
                where
                    Self: Sized,
                {
                    self.$field.traverse(optics.0)
                }
            }

            impl<Pm, Image: ?Sized, $($param,)*> PrismRef<$optic<Pm>, Image> for ($($param,)*)
            where
                $to: PrismRef<Pm, Image>,
            {
                fn preview_ref(&self, optics: $optic<Pm>) -> Option<&Image> {
                    self.$field.preview_ref(optics.0)
                }
            }

            impl<Pm, Image: ?Sized, $($param,)*> PrismMut<$optic<Pm>, Image> for ($($param,)*)
            where
                $to: PrismMut<Pm, Image>,
            {
                fn preview_mut(&mut self, optics: $optic<Pm>) -> Option<&mut Image> {
                    self.$field.preview_mut(optics.0)
                }
            }

            impl<Pm, Image, $($param,)*> Prism<$optic<Pm>, Image> for ($($param,)*)
            where
                $to: Prism<Pm, Image>,
            {
                fn preview(self, optics: $optic<Pm>) -> Option<Image>
                where
                    Self: Sized,
                {
                    self.$field.preview(optics.0)
                }
            }

            impl<Ls, Image: ?Sized, $($param,)*> LensRef<$optic<Ls>, Image> for ($($param,)*)
            where
                $to: LensRef<Ls, Image>,
            {
                fn view_ref(&self, optics: $optic<Ls>) -> &Image {
                    self.$field.view_ref(optics.0)
                }
            }

            impl<Ls, Image: ?Sized, $($param,)*> LensMut<$optic<Ls>, Image> for ($($param,)*)
            where
                $to: LensMut<Ls, Image>,
            {
                fn view_mut(&mut self, optics: $optic<Ls>) -> &mut Image {
                    self.$field.view_mut(optics.0)
                }
            }

            impl<Ls, Image, $($param,)*> Lens<$optic<Ls>, Image> for ($($param,)*)
            where
                $to: Lens<Ls, Image>,
            {
                fn view(self, optics: $optic<Ls>) -> Image
                where
                    Self: Sized,
                {
                    self.$field.view(optics.0)
                }
            }
        }
    }

    impl_tuple!({ A }, 0, _0, A);

    impl_tuple!({A B}, 0, _0, A);
    impl_tuple!({A B}, 1, _1, B);

    impl_tuple!({A B C}, 0, _0, A);
    impl_tuple!({A B C}, 1, _1, B);
    impl_tuple!({A B C}, 2, _2, C);

    impl_tuple!({A B C D}, 0, _0, A);
    impl_tuple!({A B C D}, 1, _1, B);
    impl_tuple!({A B C D}, 2, _2, C);
    impl_tuple!({A B C D}, 3, _3, D);

    impl_tuple!({A B C D E}, 0, _0, A);
    impl_tuple!({A B C D E}, 1, _1, B);
    impl_tuple!({A B C D E}, 2, _2, C);
    impl_tuple!({A B C D E}, 3, _3, D);
    impl_tuple!({A B C D E}, 4, _4, E);

    impl_tuple!({A B C D E F}, 0, _0, A);
    impl_tuple!({A B C D E F}, 1, _1, B);
    impl_tuple!({A B C D E F}, 2, _2, C);
    impl_tuple!({A B C D E F}, 3, _3, D);
    impl_tuple!({A B C D E F}, 4, _4, E);
    impl_tuple!({A B C D E F}, 5, _5, F);

    impl_tuple!({A B C D E F G}, 0, _0, A);
    impl_tuple!({A B C D E F G}, 1, _1, B);
    impl_tuple!({A B C D E F G}, 2, _2, C);
    impl_tuple!({A B C D E F G}, 3, _3, D);
    impl_tuple!({A B C D E F G}, 4, _4, E);
    impl_tuple!({A B C D E F G}, 5, _5, F);
    impl_tuple!({A B C D E F G}, 6, _6, G);

    impl<Rv, Image, A> Review<_0<Rv>, Image> for (A,)
    where
        A: Review<Rv, Image>,
    {
        fn review(optics: _0<Rv>, from: Image) -> Self {
            (Review::review(optics.0, from),)
        }
    }

    macro_rules! impl_both {
        (<$param:ident> $tuple:ty, $($fields:tt),*) => {
            impl<Tr, Image: ?Sized, $param> TraversalRef<_both<Tr>, Image> for $tuple
            where
                Tr: Clone,
                $param: TraversalRef<Tr, Image>,
            {

                fn traverse_ref(&self, optics: _both<Tr>) -> Vec<&Image> {
                    let mut vec = vec![];
                    $(vec.append(&mut self.$fields.traverse_ref(optics.0.clone()));)*
                    vec
                }
            }

            impl<Tr, Image: ?Sized, $param> TraversalMut<_both<Tr>, Image> for $tuple
            where
                Tr: Clone,
                $param: TraversalMut<Tr, Image>,
            {
                fn traverse_mut(&mut self, optics: _both<Tr>) -> Vec<&mut Image> {
                    let mut vec = vec![];
                    $(vec.append(&mut self.$fields.traverse_mut(optics.0.clone()));)*
                    vec
                }
            }

            impl<Tr, Image, $param> Traversal<_both<Tr>, Image> for $tuple
            where
                Tr: Clone,
                $param: Traversal<Tr, Image>,
            {
                fn traverse(self, optics: _both<Tr>) -> Vec<Image>
                where
                    Self: Sized,
                {
                    let mut vec = vec![];
                    $(vec.append(&mut self.$fields.traverse(optics.0.clone()));)*
                    vec
                }
            }
        }
    }

    impl_both!(<A> (A,), 0);
    impl_both!(<A> (A, A), 0, 1);
    impl_both!(<A> (A, A, A), 0, 1, 2);
    impl_both!(<A> (A, A, A, A), 0, 1, 2, 3);
    impl_both!(<A> (A, A, A, A, A), 0, 1, 2, 3, 4);
    impl_both!(<A> (A, A, A, A, A, A), 0, 1, 2, 3, 4, 5);
    impl_both!(<A> (A, A, A, A, A, A, A), 0, 1, 2, 3, 4, 5, 6);

    impl<Pm: Clone, Image: ?Sized, A> PrismRef<_both<Pm>, Image> for (A,)
    where
        A: PrismRef<Pm, Image>,
    {
        fn preview_ref(&self, optics: _both<Pm>) -> Option<&Image> {
            self.0.preview_ref(optics.0)
        }
    }

    impl<Pm: Clone, Image: ?Sized, A> PrismMut<_both<Pm>, Image> for (A,)
    where
        A: PrismMut<Pm, Image>,
    {
        fn preview_mut(&mut self, optics: _both<Pm>) -> Option<&mut Image> {
            self.0.preview_mut(optics.0)
        }
    }

    impl<Pm: Clone, Image, A> Prism<_both<Pm>, Image> for (A,)
    where
        A: Prism<Pm, Image>,
    {
        fn preview(self, optics: _both<Pm>) -> Option<Image>
        where
            Self: Sized,
        {
            self.0.preview(optics.0)
        }
    }

    impl<Ls: Clone, Image: ?Sized, A> LensRef<_both<Ls>, Image> for (A,)
    where
        A: LensRef<Ls, Image>,
    {
        fn view_ref(&self, optics: _both<Ls>) -> &Image {
            self.0.view_ref(optics.0)
        }
    }

    impl<Ls: Clone, Image: ?Sized, A> LensMut<_both<Ls>, Image> for (A,)
    where
        A: LensMut<Ls, Image>,
    {
        fn view_mut(&mut self, optics: _both<Ls>) -> &mut Image {
            self.0.view_mut(optics.0)
        }
    }

    impl<Ls: Clone, Image, A> Lens<_both<Ls>, Image> for (A,)
    where
        A: Lens<Ls, Image>,
    {
        fn view(self, optics: _both<Ls>) -> Image
        where
            Self: Sized,
        {
            self.0.view(optics.0)
        }
    }

    impl<Rv, Image, A> Review<_both<Rv>, Image> for (A,)
    where
        A: Review<Rv, Image>,
    {
        fn review(optics: _both<Rv>, from: Image) -> Self {
            (Review::review(optics.0, from),)
        }
    }
}

mod impl_collect {
    /***********************************************************
     * impl for iter
     ************************************************************/
    use crate::*;
    use std::collections::*;

    macro_rules! impl_iter {
        (<$item:ident> $collector:ty) => {
            impl<Tr: Clone, Image: ?Sized, $item> TraversalRef<_mapped<Tr>, Image> for $collector
            where
                $item: TraversalRef<Tr, Image>,
            {
                fn traverse_ref(&self, optics: _mapped<Tr>) -> Vec<&Image> {
                    self.into_iter()
                        .flat_map(|t| t.traverse_ref(optics.0.clone()))
                        .collect()
                }
            }

            impl<Tr: Clone, Image: ?Sized, $item> TraversalMut<_mapped<Tr>, Image> for $collector
            where
                $item: TraversalMut<Tr, Image>,
            {
                fn traverse_mut(&mut self, optics: _mapped<Tr>) -> Vec<&mut Image> {
                    self.into_iter()
                        .flat_map(|t| t.traverse_mut(optics.0.clone()))
                        .collect()
                }
            }

            impl<Tr: Clone, Image, $item> Traversal<_mapped<Tr>, Image> for $collector
            where
                $item: Traversal<Tr, Image>,
            {
                fn traverse(self, optics: _mapped<Tr>) -> Vec<Image>
                where
                    Self: Sized,
                {
                    self.into_iter()
                        .flat_map(|t| t.traverse(optics.0.clone()))
                        .collect()
                }
            }
        };
    }

    impl_iter!(<T> Vec<T>);
    impl_iter!(<T> VecDeque<T>);
    impl_iter!(<T> LinkedList<T>);
}

mod impl_ptr {
    use crate::*;
    use std::rc::Rc;
    use std::sync::Arc;

    macro_rules! impl_ref {
        (<$param:ident> $ptr:ty, $optic:ident) => {
            impl<$param: ?Sized, Image: ?Sized, Tr> TraversalRef<$optic<Tr>, Image> for $ptr
            where
                $param: TraversalRef<Tr, Image>,
            {
                fn traverse_ref(&self, optics: $optic<Tr>) -> Vec<&Image> {
                    (**self).traverse_ref(optics.0)
                }
            }

            impl<$param: ?Sized, Image: ?Sized, Pm> PrismRef<$optic<Pm>, Image> for $ptr
            where
                $param: PrismRef<Pm, Image>,
            {
                fn preview_ref(&self, optics: $optic<Pm>) -> Option<&Image> {
                    (**self).preview_ref(optics.0)
                }
            }

            impl<$param: ?Sized, Image: ?Sized, Ls> LensRef<$optic<Ls>, Image> for $ptr
            where
                $param: LensRef<Ls, Image>,
            {
                fn view_ref(&self, optics: $optic<Ls>) -> &Image {
                    (**self).view_ref(optics.0)
                }
            }
        };
    }

    macro_rules! impl_mut {
        (<$param:ident> $ptr:ty, $optic:ident) => {
            impl<$param: ?Sized, Image: ?Sized, Tr> TraversalMut<$optic<Tr>, Image> for $ptr
            where
                $param: TraversalMut<Tr, Image>,
            {
                fn traverse_mut(&mut self, optics: $optic<Tr>) -> Vec<&mut Image> {
                    (**self).traverse_mut(optics.0)
                }
            }

            impl<$param: ?Sized, Image: ?Sized, Pm> PrismMut<$optic<Pm>, Image> for $ptr
            where
                $param: PrismMut<Pm, Image>,
            {
                fn preview_mut(&mut self, optics: $optic<Pm>) -> Option<&mut Image> {
                    (**self).preview_mut(optics.0)
                }
            }

            impl<$param: ?Sized, Image: ?Sized, Ls> LensMut<$optic<Ls>, Image> for $ptr
            where
                $param: LensMut<Ls, Image>,
            {
                fn view_mut(&mut self, optics: $optic<Ls>) -> &mut Image {
                    (**self).view_mut(optics.0)
                }
            }
        };
    }

    impl<Rv, Image, T> Review<_box<Rv>, Image> for Box<T>
    where
        T: Review<Rv, Image>,
    {
        fn review(optics: _box<Rv>, from: Image) -> Self {
            Box::new(Review::review(optics.0, from))
        }
    }

    impl<Tr, Image, T> Traversal<_box<Tr>, Image> for Box<T>
    where
        T: Traversal<Tr, Image>,
    {
        fn traverse(self, optics: _box<Tr>) -> Vec<Image>
        where
            Self: Sized,
        {
            (*self).traverse(optics.0)
        }
    }

    impl<Pm, Image, T> Prism<_box<Pm>, Image> for Box<T>
    where
        T: Prism<Pm, Image>,
    {
        fn preview(self, optics: _box<Pm>) -> Option<Image>
        where
            Self: Sized,
        {
            (*self).preview(optics.0)
        }
    }

    impl<Ls, Image, T> Lens<_box<Ls>, Image> for Box<T>
    where
        T: Lens<Ls, Image>,
    {
        fn view(self, optics: _box<Ls>) -> Image
        where
            Self: Sized,
        {
            (*self).view(optics.0)
        }
    }

    impl<Rv, Image, T> Review<_box<Rv>, Image> for Rc<T>
    where
        T: Review<Rv, Image>,
    {
        fn review(optics: _box<Rv>, from: Image) -> Self {
            Rc::new(Review::review(optics.0, from))
        }
    }

    impl<Rv, Image, T> Review<_box<Rv>, Image> for Arc<T>
    where
        T: Review<Rv, Image>,
    {
        fn review(optics: _box<Rv>, from: Image) -> Self {
            Arc::new(Review::review(optics.0, from))
        }
    }

    impl_ref!(<T> Box<T>, _ref);
    impl_ref!(<T> Box<T>, _mut);
    impl_ref!(<T> Box<T>, _box);
    impl_ref!(<T> Rc<T>, _ref);
    impl_ref!(<T> Arc<T>, _ref);
    impl_ref!(<T> &'_ mut T, _ref);
    impl_ref!(<T> &'_ mut T, _mut);
    impl_ref!(<T> &'_ T, _ref);

    impl_mut!(<T> Box<T>, _box);
    impl_mut!(<T> Box<T>, _mut);
    impl_mut!(<T> &'_ mut T, _mut);
}

mod impl_ix {
    use crate::*;
    use std::borrow::Borrow;
    use std::collections::{BTreeMap, HashMap, VecDeque};
    use std::hash::Hash;
    use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

    macro_rules! impl_ix {
        (<$($param:ident,)? $(const $c:ident: $ct: ty)?> $t:ty[$ix:ty]: $o:ty) => {

            impl<$($param,)? Image: ?Sized, Tr, $(const $c: $ct)?> TraversalRef<_ix<$ix, Tr>, Image> for $t
            where
                $o: TraversalRef<Tr, Image>,
            {

                fn traverse_ref(&self, optics: _ix<$ix, Tr>) -> Vec<&Image> {
                    self[optics.1].traverse_ref(optics.0)
                }
            }

            impl<$($param,)? Image: ?Sized, Tr, $(const $c: $ct)?> TraversalMut<_ix<$ix, Tr>, Image> for $t
            where
                $o: TraversalMut<Tr, Image>,
            {

                fn traverse_mut(&mut self, optics: _ix<$ix, Tr>) -> Vec<&mut Image> {
                    self[optics.1].traverse_mut(optics.0)
                }
            }

            impl<$($param,)? Image: ?Sized, Pm, $(const $c: $ct)?> PrismRef<_ix<$ix, Pm>, Image> for $t
            where
                $o: PrismRef<Pm, Image>,
            {
                fn preview_ref(&self, optics: _ix<$ix, Pm>) -> Option<&Image> {
                    self[optics.1].preview_ref(optics.0)
                }
            }

            impl<$($param,)? Image: ?Sized, Pm, $(const $c: $ct)?> PrismMut<_ix<$ix, Pm>, Image> for $t
            where
                $o: PrismMut<Pm, Image>,
            {
                fn preview_mut(&mut self, optics: _ix<$ix, Pm>) -> Option<&mut Image> {
                    self[optics.1].preview_mut(optics.0)
                }
            }

            impl<$($param,)? Image: ?Sized, Ls, $(const $c: $ct)?> LensRef<_ix<$ix, Ls>, Image> for $t
            where
                $o: LensRef<Ls, Image>,
            {
                fn view_ref(&self, optics: _ix<$ix, Ls>) -> &Image {
                    self[optics.1].view_ref(optics.0)
                }
            }

            impl<$($param,)? Image: ?Sized, Pm, $(const $c: $ct)?> LensMut<_ix<$ix, Pm>, Image> for $t
            where
                $o: LensMut<Pm, Image>,
            {
                fn view_mut(&mut self, optics: _ix<$ix, Pm>) -> &mut Image {
                    self[optics.1].view_mut(optics.0)
                }
            }
        }
    }

    impl_ix!(<T,> Vec<T>[usize]: T);
    impl_ix!(<T,> Vec<T>[Range<usize>]: [T]);
    impl_ix!(<T,> Vec<T>[RangeTo<usize>]: [T]);
    impl_ix!(<T,> Vec<T>[RangeFrom<usize>]: [T]);
    impl_ix!(<T,> Vec<T>[RangeFull]: [T]);

    impl_ix!(<T,> VecDeque<T>[usize]: T);

    impl_ix!(<T, const N: usize> [T; N][usize]: T);
    impl_ix!(<T, const N: usize> [T; N][Range<usize>]: [T]);
    impl_ix!(<T, const N: usize> [T; N][RangeTo<usize>]: [T]);
    impl_ix!(<T, const N: usize> [T; N][RangeFrom<usize>]: [T]);
    impl_ix!(<T, const N: usize> [T; N][RangeFull]: [T]);

    impl_ix!(<T,> [T][usize]: T);
    impl_ix!(<T,> [T][Range<usize>]: [T]);
    impl_ix!(<T,> [T][RangeTo<usize>]: [T]);
    impl_ix!(<T,> [T][RangeFrom<usize>]: [T]);
    impl_ix!(<T,> [T][RangeFull]: [T]);

    impl_ix!(<> String[Range<usize>]: str);
    impl_ix!(<> String[RangeTo<usize>]: str);
    impl_ix!(<> String[RangeFrom<usize>]: str);
    impl_ix!(<> String[RangeFull]: str);

    impl_ix!(<> str[Range<usize>]: str);
    impl_ix!(<> str[RangeTo<usize>]: str);
    impl_ix!(<> str[RangeFrom<usize>]: str);
    impl_ix!(<> str[RangeFull]: str);

    impl<K, Q: ?Sized, V, Image: ?Sized, Tr> TraversalRef<_ix<&'_ Q, Tr>, Image> for BTreeMap<K, V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
        V: TraversalRef<Tr, Image>,
    {
        fn traverse_ref(&self, optics: _ix<&Q, Tr>) -> Vec<&Image> {
            self[optics.1].traverse_ref(optics.0)
        }
    }

    impl<K, Q: ?Sized, V, Image: ?Sized, Tr> TraversalRef<_ix<&'_ Q, Tr>, Image> for HashMap<K, V>
    where
        K: Eq + Hash + Borrow<Q>,
        Q: Eq + Hash,
        V: TraversalRef<Tr, Image>,
    {
        fn traverse_ref(&self, optics: _ix<&Q, Tr>) -> Vec<&Image> {
            self[optics.1].traverse_ref(optics.0)
        }
    }

    impl<K, Q: ?Sized, V, Image: ?Sized, Pm> PrismRef<_ix<&'_ Q, Pm>, Image> for BTreeMap<K, V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
        V: PrismRef<Pm, Image>,
    {
        fn preview_ref(&self, optics: _ix<&Q, Pm>) -> Option<&Image> {
            self[optics.1].preview_ref(optics.0)
        }
    }

    impl<K, Q: ?Sized, V, Image: ?Sized, Pm> PrismRef<_ix<&'_ Q, Pm>, Image> for HashMap<K, V>
    where
        K: Eq + Hash + Borrow<Q>,
        Q: Eq + Hash,
        V: PrismRef<Pm, Image>,
    {
        fn preview_ref(&self, optics: _ix<&Q, Pm>) -> Option<&Image> {
            self[optics.1].preview_ref(optics.0)
        }
    }

    impl<K, Q: ?Sized, V, Image: ?Sized, Ls> LensRef<_ix<&'_ Q, Ls>, Image> for BTreeMap<K, V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
        V: LensRef<Ls, Image>,
    {
        fn view_ref(&self, optics: _ix<&Q, Ls>) -> &Image {
            self[optics.1].view_ref(optics.0)
        }
    }

    impl<K, Q: ?Sized, V, Image: ?Sized, Ls> LensRef<_ix<&'_ Q, Ls>, Image> for HashMap<K, V>
    where
        K: Eq + Hash + Borrow<Q>,
        Q: Eq + Hash,
        V: LensRef<Ls, Image>,
    {
        fn view_ref(&self, optics: _ix<&Q, Ls>) -> &Image {
            self[optics.1].view_ref(optics.0)
        }
    }
}

mod impl_empty {
    use crate::*;

    macro_rules! impl_empty {
        (<$($life:lifetime),* $($gen:ident),* $(const $c:ident : $t:ident)?> $optic:ident for $ty:ty) => {
            impl<$($life),* Tr, Image: ?Sized, $($gen),* $(const $c: $t)?> TraversalRef<optics::$optic<Tr>, Image> for $ty {
                fn traverse_ref(&self, _optics: optics::$optic<Tr>) -> Vec<&Image> {
                    vec![]
                }
            }

            impl<$($life),* Tr, Image: ?Sized, $($gen),* $(const $c: $t)?> TraversalMut<optics::$optic<Tr>, Image> for $ty {
                fn traverse_mut(&mut self, _optics: optics::$optic<Tr>) -> Vec<&mut Image> {
                    vec![]
                }
            }

            impl<$($life),* Tr, Image, $($gen),* $(const $c: $t)?> Traversal<optics::$optic<Tr>, Image> for $ty {
                fn traverse(self, _optics: optics::$optic<Tr>) -> Vec<Image>
                where
                    Self: Sized,
                {
                    vec![]
                }
            }

            impl<$($life),* Pm, Image: ?Sized, $($gen),* $(const $c: $t)?> PrismRef<optics::$optic<Pm>, Image> for $ty {
                fn preview_ref(&self, _optics: optics::$optic<Pm>) -> Option<&Image> {
                    None
                }
            }

            impl<$($life),* Pm, Image: ?Sized, $($gen),* $(const $c: $t)?> PrismMut<optics::$optic<Pm>, Image> for $ty {
                fn preview_mut(&mut self, _optics: optics::$optic<Pm>) -> Option<&mut Image> {
                    None
                }
            }

            impl<$($life),* Pm, Image, $($gen),* $(const $c: $t)?> Prism<optics::$optic<Pm>, Image> for $ty {
                fn preview(self, _optics: optics::$optic<Pm>) -> Option<Image>
                where
                    Self: Sized,
                {
                    None
                }
            }
        }
    }


    impl_empty!(<T> None for Option<T>);
    impl_empty!(<T> Ok   for Option<T>);
    impl_empty!(<T> Err  for Option<T>);
    impl_empty!(<T> _0   for Option<T>);
    impl_empty!(<T> _1   for Option<T>);
    impl_empty!(<T> _2   for Option<T>);
    impl_empty!(<T> _3   for Option<T>);
    impl_empty!(<T> _4   for Option<T>);
    impl_empty!(<T> _5   for Option<T>);
    impl_empty!(<T> _6   for Option<T>);

    impl_empty!(<T, E> None for Result<T, E>);
    impl_empty!(<T, E> Some for Result<T, E>);
    impl_empty!(<T, E> _0   for Result<T, E>);
    impl_empty!(<T, E> _1   for Result<T, E>);
    impl_empty!(<T, E> _2   for Result<T, E>);
    impl_empty!(<T, E> _3   for Result<T, E>);
    impl_empty!(<T, E> _4   for Result<T, E>);
    impl_empty!(<T, E> _5   for Result<T, E>);
    impl_empty!(<T, E> _6   for Result<T, E>);

    impl_empty!(<A> Ok   for (A,));
    impl_empty!(<A> Err  for (A,));
    impl_empty!(<A> Some for (A,));
    impl_empty!(<A> None for (A,));
    impl_empty!(<A> _1   for (A,));
    impl_empty!(<A> _2   for (A,));
    impl_empty!(<A> _3   for (A,));
    impl_empty!(<A> _4   for (A,));
    impl_empty!(<A> _5   for (A,));
    impl_empty!(<A> _6   for (A,));

    impl_empty!(<A, B> Ok   for (A, B));
    impl_empty!(<A, B> Err  for (A, B));
    impl_empty!(<A, B> Some for (A, B));
    impl_empty!(<A, B> None for (A, B));
    impl_empty!(<A, B> _2   for (A, B));
    impl_empty!(<A, B> _3   for (A, B));
    impl_empty!(<A, B> _4   for (A, B));
    impl_empty!(<A, B> _5   for (A, B));
    impl_empty!(<A, B> _6   for (A, B));

    impl_empty!(<A, B, C> Ok   for (A, B, C));
    impl_empty!(<A, B, C> Err  for (A, B, C));
    impl_empty!(<A, B, C> Some for (A, B, C));
    impl_empty!(<A, B, C> None for (A, B, C));
    impl_empty!(<A, B, C> _3   for (A, B, C));
    impl_empty!(<A, B, C> _4   for (A, B, C));
    impl_empty!(<A, B, C> _5   for (A, B, C));
    impl_empty!(<A, B, C> _6   for (A, B, C));

    impl_empty!(<A, B, C, D> Ok   for (A, B, C, D));
    impl_empty!(<A, B, C, D> Err  for (A, B, C, D));
    impl_empty!(<A, B, C, D> Some for (A, B, C, D));
    impl_empty!(<A, B, C, D> None for (A, B, C, D));
    impl_empty!(<A, B, C, D> _4   for (A, B, C, D));
    impl_empty!(<A, B, C, D> _5   for (A, B, C, D));
    impl_empty!(<A, B, C, D> _6   for (A, B, C, D));

    impl_empty!(<A, B, C, D, E> Ok   for (A, B, C, D, E));
    impl_empty!(<A, B, C, D, E> Err  for (A, B, C, D, E));
    impl_empty!(<A, B, C, D, E> Some for (A, B, C, D, E));
    impl_empty!(<A, B, C, D, E> None for (A, B, C, D, E));
    impl_empty!(<A, B, C, D, E> _5   for (A, B, C, D, E));
    impl_empty!(<A, B, C, D, E> _6   for (A, B, C, D, E));

    impl_empty!(<A, B, C, D, E, F> Ok   for (A, B, C, D, E, F));
    impl_empty!(<A, B, C, D, E, F> Err  for (A, B, C, D, E, F));
    impl_empty!(<A, B, C, D, E, F> Some for (A, B, C, D, E, F));
    impl_empty!(<A, B, C, D, E, F> None for (A, B, C, D, E, F));
    impl_empty!(<A, B, C, D, E, F> _6   for (A, B, C, D, E, F));

    impl_empty!(<A, B, C, D, E, F, G> Ok   for (A, B, C, D, E, F, G));
    impl_empty!(<A, B, C, D, E, F, G> Err  for (A, B, C, D, E, F, G));
    impl_empty!(<A, B, C, D, E, F, G> Some for (A, B, C, D, E, F, G));
    impl_empty!(<A, B, C, D, E, F, G> None for (A, B, C, D, E, F, G));
}

include!(concat!(env!("OUT_DIR"), "/optics.rs"));
