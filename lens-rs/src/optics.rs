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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _0<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _1<Optic>(pub Optic);
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _both<Optic>(pub Optic);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _mapped<Optic>(pub Optic);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _box<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _ref<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _mut<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _rc<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _arc<Optic>(pub Optic);

//impls
mod impl__ {
    /***********************************************************
     * impl for __
     ************************************************************/
    use crate::*;
    use std::iter::FromIterator;

    impl<T> Review<T> for __ {
        type From = T;

        fn review<F: Into<Self::From>>(&self, from: F) -> T {
            from.into()
        }
    }

    impl<T> TraversalRef<T> for __ {
        type To = T;
        fn traverse_ref<'a, F: FromIterator<&'a Self::To>>(&self, source: &'a T) -> F
        where
            Self::To: 'a
        {
            FromIterator::from_iter(Option::Some(source))
        }
    }

    impl<T> TraversalMut<T> for __ {
        fn traverse_mut<'a, F: FromIterator<&'a mut Self::To>>(&self, source: &'a mut T) -> F
        where
            Self::To: 'a
        {
            FromIterator::from_iter(Option::Some(source))
        }
    }

    impl<T> Traversal<T> for __ {
        fn traverse<F: FromIterator<Self::To>>(&self, source: T) -> F {
            FromIterator::from_iter(Option::Some(source))
        }
    }

    impl<T> PrismRef<T> for __ {
        fn pm_ref<'a, F: From<Option<&'a Self::To>>>(&self, source: &'a T) -> F
        where
            Self::To: 'a
        {
            From::from(Option::Some(source))
        }
    }

    impl<T> PrismMut<T> for __ {
        fn pm_mut<'a, F: From<Option<&'a mut Self::To>>>(&self, source: &'a mut T) -> F
        where
            Self::To: 'a
        {
            From::from(Option::Some(source))
        }
    }

    impl<T> Prism<T> for __ {
        fn pm<F: From<Option<Self::To>>>(&self, source: T) -> F {
            From::from(Option::Some(source))
        }
    }

    impl<T> LensRef<T> for __ {
        fn view_ref<'a, F: From<&'a Self::To>>(&self, source: &'a T) -> F
        where
            Self::To: 'a
        {
            From::from(source)
        }
    }

    impl<T> LensMut<T> for __ {
        fn view_mut<'a, F: From<&'a mut Self::To>>(&self, source: &'a mut T) -> F
        where
            Self::To: 'a
        {
            From::from(source)
        }
    }

    impl<T> Lens<T> for __ {
        fn view<F: From<Self::To>>(&self, source: T) -> F {
            From::from(source)
        }
    }
}

mod impl_result {
    /***********************************************************
     * impl for Result
     ************************************************************/
    use crate::*;
    use std::iter::FromIterator;

    impl<Rv, T, E> Review<Result<T, E>> for optics::Ok<Rv>
    where
        Rv: Review<T>,
    {
        type From = Rv::From;

        fn review<F: Into<Self::From>>(&self, from: F) -> Result<T, E> {
            Result::Ok(self.0.review(from.into()))
        }
    }

    impl<Tr, T, E> TraversalRef<Result<T, E>> for optics::Ok<Tr>
    where
        Tr: TraversalRef<T>,
    {
        type To = Tr::To;
        fn traverse_ref<'a, F: FromIterator<&'a Self::To>>(&self, source: &'a Result<T, E>) -> F
        where
            Self::To: 'a
        {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_ref::<Vec<_>>(t))
                .collect()
        }
    }

    impl<Tr, T, E> TraversalMut<Result<T, E>> for optics::Ok<Tr>
    where
        Tr: TraversalMut<T>,
    {
        fn traverse_mut<'a, F: FromIterator<&'a mut Self::To>>(&self, source: &'a mut Result<T, E>) -> F
        where
            Self::To: 'a
        {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_mut::<Vec<_>>(t))
                .collect()
        }
    }

    impl<Tr, T, E> Traversal<Result<T, E>> for optics::Ok<Tr>
    where
        Tr: Traversal<T>,
    {
        fn traverse<F: FromIterator<Self::To>>(&self, source: Result<T, E>) -> F {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse::<Vec<_>>(t))
                .collect()
        }
    }

