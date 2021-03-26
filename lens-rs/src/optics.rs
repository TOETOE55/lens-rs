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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _ix<I, Optic>(pub Optic, pub I);


//impls
mod impl__ {
    /***********************************************************
     * impl for __
     ************************************************************/
    use crate::*;

    impl<T> Review<T> for __ {
        type From = T;

        fn review(&self, from: Self::From) -> T {
            from
        }
    }

    impl<T> TraversalRef<T> for __ {
        type To = T;
        fn traverse_ref<'a>(&self, source: &'a T) -> Vec<&'a Self::To> {
            vec![source]
        }
    }

    impl<T> TraversalMut<T> for __ {
        fn traverse_mut<'a>(&self, source: &'a mut T) -> Vec<&'a mut Self::To> {
            vec![source]
        }
    }

    impl<T> Traversal<T> for __ {
        fn traverse(&self, source: T) -> Vec<Self::To> {
            vec![source]
        }
    }

    impl<T> PrismRef<T> for __ {
        fn pm_ref<'a>(&self, source: &'a T) -> Option<&'a Self::To> {
            Option::Some(source)
        }
    }

    impl<T> PrismMut<T> for __ {
        fn pm_mut<'a>(&self, source: &'a mut T) -> Option<&'a mut Self::To> {
            Option::Some(source)
        }
    }

    impl<T> Prism<T> for __ {
        fn pm(&self, source: T) -> Option<Self::To> {
            Option::Some(source)
        }
    }

    impl<T> LensRef<T> for __ {
        fn view_ref<'a>(&self, source: &'a T) -> &'a Self::To {
            source
        }
    }

    impl<T> LensMut<T> for __ {
        fn view_mut<'a>(&self, source: &'a mut T) -> &'a mut Self::To {
            source
        }
    }

    impl<T> Lens<T> for __ {
        fn view(&self, source: T) -> Self::To {
            source
        }
    }
}

mod impl_result {
    /***********************************************************
     * impl for Result
     ************************************************************/
    use crate::*;

    impl<Rv, T, E> Review<Result<T, E>> for optics::Ok<Rv>
    where
        Rv: Review<T>,
    {
        type From = Rv::From;

        fn review(&self, from: Self::From) -> Result<T, E> {
            Result::Ok(self.0.review(from))
        }
    }

