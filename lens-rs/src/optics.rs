#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct __;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _Ok<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _Err<Optic>(pub Optic);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _Some<Optic>(pub Optic);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _None;

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
pub struct Both<Optic>(pub Optic);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mapped<Optic>(pub Optic);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _Box<Optic>(pub Optic);

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

    impl<T> Traversal<T> for __ {
        type To = T;

        fn traverse(&self, source: T) -> Vec<Self::To> {
            self.pm(source).into_iter().collect()
        }

        fn traverse_ref<'a>(&self, source: &'a T) -> Vec<&'a Self::To> {
            self.pm_ref(source).into_iter().collect()
        }

        fn traverse_mut<'a>(&self, source: &'a mut T) -> Vec<&'a mut Self::To> {
            self.pm_mut(source).into_iter().collect()
        }
    }

    impl<T> Prism<T> for __ {
        type To = T;

        fn pm(&self, source: T) -> Option<Self::To> {
            Some(self.view(source))
        }

        fn pm_ref<'a>(&self, source: &'a T) -> Option<&'a Self::To> {
            Some(self.view_ref(source))
        }

        fn pm_mut<'a>(&self, source: &'a mut T) -> Option<&'a mut Self::To> {
            Some(self.view_mut(source))
        }
    }

    impl<T> Lens<T> for __ {
        type To = T;

        fn view(&self, source: T) -> Self::To {
            source
        }

        fn view_ref<'a>(&self, source: &'a T) -> &'a Self::To {
            source
        }

        fn view_mut<'a>(&self, source: &'a mut T) -> &'a mut Self::To {
            source
        }
    }
}

mod impl_result {
    /***********************************************************
     * impl for Result
     ************************************************************/
    use crate::*;

    impl<Rv, T, E> Review<Result<T, E>> for _Ok<Rv>
    where
        Rv: Review<T>,
    {
        type From = Rv::From;

        fn review(&self, from: Self::From) -> Result<T, E> {
            Ok(self.0.review(from))
        }
    }

    impl<Tr, T, E> Traversal<Result<T, E>> for _Ok<Tr>
    where
        Tr: Traversal<T>,
    {
        type To = Tr::To;

        fn traverse(&self, source: Result<T, E>) -> Vec<Self::To> {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse(t))
                .collect()
        }

        fn traverse_ref<'a>(&self, source: &'a Result<T, E>) -> Vec<&'a Self::To> {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_ref(t))
                .collect()
        }

        fn traverse_mut<'a>(&self, source: &'a mut Result<T, E>) -> Vec<&'a mut Self::To> {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_mut(t))
                .collect()
        }
    }

    impl<Rv, T, E> Review<Result<T, E>> for _Err<Rv>
    where
        Rv: Review<E>,
    {
        type From = Rv::From;

        fn review(&self, from: Self::From) -> Result<T, E> {
            Err(self.0.review(from))
        }
    }

    impl<Pm, T, E> Prism<Result<T, E>> for _Ok<Pm>
    where
        Pm: Prism<T>,
    {
        type To = Pm::To;

        fn pm(&self, source: Result<T, E>) -> Option<Self::To> {
            source.ok().and_then(|t| self.0.pm(t))
        }

        fn pm_ref<'a>(&self, source: &'a Result<T, E>) -> Option<&'a Self::To> {
            source.as_ref().ok().and_then(|t| self.0.pm_ref(t))
        }

        fn pm_mut<'a>(&self, source: &'a mut Result<T, E>) -> Option<&'a mut Self::To> {
            source.as_mut().ok().and_then(|t| self.0.pm_mut(t))
        }
    }

    impl<Pm, T, E> Prism<Result<T, E>> for _Err<Pm>
    where
        Pm: Prism<E>,
    {
        type To = Pm::To;

        fn pm(&self, source: Result<T, E>) -> Option<Self::To> {
            source.err().and_then(|t| self.0.pm(t))
        }

        fn pm_ref<'a>(&self, source: &'a Result<T, E>) -> Option<&'a Self::To> {
            source.as_ref().err().and_then(|t| self.0.pm_ref(t))
        }

        fn pm_mut<'a>(&self, source: &'a mut Result<T, E>) -> Option<&'a mut Self::To> {
            source.as_mut().err().and_then(|t| self.0.pm_mut(t))
        }
    }
}

mod impl_some {
    /***********************************************************
     * impl for Option
     ************************************************************/

    use crate::*;