    impl<Tr, T, E> Traversal<Result<T, E>> for optics::Err<Tr>
    where
        Tr: Traversal<E>,
    {
        fn traverse<F: FromIterator<Self::To>>(&self, source: Result<T, E>) -> F {
            source
                .err()
                .into_iter()
                .flat_map(|t| self.0.traverse::<Vec<_>>(t))
                .collect()
        }
    }

    impl<Tr, T, E> TraversalRef<Result<T, E>> for optics::Err<Tr>
    where
        Tr: TraversalRef<E>,
    {
        type To = Tr::To;
        fn traverse_ref<'a, F: FromIterator<&'a Self::To>>(&self, source: &'a Result<T, E>) -> F
            where
                Self::To: 'a
        {
            source
                .as_ref()
                .err()
                .into_iter()
                .flat_map(|t| self.0.traverse_ref::<Vec<_>>(t))
                .collect()
        }
    }

    impl<Tr, T, E> TraversalMut<Result<T, E>> for optics::Err<Tr>
    where
        Tr: TraversalMut<E>,
    {
        fn traverse_mut<'a, F: FromIterator<&'a mut Self::To>>(&self, source: &'a mut Result<T, E>) -> F
            where
                Self::To: 'a
        {
            source
                .as_mut()
                .err()
                .into_iter()
                .flat_map(|t| self.0.traverse_mut::<Vec<_>>(t))
                .collect()
        }
    }

    impl<Pm, T, E> PrismRef<Result<T, E>> for optics::Ok<Pm>
    where
        Pm: PrismRef<T>,
    {
        fn pm_ref<'a, F: From<Option<&'a Self::To>>>(&self, source: &'a Result<T, E>) -> F
        where
            Self::To: 'a
        {
            From::from(source.as_ref().ok().and_then(|t| self.0.pm_ref(t)))
        }
    }

    impl<Pm, T, E> PrismMut<Result<T, E>> for optics::Ok<Pm>
    where
        Pm: PrismMut<T>,
    {
        fn pm_mut<'a, F: From<Option<&'a mut Self::To>>>(&self, source: &'a mut Result<T, E>) -> F
        where
            Self::To: 'a
        {
            From::from(source.as_mut().ok().and_then(|t| self.0.pm_mut(t)))
        }
    }

    impl<Pm, T, E> Prism<Result<T, E>> for optics::Ok<Pm>
    where
        Pm: Prism<T>,
    {
        fn pm<F: From<Option<Self::To>>>(&self, source: Result<T, E>) -> F {
            From::from(source.ok().and_then(|t| self.0.pm(t)))
        }
    }

    impl<Rv, T, E> Review<Result<T, E>> for optics::Err<Rv>
    where
        Rv: Review<E>,
    {
        type From = Rv::From;

        fn review<F: Into<Self::From>>(&self, from: F) -> Result<T, E> {
            Result::Err(self.0.review(from.into()))
        }
    }

    impl<Pm, T, E> PrismRef<Result<T, E>> for optics::Err<Pm>
    where
        Pm: PrismRef<E>,
    {
        fn pm_ref<'a, F: From<Option<&'a Self::To>>>(&self, source: &'a Result<T, E>) -> F
        where
            Self::To: 'a
        {
            From::from(source.as_ref().err().and_then(|t| self.0.pm_ref(t)))
        }
    }

    impl<Pm, T, E> PrismMut<Result<T, E>> for optics::Err<Pm>
    where
        Pm: PrismMut<E>,
    {
        fn pm_mut<'a, F: From<Option<&'a mut Self::To>>>(&self, source: &'a mut Result<T, E>) -> F
        where
            Self::To: 'a
        {
            From::from(source.as_mut().err().and_then(|t| self.0.pm_mut(t)))
        }
    }

    impl<Pm, T, E> Prism<Result<T, E>> for optics::Err<Pm>
    where
        Pm: Prism<E>,
    {
        fn pm<F: From<Option<Self::To>>>(&self, source: Result<T, E>) -> F {
            From::from(source.err().and_then(|t| self.0.pm(t)))
        }
    }
}

mod impl_some {
    /***********************************************************
     * impl for Option
     ************************************************************/

    use crate::*;
    use std::iter::FromIterator;

    impl<Rv, T> Review<Option<T>> for optics::Some<Rv>
    where
        Rv: Review<T>,
    {
        type From = Rv::From;

        fn review<F: Into<Self::From>>(&self, from: F) -> Option<T> {
            Option::Some(self.0.review(from.into()))
        }
    }

    impl<Tr, T> TraversalRef<Option<T>> for optics::Some<Tr>
    where
        Tr: TraversalRef<T>,
    {
        type To = Tr::To;

        fn traverse_ref<'a, F: FromIterator<&'a Self::To>>(&self, source: &'a Option<T>) -> F
        where
            Self::To: 'a
        {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_ref::<Vec<_>>(t))
                .collect()
        }
    }

    impl<Tr, T> TraversalMut<Option<T>> for optics::Some<Tr>
    where
        Tr: TraversalMut<T>,
    {
        fn traverse_mut<'a, F: FromIterator<&'a mut Self::To>>(&self, source: &'a mut Option<T>) -> F
        where
            Self::To: 'a
        {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_mut::<Vec<_>>(t))
                .collect()
        }
    }

    impl<Tr, T> Traversal<Option<T>> for optics::Some<Tr>
    where
        Tr: Traversal<T>,
    {
        fn traverse<F: FromIterator<Self::To>>(&self, source: Option<T>) -> F {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse::<Vec<_>>(t))
                .collect()
        }
    }

    impl<Pm, T> PrismRef<Option<T>> for optics::Some<Pm>
    where
        Pm: PrismRef<T>,
    {
        fn pm_ref<'a, F: From<Option<&'a Self::To>>>(&self, source: &'a Option<T>) -> F
        where
            Self::To: 'a
        {
            From::from(source.as_ref().and_then(|t| self.0.pm_ref(t)))
        }
    }

    impl<Pm, T> PrismMut<Option<T>> for optics::Some<Pm>
    where
        Pm: PrismMut<T>,
    {
        fn pm_mut<'a, F: From<Option<&'a mut Self::To>>>(&self, source: &'a mut Option<T>) -> F
        where
            Self::To: 'a
        {
            From::from(source.as_mut().and_then(|t| self.0.pm_mut(t)))
        }
    }

    impl<Pm, T> Prism<Option<T>> for optics::Some<Pm>
    where
        Pm: Prism<T>,
    {
        fn pm<F: From<Option<Self::To>>>(&self, source: Option<T>) -> F {
            From::from(source.and_then(|t| self.0.pm(t)))
        }
    }



    impl<Rv, T> Review<Option<T>> for optics::None<Rv>
    where
        Rv: Review<()>,
    {
        type From = ();

        fn review<F: Into<Self::From>>(&self, _from: F) -> Option<T> {
            Option::None
        }
    }
}

mod impl_tuples {

    /***********************************************************
     * impl for tuple
     ************************************************************/
    use crate::*;
    use std::iter::FromIterator;

    macro_rules! impl_tuple {
        ({$($param:ident)*}, $field:tt, $optic:ident, $to:ident) => {
            impl<Tr, $($param,)*> TraversalRef<($($param,)*)> for $optic<Tr>
            where
                Tr: TraversalRef<$to>,
            {
                type To = Tr::To;

                fn traverse_ref<'a, F: FromIterator<&'a Self::To>>(&self, source: &'a ($($param,)*)) -> F
                where
                    Self::To: 'a
                {
                    self.0.traverse_ref(&source.$field)
                }
            }

            impl<Tr, $($param,)*> TraversalMut<($($param,)*)> for $optic<Tr>
            where
                Tr: TraversalMut<$to>,
            {
                fn traverse_mut<'a, F: FromIterator<&'a mut Self::To>>(&self, source: &'a mut ($($param,)*)) -> F
                where
                    Self::To: 'a
                {
                    self.0.traverse_mut(&mut source.$field)
                }
            }

            impl<Tr, $($param,)*> Traversal<($($param,)*)> for $optic<Tr>
            where
                Tr: Traversal<$to>,
            {
                fn traverse<F: FromIterator<Self::To>>(&self, source: ($($param,)*)) -> F {
                    self.0.traverse(source.$field)
                }
            }

            impl<Pm, $($param,)*> PrismRef<($($param,)*)> for $optic<Pm>
            where
                Pm: PrismRef<$to>,
            {

                fn pm_ref<'a, F: From<Option<&'a Self::To>>>(&self, source: &'a ($($param,)*)) -> F
                where
                    Self::To: 'a
                {
                    self.0.pm_ref(&source.$field)
                }
            }

            impl<Pm, $($param,)*> PrismMut<($($param,)*)> for $optic<Pm>
            where
                Pm: PrismMut<$to>,
            {

                fn pm_mut<'a, F: From<Option<&'a mut Self::To>>>(&self, source: &'a mut ($($param,)*)) -> F
                where
                    Self::To: 'a
                {
                    self.0.pm_mut(&mut source.$field)
                }
            }

            impl<Pm, $($param,)*> Prism<($($param,)*)> for $optic<Pm>
            where
                Pm: Prism<$to>,
            {

                fn pm<F: From<Option<Self::To>>>(&self, source: ($($param,)*)) -> F {
                    self.0.pm(source.$field)
                }
            }

            impl<Ls, $($param,)* > LensRef<($($param,)*)> for $optic<Ls>
            where
                Ls: LensRef<$to>,
            {
                fn view_ref<'a, F: From<&'a Self::To>>(&self, source: &'a ($($param,)*)) -> F
                where
                    Self::To: 'a
                {
                    self.0.view_ref(&source.$field)
                }
            }

            impl<Ls, $($param,)* > LensMut<($($param,)*)> for $optic<Ls>
            where
                Ls: LensMut<$to>,
            {
                fn view_mut<'a, F: From<&'a mut Self::To>>(&self, source: &'a mut ($($param,)*)) -> F
                where
                    Self::To: 'a
                {
                    self.0.view_mut(&mut source.$field)
                }
            }

            impl<Ls, $($param,)* > Lens<($($param,)*)> for $optic<Ls>
            where
                Ls: Lens<$to>,
            {
                fn view<F: From<Self::To>>(&self, source: ($($param,)*)) -> F {
                    self.0.view(source.$field)
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

    impl_tuple!({A B C D E F1}, 0, _0, A);
    impl_tuple!({A B C D E F1}, 1, _1, B);
    impl_tuple!({A B C D E F1}, 2, _2, C);
    impl_tuple!({A B C D E F1}, 3, _3, D);
    impl_tuple!({A B C D E F1}, 4, _4, E);
    impl_tuple!({A B C D E F1}, 5, _5, F1);

    impl_tuple!({A B C D E F1 G}, 0, _0, A);
    impl_tuple!({A B C D E F1 G}, 1, _1, B);
    impl_tuple!({A B C D E F1 G}, 2, _2, C);
    impl_tuple!({A B C D E F1 G}, 3, _3, D);
    impl_tuple!({A B C D E F1 G}, 4, _4, E);
    impl_tuple!({A B C D E F1 G}, 5, _5, F1);
    impl_tuple!({A B C D E F1 G}, 6, _6, G);

    impl<Rv, A> Review<(A,)> for _0<Rv>
    where
        Rv: Review<A>,
    {
        type From = Rv::From;

        fn review<F: Into<Self::From>>(&self, from: F) -> (A,) {
            (self.0.review(from.into()),)
        }
    }

    macro_rules! impl_both {
        (<$param:ident> $tuple:ty, $($fields:tt),*) => {
            impl<Tr, $param> TraversalRef<$tuple> for _both<Tr>
            where
                Tr: TraversalRef<$param>
            {
                type To = Tr::To;

                fn traverse_ref<'a, F: FromIterator<&'a Self::To>>(&self, source: &'a $tuple) -> F
                where
                    Self::To: 'a
                {
                    let mut vec = vec![];
                    $(vec.extend(self.0.traverse_ref::<Vec<_>>(&source.$fields));)*
                    FromIterator::from_iter(vec)
                }
            }

            impl<Tr, $param> TraversalMut<$tuple> for _both<Tr>
            where
                Tr: TraversalMut<$param>
            {
                fn traverse_mut<'a, F: FromIterator<&'a mut Self::To>>(&self, source: &'a mut $tuple) -> F
                where
                    Self::To: 'a
                {
                    let mut vec = vec![];
                    $(vec.extend(self.0.traverse_mut::<Vec<_>>(&mut source.$fields));)*
                    FromIterator::from_iter(vec)
                }
            }

            impl<Tr, $param> Traversal<$tuple> for _both<Tr>
            where
                Tr: Traversal<$param>
            {
                fn traverse<F: FromIterator<Self::To>>(&self, source: $tuple) -> F {
                    let mut vec = vec![];
                    $(vec.extend(self.0.traverse::<Vec<_>>(source.$fields));)*
                    FromIterator::from_iter(vec)
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

    impl<Pm, A> PrismRef<(A,)> for _both<Pm>
    where
        Pm: PrismRef<A>,
    {
        fn pm_ref<'a, F: From<Option<&'a Self::To>>>(&self, source: &'a (A,)) -> F
        where
            Self::To: 'a
        {
            self.0.pm_ref(&source.0)
        }
    }

    impl<Pm, A> PrismMut<(A,)> for _both<Pm>
    where
        Pm: PrismMut<A>,
    {
        fn pm_mut<'a, F: From<Option<&'a mut Self::To>>>(&self, source: &'a mut (A,)) -> F
        where
            Self::To: 'a
        {
            self.0.pm_mut(&mut source.0)
        }
    }

    impl<Pm, A> Prism<(A,)> for _both<Pm>
    where
        Pm: Prism<A>,
    {
        fn pm<F: From<Option<Self::To>>>(&self, source: (A,)) -> F {
            self.0.pm(source.0)
        }
    }

    impl<Ls, A> LensRef<(A,)> for _both<Ls>
    where
        Ls: LensRef<A>,
    {
        fn view_ref<'a, F: From<&'a Self::To>>(&self, source: &'a (A,)) -> F
        where
            Self::To: 'a
        {
            self.0.view_ref(&source.0)
        }
    }

    impl<Ls, A> LensMut<(A,)> for _both<Ls>
    where
        Ls: LensMut<A>,
    {
        fn view_mut<'a, F: From<&'a mut Self::To>>(&self, source: &'a mut (A,)) -> F
        where
            Self::To: 'a
        {
            self.0.view_mut(&mut source.0)
        }
    }

    impl<Ls, A> Lens<(A,)> for _both<Ls>
    where
        Ls: Lens<A>,
    {
        fn view<F: From<Self::To>>(&self, source: (A,)) -> F {
            self.0.view(source.0)
        }
    }

    impl<Rv, A> Review<(A,)> for _both<Rv>
    where
        Rv: Review<A>,
    {
        type From = Rv::From;

        fn review<F: Into<Self::From>>(&self, from: F) -> (A,) {
            (self.0.review(from.into()),)
        }
    }
}

mod impl_iters {
    /***********************************************************
     * impl for iter
     ************************************************************/
    use crate::*;
    use std::collections::*;
    use std::iter::FromIterator;
    use std::iter::IntoIterator;

    macro_rules! impl_iter {
        (<$param:ident> $iter:ty) => {
            impl<Tr, $param> TraversalRef<$iter> for _mapped<Tr>
            where
                Tr: TraversalRef<$param>,
            {
                type To = Tr::To;

                fn traverse_ref<'a, F: FromIterator<&'a Self::To>>(&self, source: &'a $iter) -> F
                where
                    Self::To: 'a
                {
                    source.into_iter().flat_map(|t| self.0.traverse_ref::<Vec<_>>(t)).collect()
                }
            }

            impl<Tr, $param> TraversalMut<$iter> for _mapped<Tr>
            where
                Tr: TraversalMut<$param>,
            {
                fn traverse_mut<'a, F: FromIterator<&'a mut Self::To>>(&self, source: &'a mut $iter) -> F
                where
                    Self::To: 'a
                {
                    source
                        .into_iter()
                        .flat_map(|t| self.0.traverse_mut::<Vec<_>>(t))
                        .collect()
                }
            }

            impl<Tr, $param> Traversal<$iter> for _mapped<Tr>
            where
                Tr: Traversal<$param>,
            {
                fn traverse<F: FromIterator<Self::To>>(&self, source: $iter) -> F {
                    source
                        .into_iter()
                        .flat_map(|t| self.0.traverse::<Vec<_>>(t))
                        .collect()
                }
            }
        }
    }

    impl_iter!(<T> Vec<T>);
    impl_iter!(<T> VecDeque<T>);
    impl_iter!(<T> LinkedList<T>);
}

mod impl_ptr {
    use crate::*;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::iter::FromIterator;

    macro_rules! impl_ref {
        (<$($life:lifetime),*; $param:ident> $ptr:ty, $optic:ident) => {
            impl<$($life,)* $param, Tr> TraversalRef<$ptr> for $optic<Tr>
            where
                $param: ?Sized,
                Tr: TraversalRef<$param>,
            {
                type To = Tr::To;

                fn traverse_ref<'a, F: FromIterator<&'a Self::To>>(&self, source: &'a $ptr) -> F
                where
                    Self::To: 'a
                {
                    self.0.traverse_ref(source)
                }
            }

            impl<$($life,)* $param, Pm> PrismRef<$ptr> for $optic<Pm>
            where
                $param: ?Sized,
                Pm: PrismRef<$param>,
            {
                fn pm_ref<'a, F: From<Option<&'a Self::To>>>(&self, source: &'a $ptr) -> F
                where
                    Self::To: 'a
                {
                    self.0.pm_ref(source)
                }
            }

            impl<$($life,)* $param, Ls> LensRef<$ptr> for $optic<Ls>
            where
                $param: ?Sized,
                Ls: LensRef<$param>
            {
                fn view_ref<'a, F: From<&'a Self::To>>(&self, source: &'a $ptr) -> F
                where
                    Self::To: 'a
                {
                    self.0.view_ref(source)
                }
            }
        }
    }

    macro_rules! impl_mut {
        (<$($life:lifetime),*; $param:ident> $ptr:ty, $optic:ident) => {
            impl<$($life,)* $param, Tr> TraversalMut<$ptr> for $optic<Tr>
            where
                $param: ?Sized,
                Tr: TraversalMut<$param>,
            {
                fn traverse_mut<'a, F: FromIterator<&'a mut Self::To>>(&self, source: &'a mut $ptr) -> F
                where
                    Self::To: 'a
                {
                    self.0.traverse_mut(source)
                }
            }

            impl<$($life,)* $param, Pm> PrismMut<$ptr> for $optic<Pm>
            where
                $param: ?Sized,
                Pm: PrismMut<$param>,
            {
                fn pm_mut<'a, F: From<Option<&'a mut Self::To>>>(&self, source: &'a mut $ptr) -> F
                where
                    Self::To: 'a
                {
                    self.0.pm_mut(source)
                }
            }

            impl<$($life,)* $param, Ls> LensMut<$ptr> for $optic<Ls>
            where
                $param: ?Sized,
                Ls: LensMut<$param>
            {
                fn view_mut<'a, F: From<&'a mut Self::To>>(&self, source: &'a mut $ptr) -> F
                where
                    Self::To: 'a
                {
                    self.0.view_mut(source)
                }
            }
        }
    }

    impl<Rv, T> Review<Box<T>> for _box<Rv>
    where
        Rv: Review<T>,
    {
        type From = Rv::From;

        fn review<F: Into<Self::From>>(&self, from: F) -> Box<T> {
            Box::new(self.0.review(from.into()))
        }
    }

    impl<Tr, T> Traversal<Box<T>> for _box<Tr>
    where
        Tr: Traversal<T>,
    {
        fn traverse<F: FromIterator<Self::To>>(&self, source: Box<T>) -> F {
            self.0.traverse(*source)
        }
    }

    impl<Pm, T> Prism<Box<T>> for _box<Pm>
    where
        Pm: Prism<T>,
    {
        fn pm<F: From<Option<Self::To>>>(&self, source: Box<T>) -> F {
            self.0.pm(*source)
        }
    }

    impl<Ls, T> Lens<Box<T>> for _box<Ls>
    where
        Ls: Lens<T>,
    {
        fn view<F: From<Self::To>>(&self, source: Box<T>) -> F {
            self.0.view(*source)
        }
    }

    impl<Rv, T> Review<Rc<T>> for _rc<Rv>
    where
        Rv: Review<T>,
    {
        type From = Rv::From;

        fn review<F: Into<Self::From>>(&self, from: F) -> Rc<T> {
            Rc::new(self.0.review(from.into()))
        }
    }

    impl<Rv, T> Review<Arc<T>> for _arc<Rv>
    where
        Rv: Review<T>,
    {
        type From = Rv::From;

        fn review<F: Into<Self::From>>(&self, from: F) -> Arc<T> {
            Arc::new(self.0.review(from.into()))
        }
    }

    impl_ref!(<; T> Box<T>, _box);
    impl_ref!(<; T> Rc<T>, _rc);
    impl_ref!(<; T> Rc<T>, _arc);
    impl_ref!(<'t; T> &'t mut T, _mut);
    impl_ref!(<'t; T> &'t T, _ref);

    impl_mut!(<; T> Box<T>, _box);
    impl_mut!(<'t; T> &'t mut T, _mut);
}

include!(concat!(env!("OUT_DIR"), "/optics.rs"));