    impl<Tr, T, E> TraversalRef<Result<T, E>> for optics::Ok<Tr>
    where
        Tr: TraversalRef<T>,
    {
        type To = Tr::To;
        fn traverse_ref<'a>(&self, source: &'a Result<T, E>) -> Vec<&'a Self::To> {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_ref(t))
                .collect()
        }
    }

    impl<Tr, T, E> TraversalMut<Result<T, E>> for optics::Ok<Tr>
    where
        Tr: TraversalMut<T>,
    {
        fn traverse_mut<'a>(&self, source: &'a mut Result<T, E>) -> Vec<&'a mut Self::To> {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_mut(t))
                .collect()
        }
    }

    impl<Tr, T, E> Traversal<Result<T, E>> for optics::Ok<Tr>
    where
        Tr: Traversal<T>,
    {
        fn traverse(&self, source: Result<T, E>) -> Vec<Self::To>
        where
            Self::To: Sized,
        {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse(t))
                .collect()
        }
    }

    impl<Tr, T, E> Traversal<Result<T, E>> for optics::Err<Tr>
    where
        Tr: Traversal<E>,
    {
        fn traverse(&self, source: Result<T, E>) -> Vec<Self::To>
        where
            Self::To: Sized,
        {
            source
                .err()
                .into_iter()
                .flat_map(|t| self.0.traverse(t))
                .collect()
        }
    }

    impl<Tr, T, E> TraversalRef<Result<T, E>> for optics::Err<Tr>
    where
        Tr: TraversalRef<E>,
    {
        type To = Tr::To;
        fn traverse_ref<'a>(&self, source: &'a Result<T, E>) -> Vec<&'a Self::To> {
            source
                .as_ref()
                .err()
                .into_iter()
                .flat_map(|t| self.0.traverse_ref(t))
                .collect()
        }
    }

    impl<Tr, T, E> TraversalMut<Result<T, E>> for optics::Err<Tr>
    where
        Tr: TraversalMut<E>,
    {
        fn traverse_mut<'a>(&self, source: &'a mut Result<T, E>) -> Vec<&'a mut Self::To> {
            source
                .as_mut()
                .err()
                .into_iter()
                .flat_map(|t| self.0.traverse_mut(t))
                .collect()
        }
    }

    impl<Pm, T, E> PrismRef<Result<T, E>> for optics::Ok<Pm>
    where
        Pm: PrismRef<T>,
    {
        fn pm_ref<'a>(&self, source: &'a Result<T, E>) -> Option<&'a Self::To> {
            source.as_ref().ok().and_then(|t| self.0.pm_ref(t))
        }
    }

    impl<Pm, T, E> PrismMut<Result<T, E>> for optics::Ok<Pm>
    where
        Pm: PrismMut<T>,
    {
        fn pm_mut<'a>(&self, source: &'a mut Result<T, E>) -> Option<&'a mut Self::To> {
            source.as_mut().ok().and_then(|t| self.0.pm_mut(t))
        }
    }

    impl<Pm, T, E> Prism<Result<T, E>> for optics::Ok<Pm>
    where
        Pm: Prism<T>,
    {
        fn pm(&self, source: Result<T, E>) -> Option<Self::To>
        where
            Self::To: Sized,
        {
            source.ok().and_then(|t| self.0.pm(t))
        }
    }

    impl<Rv, T, E> Review<Result<T, E>> for optics::Err<Rv>
    where
        Rv: Review<E>,
    {
        type From = Rv::From;

        fn review(&self, from: Self::From) -> Result<T, E> {
            Result::Err(self.0.review(from))
        }
    }

    impl<Pm, T, E> PrismRef<Result<T, E>> for optics::Err<Pm>
    where
        Pm: PrismRef<E>,
    {
        fn pm_ref<'a>(&self, source: &'a Result<T, E>) -> Option<&'a Self::To> {
            source.as_ref().err().and_then(|t| self.0.pm_ref(t))
        }
    }

    impl<Pm, T, E> PrismMut<Result<T, E>> for optics::Err<Pm>
    where
        Pm: PrismMut<E>,
    {
        fn pm_mut<'a>(&self, source: &'a mut Result<T, E>) -> Option<&'a mut Self::To> {
            source.as_mut().err().and_then(|t| self.0.pm_mut(t))
        }
    }

    impl<Pm, T, E> Prism<Result<T, E>> for optics::Err<Pm>
    where
        Pm: Prism<E>,
    {
        fn pm(&self, source: Result<T, E>) -> Option<Self::To>
        where
            Self::To: Sized,
        {
            source.err().and_then(|t| self.0.pm(t))
        }
    }
}

mod impl_some {
    /***********************************************************
     * impl for Option
     ************************************************************/

    use crate::*;

    impl<Rv, T> Review<Option<T>> for optics::Some<Rv>
    where
        Rv: Review<T>,
    {
        type From = Rv::From;

        fn review(&self, from: Self::From) -> Option<T> {
            Option::Some(self.0.review(from))
        }
    }