    impl<Rv, T> Review<Option<T>> for _Some<Rv>
    where
        Rv: Review<T>,
    {
        type From = Rv::From;

        fn review(&self, from: Self::From) -> Option<T> {
            Some(self.0.review(from))
        }
    }

    impl<Tr, T> Traversal<Option<T>> for _Some<Tr>
    where
        Tr: Traversal<T>,
    {
        type To = Tr::To;

        fn traverse(&self, source: Option<T>) -> Vec<Self::To> {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse(t))
                .collect()
        }

        fn traverse_ref<'a>(&self, source: &'a Option<T>) -> Vec<&'a Self::To> {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_ref(t))
                .collect()
        }

        fn traverse_mut<'a>(&self, source: &'a mut Option<T>) -> Vec<&'a mut Self::To> {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_mut(t))
                .collect()
        }
    }

    impl<Pm, T> Prism<Option<T>> for _Some<Pm>
    where
        Pm: Prism<T>,
    {
        type To = Pm::To;

        fn pm(&self, source: Option<T>) -> Option<Self::To> {
            source.and_then(|t| self.0.pm(t))
        }

        fn pm_ref<'a>(&self, source: &'a Option<T>) -> Option<&'a Self::To> {
            source.as_ref().and_then(|t| self.0.pm_ref(t))
        }

        fn pm_mut<'a>(&self, source: &'a mut Option<T>) -> Option<&'a mut Self::To> {
            source.as_mut().and_then(|t| self.0.pm_mut(t))
        }
    }

    impl<T> Review<Option<T>> for _None {
        type From = ();

        fn review(&self, _from: Self::From) -> Option<T> {
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

            impl<Tr, $($param,)*> Traversal<($($param,)*)> for $optic<Tr>
            where
                Tr: Traversal<$to>,
            {
                type To = Tr::To;

                fn traverse(&self, source: ($($param,)*)) -> Vec<Self::To> {
                    self.0.traverse(source.$field)
                }

                fn traverse_ref<'a>(&self, source: &'a ($($param,)*)) -> Vec<&'a Self::To> {
                    self.0.traverse_ref(&source.$field)
                }

                fn traverse_mut<'a>(&self, source: &'a mut ($($param,)*)) -> Vec<&'a mut Self::To> {
                    self.0.traverse_mut(&mut source.$field)
                }
            }

            impl<Pm, $($param,)*> Prism<($($param,)*)> for $optic<Pm>
            where
                Pm: Prism<$to>,
            {
                type To = Pm::To;

                fn pm(&self, source: ($($param,)*)) -> Option<Self::To> {
                    self.0.pm(source.$field)
                }

                fn pm_ref<'a>(&self, source: &'a ($($param,)*)) -> Option<&'a Self::To> {
                    self.0.pm_ref(&source.$field)
                }

                fn pm_mut<'a>(&self, source: &'a mut ($($param,)*)) -> Option<&'a mut Self::To> {
                    self.0.pm_mut(&mut source.$field)
                }
            }

            impl<Ls, $($param,)* > Lens<($($param,)*)> for $optic<Ls>
            where
                Ls: Lens<$to>,
            {
                type To = Ls::To;

                fn view(&self, source: ($($param,)*)) -> Self::To {
                    self.0.view(source.$field)
                }

                fn view_ref<'a>(&self, source: &'a ($($param,)*)) -> &'a Self::To {
                    self.0.view_ref(&source.$field)
                }

                fn view_mut<'a>(&self, source: &'a mut ($($param,)*)) -> &'a mut Self::To {
                    self.0.view_mut(&mut source.$field)
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
        Rv: Review<A>
    {
        type From = Rv::From;

        fn review(&self, from: Self::From) -> (A,) {
            (self.0.review(from),)
        }
    }

    macro_rules! impl_both {
        (<$param:ident> $tuple:ty, $($fields:tt),*) => {
            impl<Tr, $param> Traversal<$tuple> for Both<Tr>
            where
                Tr: Traversal<$param>
            {
                type To = Tr::To;

                fn traverse(&self, source: $tuple) -> Vec<Self::To> {
                    let mut vec = vec![];
                    $(vec.extend(self.0.traverse(source.$fields));)*
                    vec
                }

                fn traverse_ref<'a>(&self, source: &'a $tuple) -> Vec<&'a Self::To> {
                    let mut vec = vec![];
                    $(vec.extend(self.0.traverse_ref(&source.$fields));)*
                    vec
                }

                fn traverse_mut<'a>(&self, source: &'a mut $tuple) -> Vec<&'a mut Self::To> {
                    let mut vec = vec![];
                    $(vec.extend(self.0.traverse_mut(&mut source.$fields));)*
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

    impl<Pm, A> Prism<(A,)> for Both<Pm>
    where
        Pm: Prism<A>,
    {
        type To = Pm::To;

        fn pm(&self, source: (A,)) -> Option<Self::To> {
            self.0.pm(source.0)
        }

        fn pm_ref<'a>(&self, source: &'a (A,)) -> Option<&'a Self::To> {
            self.0.pm_ref(&source.0)
        }

        fn pm_mut<'a>(&self, source: &'a mut (A,)) -> Option<&'a mut Self::To> {
            self.0.pm_mut(&mut source.0)
        }
    }

    impl<Ls, A> Lens<(A,)> for Both<Ls>
    where
        Ls: Lens<A>
    {
        type To = Ls::To;

        fn view(&self, source: (A,)) -> Self::To {
            self.0.view(source.0)
        }

        fn view_ref<'a>(&self, source: &'a (A,)) -> &'a Self::To {
            self.0.view_ref(&source.0)
        }

        fn view_mut<'a>(&self, source: &'a mut (A,)) -> &'a mut Self::To {
            self.0.view_mut(&mut source.0)
        }
    }

    impl<Rv, A> Review<(A,)> for Both<Rv>
    where
        Rv: Review<A>
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
        (<$($param:ident)*> $iter:ty) => {
    impl<Tr, $($param,)*> Traversal<$iter> for Mapped<Tr>
    where
        Tr: Traversal<<$iter as IntoIterator>::Item>,
    {
        type To = Tr::To;

        fn traverse(&self, source: $iter) -> Vec<Self::To> {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse(t))
                .collect()
        }

        fn traverse_ref<'a>(&self, source: &'a $iter) -> Vec<&'a Self::To> {
            source.into_iter().flat_map(|t| self.0.traverse_ref(t)).collect()
        }

        fn traverse_mut<'a>(&self, source: &'a mut $iter) -> Vec<&'a mut Self::To> {
            source
                .into_iter()
                .flat_map(|t| self.0.traverse_mut(t))
                .collect()
        }
    }
        }
    }

    impl_iter!(<T> Vec<T>);
    impl_iter!(<T> VecDeque<T>);
    impl_iter!(<T> LinkedList<T>);
}