    impl<Tr, T> TraversalRef<Option<T>> for optics::Some<Tr>
    where
        Tr: TraversalRef<T>,
    {
        type To = Tr::To;

        fn traverse_ref<'a>(&self, source: &'a Option<T>) -> Vec<&'a Self::To> {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_ref(t))
                .collect()
        }
    }

    impl<Tr, T> TraversalMut<Option<T>> for optics::Some<Tr>
    where
        Tr: TraversalMut<T>,
    {
        fn traverse_mut<'a>(&self, source: &'a mut Option<T>) -> Vec<&'a mut Self::To> {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_mut(t))
                .collect()
        }
    }

    impl<Tr, T> Traversal<Option<T>> for optics::Some<Tr>
    where
        Tr: Traversal<T>,
    {
        fn traverse(&self, source: Option<T>) -> Vec<Self::To>
        where
            Self::To: Sized,
        {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse(t))
                .collect()
        }
    }

    impl<Pm, T> PrismRef<Option<T>> for optics::Some<Pm>
    where
        Pm: PrismRef<T>,
    {
        fn pm_ref<'a>(&self, source: &'a Option<T>) -> Option<&'a Self::To> {
            source.as_ref().and_then(|t| self.0.pm_ref(t))
        }
    }

    impl<Pm, T> PrismMut<Option<T>> for optics::Some<Pm>
    where
        Pm: PrismMut<T>,
    {
        fn pm_mut<'a>(&self, source: &'a mut Option<T>) -> Option<&'a mut Self::To> {
            source.as_mut().and_then(|t| self.0.pm_mut(t))
        }
    }

    impl<Pm, T> Prism<Option<T>> for optics::Some<Pm>
    where
        Pm: Prism<T>,
    {
        fn pm(&self, source: Option<T>) -> Option<Self::To>
        where
            Self::To: Sized,
        {
            source.and_then(|t| self.0.pm(t))
        }
    }

    impl<Rv, T> Review<Option<T>> for optics::None<Rv>
    where
        Rv: Review<()>,
    {
        type From = ();

        fn review(&self, _from: Self::From) -> Option<T> {
            Option::None
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
            impl<Tr, $($param,)*> TraversalRef<($($param,)*)> for $optic<Tr>
            where
                Tr: TraversalRef<$to>,
            {
                type To = Tr::To;

                fn traverse_ref<'a>(&self, source: &'a ($($param,)*)) -> Vec<&'a Self::To> {
                    self.0.traverse_ref(&source.$field)
                }
            }

            impl<Tr, $($param,)*> TraversalMut<($($param,)*)> for $optic<Tr>
            where
                Tr: TraversalMut<$to>,
            {
                fn traverse_mut<'a>(&self, source: &'a mut ($($param,)*)) -> Vec<&'a mut Self::To> {
                    self.0.traverse_mut(&mut source.$field)
                }
            }

            impl<Tr, $($param,)*> Traversal<($($param,)*)> for $optic<Tr>
            where
                Tr: Traversal<$to>,
            {
                fn traverse(&self, source: ($($param,)*)) -> Vec<Self::To> where Self::To: Sized {
                    self.0.traverse(source.$field)
                }
            }

            impl<Pm, $($param,)*> PrismRef<($($param,)*)> for $optic<Pm>
            where
                Pm: PrismRef<$to>,
            {

                fn pm_ref<'a>(&self, source: &'a ($($param,)*)) -> Option<&'a Self::To> {
                    self.0.pm_ref(&source.$field)
                }
            }

            impl<Pm, $($param,)*> PrismMut<($($param,)*)> for $optic<Pm>
            where
                Pm: PrismMut<$to>,
            {

                fn pm_mut<'a>(&self, source: &'a mut ($($param,)*)) -> Option<&'a mut Self::To> {
                    self.0.pm_mut(&mut source.$field)
                }
            }

            impl<Pm, $($param,)*> Prism<($($param,)*)> for $optic<Pm>
            where
                Pm: Prism<$to>,
            {

                fn pm(&self, source: ($($param,)*)) -> Option<Self::To> where Self::To: Sized {
                    self.0.pm(source.$field)
                }
            }

            impl<Ls, $($param,)* > LensRef<($($param,)*)> for $optic<Ls>
            where
                Ls: LensRef<$to>,
            {
                fn view_ref<'a>(&self, source: &'a ($($param,)*)) -> &'a Self::To {
                    self.0.view_ref(&source.$field)
                }
            }

            impl<Ls, $($param,)* > LensMut<($($param,)*)> for $optic<Ls>
            where
                Ls: LensMut<$to>,
            {
                fn view_mut<'a>(&self, source: &'a mut ($($param,)*)) -> &'a mut Self::To {
                    self.0.view_mut(&mut source.$field)
                }
            }

            impl<Ls, $($param,)* > Lens<($($param,)*)> for $optic<Ls>
            where
                Ls: Lens<$to>,
            {
                fn view(&self, source: ($($param,)*)) -> Self::To where Self::To: Sized {
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

    impl<Rv, A> Review<(A,)> for _0<Rv>
    where
        Rv: Review<A>,
    {
        type From = Rv::From;

        fn review(&self, from: Self::From) -> (A,) {
            (self.0.review(from),)
        }
    }

    macro_rules! impl_both {
        (<$param:ident> $tuple:ty, $($fields:tt),*) => {
            impl<Tr, $param> TraversalRef<$tuple> for _both<Tr>
            where
                Tr: TraversalRef<$param>
            {
                type To = Tr::To;

                fn traverse_ref<'a>(&self, source: &'a $tuple) -> Vec<&'a Self::To> {
                    let mut vec = vec![];
                    $(vec.extend(self.0.traverse_ref(&source.$fields));)*
                    vec
                }
            }

            impl<Tr, $param> TraversalMut<$tuple> for _both<Tr>
            where
                Tr: TraversalMut<$param>
            {
                fn traverse_mut<'a>(&self, source: &'a mut $tuple) -> Vec<&'a mut Self::To> {
                    let mut vec = vec![];
                    $(vec.extend(self.0.traverse_mut(&mut source.$fields));)*
                    vec
                }
            }

            impl<Tr, $param> Traversal<$tuple> for _both<Tr>
            where
                Tr: Traversal<$param>
            {
                fn traverse(&self, source: $tuple) -> Vec<Self::To> where Self::To: Sized {
                    let mut vec = vec![];
                    $(vec.extend(self.0.traverse(source.$fields));)*
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

    impl<Pm, A> PrismRef<(A,)> for _both<Pm>
    where
        Pm: PrismRef<A>,
    {
        fn pm_ref<'a>(&self, source: &'a (A,)) -> Option<&'a Self::To> {
            self.0.pm_ref(&source.0)
        }
    }

    impl<Pm, A> PrismMut<(A,)> for _both<Pm>
    where
        Pm: PrismMut<A>,
    {
        fn pm_mut<'a>(&self, source: &'a mut (A,)) -> Option<&'a mut Self::To> {
            self.0.pm_mut(&mut source.0)
        }
    }

    impl<Pm, A> Prism<(A,)> for _both<Pm>
    where
        Pm: Prism<A>,
    {
        fn pm(&self, source: (A,)) -> Option<Self::To>
        where
            Self::To: Sized,
        {
            self.0.pm(source.0)
        }
    }

    impl<Ls, A> LensRef<(A,)> for _both<Ls>
    where
        Ls: LensRef<A>,
    {
        fn view_ref<'a>(&self, source: &'a (A,)) -> &'a Self::To {
            self.0.view_ref(&source.0)
        }
    }

    impl<Ls, A> LensMut<(A,)> for _both<Ls>
    where
        Ls: LensMut<A>,
    {
        fn view_mut<'a>(&self, source: &'a mut (A,)) -> &'a mut Self::To {
            self.0.view_mut(&mut source.0)
        }
    }

    impl<Ls, A> Lens<(A,)> for _both<Ls>
    where
        Ls: Lens<A>,
    {
        fn view(&self, source: (A,)) -> Self::To
        where
            Self::To: Sized,
        {
            self.0.view(source.0)
        }
    }

    impl<Rv, A> Review<(A,)> for _both<Rv>
    where
        Rv: Review<A>,
    {
        type From = Rv::From;

        fn review(&self, from: Self::From) -> (A,) {
            (self.0.review(from),)
        }
    }
}