mod impl_box {
    use crate::*;

    impl<Rv, T> Review<Box<T>> for _Box<Rv>
    where
        Rv: Review<T>,
    {
        type From = Rv::From;

        fn review(&self, from: Self::From) -> Box<T> {
            Box::new(self.0.review(from))
        }
    }

    impl<Tr, T> Traversal<Box<T>> for _Box<Tr>
    where
        Tr: Traversal<T>,
    {
        type To = Tr::To;

        fn traverse(&self, source: Box<T>) -> Vec<Self::To> {
            self.0.traverse(*source)
        }

        fn traverse_ref<'a>(&self, source: &'a Box<T>) -> Vec<&'a Self::To> {
            self.0.traverse_ref(source)
        }

        fn traverse_mut<'a>(&self, source: &'a mut Box<T>) -> Vec<&'a mut Self::To> {
            self.0.traverse_mut(source)
        }
    }

    impl<Pm, T> Prism<Box<T>> for _Box<Pm>
    where
        Pm: Prism<T>,
    {
        type To = Pm::To;

        fn pm(&self, source: Box<T>) -> Option<Self::To> {
            self.0.pm(*source)
        }

        fn pm_ref<'a>(&self, source: &'a Box<T>) -> Option<&'a Self::To> {
            self.0.pm_ref(source)
        }

        fn pm_mut<'a>(&self, source: &'a mut Box<T>) -> Option<&'a mut Self::To> {
            self.0.pm_mut(source)
        }
    }

    impl<Ls, T> Lens<Box<T>> for _Box<Ls>
    where
        Ls: Lens<T>
    {
        type To = Ls::To;

        fn view(&self, source: Box<T>) -> Self::To {
            self.0.view(*source)
        }

        fn view_ref<'a>(&self, source: &'a Box<T>) -> &'a Self::To {
            self.0.view_ref(source)
        }

        fn view_mut<'a>(&self, source: &'a mut Box<T>) -> &'a mut Self::To {
            self.0.view_mut(source)
        }
    }
}