mod impl_iters {
    /***********************************************************
     * impl for iter
     ************************************************************/
    use crate::*;
    use std::collections::*;

    macro_rules! impl_iter {
        (<$param:ident> $iter:ty) => {
            impl<Tr, $param> TraversalRef<$iter> for _mapped<Tr>
            where
                Tr: TraversalRef<$param>,
            {
                type To = Tr::To;

                fn traverse_ref<'a>(&self, source: &'a $iter) -> Vec<&'a Self::To> {
                    source
                        .into_iter()
                        .flat_map(|t| self.0.traverse_ref(t))
                        .collect()
                }
            }

            impl<Tr, $param> TraversalMut<$iter> for _mapped<Tr>
            where
                Tr: TraversalMut<$param>,
            {
                fn traverse_mut<'a>(&self, source: &'a mut $iter) -> Vec<&'a mut Self::To> {
                    source
                        .into_iter()
                        .flat_map(|t| self.0.traverse_mut(t))
                        .collect()
                }
            }

            impl<Tr, $param> Traversal<$iter> for _mapped<Tr>
            where
                Tr: Traversal<$param>,
            {
                fn traverse(&self, source: $iter) -> Vec<Self::To>
                where
                    Self::To: Sized,
                {
                    source
                        .into_iter()
                        .flat_map(|t| self.0.traverse(t))
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
    use std::ops::Deref;
    use std::rc::Rc;
    use std::sync::Arc;

    macro_rules! impl_ref {
        (<$($life:lifetime),*; $param:ident> $ptr:ty, $optic:ident) => {
            impl<$($life,)* $param: ?Sized, Tr> TraversalRef<$ptr> for $optic<Tr>
            where
                Tr: TraversalRef<<$ptr as Deref>::Target>,
            {
                type To = Tr::To;

                fn traverse_ref<'a>(&self, source: &'a $ptr) -> Vec<&'a Self::To> {
                    self.0.traverse_ref(source)
                }
            }

            impl<$($life,)* $param: ?Sized, Pm> PrismRef<$ptr> for $optic<Pm>
                where
                    Pm: PrismRef<<$ptr as Deref>::Target>,
            {
                fn pm_ref<'a>(&self, source: &'a $ptr) -> Option<&'a Self::To> {
                    self.0.pm_ref(source)
                }
            }

            impl<$($life,)* $param: ?Sized, Ls> LensRef<$ptr> for $optic<Ls>
                where
                    Ls: LensRef<<$ptr as Deref>::Target>
            {
                fn view_ref<'a>(&self, source: &'a $ptr) -> &'a Self::To {
                    self.0.view_ref(source)
                }
            }
        }
    }

    macro_rules! impl_mut {
        (<$($life:lifetime),*; $param:ident> $ptr:ty, $optic:ident) => {
            impl<$($life,)* $param: ?Sized, Tr> TraversalMut<$ptr> for $optic<Tr>
            where
                Tr: TraversalMut<<$ptr as Deref>::Target>,
            {
                fn traverse_mut<'a>(&self, source: &'a mut $ptr) -> Vec<&'a mut Self::To> {
                    self.0.traverse_mut(source)
                }
            }

            impl<$($life,)* $param: ?Sized, Pm> PrismMut<$ptr> for $optic<Pm>
            where
                Pm: PrismMut<<$ptr as Deref>::Target>,
            {
                fn pm_mut<'a>(&self, source: &'a mut $ptr) -> Option<&'a mut Self::To> {
                    self.0.pm_mut(source)
                }
            }

            impl<$($life,)* $param: ?Sized, Ls> LensMut<$ptr> for $optic<Ls>
            where
                Ls: LensMut<<$ptr as Deref>::Target>
            {
                fn view_mut<'a>(&self, source: &'a mut $ptr) -> &'a mut Self::To {
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

        fn review(&self, from: Self::From) -> Box<T> {
            Box::new(self.0.review(from))
        }
    }

    impl<Tr, T> Traversal<Box<T>> for _box<Tr>
    where
        Tr: Traversal<T>,
    {
        fn traverse(&self, source: Box<T>) -> Vec<Self::To>
        where
            Self::To: Sized,
        {
            self.0.traverse(*source)
        }
    }

    impl<Pm, T> Prism<Box<T>> for _box<Pm>
    where
        Pm: Prism<T>,
    {
        fn pm(&self, source: Box<T>) -> Option<Self::To>
        where
            Self::To: Sized,
        {
            self.0.pm(*source)
        }
    }

    impl<Ls, T> Lens<Box<T>> for _box<Ls>
    where
        Ls: Lens<T>,
    {
        fn view(&self, source: Box<T>) -> Self::To
        where
            Self::To: Sized,
        {
            self.0.view(*source)
        }
    }

    impl<Rv, T> Review<Rc<T>> for _rc<Rv>
    where
        Rv: Review<T>,
    {
        type From = Rv::From;

        fn review(&self, from: Self::From) -> Rc<T> {
            Rc::new(self.0.review(from))
        }
    }

    impl<Rv, T> Review<Arc<T>> for _arc<Rv>
    where
        Rv: Review<T>,
    {
        type From = Rv::From;

        fn review(&self, from: Self::From) -> Arc<T> {
            Arc::new(self.0.review(from))
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

mod impl_ix {
    use crate::*;
    use std::borrow::Borrow;
    use std::collections::{BTreeMap, HashMap, VecDeque};
    use std::hash::Hash;
    use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

    macro_rules! impl_ix {
        (<$($param:ident,)? $(const $c:ident: usize)?> $t:ty[$ix:ty]: $o:ty) => {
            impl<$($param,)? Tr, $(const $c : usize)?> TraversalRef<$t> for _ix<$ix, Tr>
            where
                Tr: TraversalRef<$o>,
            {
                type To = Tr::To;

                fn traverse_ref<'a>(&self, source: &'a $t) -> Vec<&'a Self::To> {
                    self.0.traverse_ref(&source[self.1.clone()])
                }
            }

            impl<$($param,)? Tr, $(const $c: usize)?> TraversalMut<$t> for _ix<$ix, Tr>
            where
                Tr: TraversalMut<$o>,
            {
                fn traverse_mut<'a>(&self, source: &'a mut $t) -> Vec<&'a mut Self::To> {
                    self.0.traverse_mut(&mut source[self.1.clone()])
                }
            }

            impl<$($param,)? Pm, $(const $c: usize)?> PrismRef<$t> for _ix<$ix, Pm>
            where
                Pm: PrismRef<$o>,
            {

                fn pm_ref<'a>(&self, source: &'a $t) -> Option<&'a Self::To> {
                    self.0.pm_ref(&source[self.1.clone()])
                }
            }

            impl<$($param,)? Pm, $(const $c: usize)?> PrismMut<$t> for _ix<$ix, Pm>
            where
                Pm: PrismMut<$o>,
            {
                fn pm_mut<'a>(&self, source: &'a mut $t) -> Option<&'a mut Self::To> {
                    self.0.pm_mut(&mut source[self.1.clone()])
                }
            }

            impl<$($param,)? Ls, $(const $c: usize)?> LensRef<$t> for _ix<$ix, Ls>
            where
                Ls: LensRef<$o>,
            {

                fn view_ref<'a>(&self, source: &'a $t) -> &'a Self::To {
                    self.0.view_ref(&source[self.1.clone()])
                }
            }

            impl<$($param,)? Ls, $(const $c: usize)?> LensMut<$t> for _ix<$ix, Ls>
            where
                Ls: LensMut<$o>,
            {
                fn view_mut<'a>(&self, source: &'a mut $t) -> &'a mut Self::To {
                    self.0.view_mut(&mut source[self.1.clone()])
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

    impl<K, Q: ?Sized, V, Tr> TraversalRef<BTreeMap<K, V>> for _ix<&'_ Q, Tr>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
        Tr: TraversalRef<V>,
    {
        type To = Tr::To;

        fn traverse_ref<'a>(&self, source: &'a BTreeMap<K, V>) -> Vec<&'a Self::To> {
            self.0.traverse_ref(&source[self.1])
        }
    }

    impl<K, Q: ?Sized, V, Tr> TraversalRef<HashMap<K, V>> for _ix<&'_ Q, Tr>
    where
        K: Eq + Hash + Borrow<Q>,
        Q: Eq + Hash,
        Tr: TraversalRef<V>,
    {
        type To = Tr::To;

        fn traverse_ref<'a>(&self, source: &'a HashMap<K, V>) -> Vec<&'a Self::To> {
            self.0.traverse_ref(&source[self.1])
        }
    }

    impl<K, Q: ?Sized, V, Tr> PrismRef<BTreeMap<K, V>> for _ix<&'_ Q, Tr>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
        Tr: PrismRef<V>,
    {
        fn pm_ref<'a>(&self, source: &'a BTreeMap<K, V>) -> Option<&'a Self::To> {
            self.0.pm_ref(&source[self.1])
        }
    }

    impl<K, Q: ?Sized, V, Tr> PrismRef<HashMap<K, V>> for _ix<&'_ Q, Tr>
    where
        K: Eq + Hash + Borrow<Q>,
        Q: Eq + Hash,
        Tr: PrismRef<V>,
    {
        fn pm_ref<'a>(&self, source: &'a HashMap<K, V>) -> Option<&'a Self::To> {
            self.0.pm_ref(&source[self.1])
        }
    }

    impl<K, Q: ?Sized, V, Tr> LensRef<BTreeMap<K, V>> for _ix<&'_ Q, Tr>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
        Tr: LensRef<V>,
    {
        fn view_ref<'a>(&self, source: &'a BTreeMap<K, V>) -> &'a Self::To {
            self.0.view_ref(&source[self.1])
        }
    }

    impl<K, Q: ?Sized, V, Tr> LensRef<HashMap<K, V>> for _ix<&'_ Q, Tr>
    where
        K: Eq + Hash + Borrow<Q>,
        Q: Eq + Hash,
        Tr: LensRef<V>,
    {
        fn view_ref<'a>(&self, source: &'a HashMap<K, V>) -> &'a Self::To {
            self.0.view_ref(&source[self.1])
        }
    }
}


include!(concat!(env!("OUT_DIR"), "/optics.rs"));
