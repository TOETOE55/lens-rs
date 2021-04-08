pub use lens_rs_generator::*;

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
//
// mod impl_result {
//     /***********************************************************
//      * impl for Result
//      ************************************************************/
//     use crate::*;
//
//     impl<Rv, Image, T, E> Review<optics::Ok<Rv>, Image> for Result<T, E>
//     where
//         T: Review<Rv, Image>,
//     {
//         fn review(optics: optics::Ok<Rv>, from: Image) -> Self {
//             Result::Ok(Review::review(optics.0, from))
//         }
//     }
//
//     impl<Tr, Image: ?Sized, T, E> TraversalRef<optics::Ok<Tr>, Image> for Result<T, E>
//     where
//         T: TraversalRef<Tr, Image>,
//     {
//         fn traverse_ref(&self, optics: optics::Ok<Tr>) -> Vec<&Image> {
//             match self {
//                 Result::Ok(t) => t.traverse_ref(optics.0),
//                 _ => vec![],
//             }
//         }
//     }
//
//     impl<Tr, Image: ?Sized, T, E> TraversalMut<optics::Ok<Tr>, Image> for Result<T, E>
//     where
//         T: TraversalMut<Tr, Image>,
//     {
//         fn traverse_mut(&mut self, optics: optics::Ok<Tr>) -> Vec<&mut Image> {
//             match self {
//                 Result::Ok(t) => t.traverse_mut(optics.0),
//                 _ => vec![],
//             }
//         }
//     }
//
//     impl<Tr, Image, T, E> Traversal<optics::Ok<Tr>, Image> for Result<T, E>
//     where
//         T: Traversal<Tr, Image>,
//     {
//         fn traverse(self, optics: optics::Ok<Tr>) -> Vec<Image>
//         where
//             Self: Sized,
//         {
//             match self {
//                 Result::Ok(t) => t.traverse(optics.0),
//                 _ => vec![],
//             }
//         }
//     }
//
//     impl<Tr, Image, T, E> Traversal<optics::Err<Tr>, Image> for Result<T, E>
//     where
//         E: Traversal<Tr, Image>,
//     {
//         fn traverse(self, optics: optics::Err<Tr>) -> Vec<Image>
//         where
//             Self: Sized,
//         {
//             match self {
//                 Result::Err(e) => e.traverse(optics.0),
//                 _ => vec![],
//             }
//         }
//     }
//
//     impl<Tr, Image: ?Sized, T, E> TraversalRef<optics::Err<Tr>, Image> for Result<T, E>
//     where
//         E: TraversalRef<Tr, Image>,
//     {
//         fn traverse_ref(&self, optics: optics::Err<Tr>) -> Vec<&Image> {
//             match self {
//                 Result::Err(e) => e.traverse_ref(optics.0),
//                 _ => vec![],
//             }
//         }
//     }
//
//     impl<Tr, Image: ?Sized, T, E> TraversalMut<optics::Err<Tr>, Image> for Result<T, E>
//     where
//         E: TraversalMut<Tr, Image>,
//     {
//         fn traverse_mut(&mut self, optics: optics::Err<Tr>) -> Vec<&mut Image> {
//             match self {
//                 Result::Err(e) => e.traverse_mut(optics.0),
//                 _ => vec![],
//             }
//         }
//     }
//
//     impl<Pm, Image: ?Sized, T, E> PrismRef<optics::Ok<Pm>, Image> for Result<T, E>
//     where
//         T: PrismRef<Pm, Image>,
//     {
//         fn preview_ref(&self, optics: optics::Ok<Pm>) -> Option<&Image> {
//             let t = self.as_ref().ok()?;
//             t.preview_ref(optics.0)
//         }
//     }
//
//     impl<Pm, Image: ?Sized, T, E> PrismMut<optics::Ok<Pm>, Image> for Result<T, E>
//     where
//         T: PrismMut<Pm, Image>,
//     {
//         fn preview_mut(&mut self, optics: optics::Ok<Pm>) -> Option<&mut Image> {
//             let t = self.as_mut().ok()?;
//             t.preview_mut(optics.0)
//         }
//     }
//
//     impl<Pm, Image, T, E> Prism<optics::Ok<Pm>, Image> for Result<T, E>
//     where
//         T: Prism<Pm, Image>,
//     {
//         fn preview(self, optics: optics::Ok<Pm>) -> Option<Image>
//         where
//             Self: Sized,
//         {
//             let t = self.ok()?;
//             t.preview(optics.0)
//         }
//     }
//
//     impl<Rv, Image, T, E> Review<optics::Err<Rv>, Image> for Result<T, E>
//     where
//         E: Review<Rv, Image>,
//     {
//         fn review(optics: optics::Err<Rv>, from: Image) -> Self {
//             Result::Err(Review::review(optics.0, from))
//         }
//     }
//
//     impl<Pm, Image: ?Sized, T, E> PrismRef<optics::Err<Pm>, Image> for Result<T, E>
//     where
//         E: PrismRef<Pm, Image>,
//     {
//         fn preview_ref(&self, optics: optics::Err<Pm>) -> Option<&Image> {
//             let t = self.as_ref().err()?;
//             t.preview_ref(optics.0)
//         }
//     }
//
//     impl<Pm, Image: ?Sized, T, E> PrismMut<optics::Err<Pm>, Image> for Result<T, E>
//     where
//         E: PrismMut<Pm, Image>,
//     {
//         fn preview_mut(&mut self, optics: optics::Err<Pm>) -> Option<&mut Image> {
//             let t = self.as_mut().err()?;
//             t.preview_mut(optics.0)
//         }
//     }
//
//     impl<Pm, Image, T, E> Prism<optics::Err<Pm>, Image> for Result<T, E>
//     where
//         E: Prism<Pm, Image>,
//     {
//         fn preview(self, optics: optics::Err<Pm>) -> Option<Image>
//         where
//             Self: Sized,
//         {
//             let t = self.err()?;
//             t.preview(optics.0)
//         }
//     }
// }
//
// mod impl_some {
//     /***********************************************************
//      * impl for Option
//      ************************************************************/
//
//     use crate::*;
//
//     impl<Rv, Image, T> Review<optics::Some<Rv>, Image> for Option<T>
//     where
//         T: Review<Rv, Image>,
//     {
//         fn review(optics: optics::Some<Rv>, from: Image) -> Self {
//             Some(Review::review(optics.0, from))
//         }
//     }
//
//     impl<Tr, Image: ?Sized, T> TraversalRef<optics::Some<Tr>, Image> for Option<T>
//     where
//         T: TraversalRef<Tr, Image>,
//     {
//         fn traverse_ref(&self, optics: optics::Some<Tr>) -> Vec<&Image> {
//             match self {
//                 Some(t) => t.traverse_ref(optics.0),
//                 _ => vec![],
//             }
//         }
//     }
//
//     impl<Tr, Image: ?Sized, T> TraversalMut<optics::Some<Tr>, Image> for Option<T>
//     where
//         T: TraversalMut<Tr, Image>,
//     {
//         fn traverse_mut(&mut self, optics: optics::Some<Tr>) -> Vec<&mut Image> {
//             match self {
//                 Some(t) => t.traverse_mut(optics.0),
//                 _ => vec![],
//             }
//         }
//     }
//
//     impl<Tr, Image, T> Traversal<optics::Some<Tr>, Image> for Option<T>
//     where
//         T: Traversal<Tr, Image>,
//     {
//         fn traverse(self, optics: optics::Some<Tr>) -> Vec<Image>
//         where
//             Self: Sized,
//         {
//             match self {
//                 Some(t) => t.traverse(optics.0),
//                 _ => vec![],
//             }
//         }
//     }
//
//     impl<Pm, Image: ?Sized, T> PrismRef<optics::Some<Pm>, Image> for Option<T>
//     where
//         T: PrismRef<Pm, Image>,
//     {
//         fn preview_ref(&self, optics: optics::Some<Pm>) -> Option<&Image> {
//             self.as_ref()?.preview_ref(optics.0)
//         }
//     }
//
//     impl<Pm, Image: ?Sized, T> PrismMut<optics::Some<Pm>, Image> for Option<T>
//     where
//         T: PrismMut<Pm, Image>,
//     {
//         fn preview_mut(&mut self, optics: optics::Some<Pm>) -> Option<&mut Image> {
//             self.as_mut()?.preview_mut(optics.0)
//         }
//     }
//
//     impl<Pm, Image, T> Prism<optics::Some<Pm>, Image> for Option<T>
//     where
//         T: Prism<Pm, Image>,
//     {
//         fn preview(self, optics: optics::Some<Pm>) -> Option<Image>
//         where
//             Self: Sized,
//         {
//             self?.preview(optics.0)
//         }
//     }
//
//     impl<Rv, Image, T> Review<optics::None<Rv>, Image> for Option<T> {
//         fn review(_optics: optics::None<Rv>, _from: Image) -> Self {
//             None
//         }
//     }
// }

mod impl_tuples {

    /***********************************************************
     * impl for tuple
     ************************************************************/
    use crate::*;
    use lens_rs_derive::derive_lens_for_tuple;
    mod lens_rs {
        pub use crate::*;
        pub mod optics {
            pub use lens_rs_generator::*;
        }
    }

    derive_lens_for_tuple!((A,));
    derive_lens_for_tuple!((A, B));
    derive_lens_for_tuple!((A, B, C));
    derive_lens_for_tuple!((A, B, C, D));
    derive_lens_for_tuple!((A, B, C, D, E));
    derive_lens_for_tuple!((A, B, C, D, E, F));
    derive_lens_for_tuple!((A, B, C, D, E, F, G));
    derive_lens_for_tuple!((A, B, C, D, E, F, G, H));
    derive_lens_for_tuple!((A, B, C, D, E, F, G, H, I));
    derive_lens_for_tuple!((A, B, C, D, E, F, G, H, I, J));
    derive_lens_for_tuple!((A, B, C, D, E, F, G, H, I, J, K));
    derive_lens_for_tuple!((A, B, C, D, E, F, G, H, I, J, K, L));
    derive_lens_for_tuple!((A, B, C, D, E, F, G, H, I, J, K, L, M));
    derive_lens_for_tuple!((A, B, C, D, E, F, G, H, I, J, K, L, M, N));
    derive_lens_for_tuple!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O));
    derive_lens_for_tuple!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P));
    derive_lens_for_tuple!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q));

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
    impl_both!(<A> (A, A, A, A, A, A, A, A), 0, 1, 2, 3, 4, 5, 6, 7);
    impl_both!(<A> (A, A, A, A, A, A, A, A, A), 0, 1, 2, 3, 4, 5, 6, 7, 8);
    impl_both!(<A> (A, A, A, A, A, A, A, A, A, A), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    impl_both!(<A> (A, A, A, A, A, A, A, A, A, A, A), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    impl_both!(<A> (A, A, A, A, A, A, A, A, A, A, A, A), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    impl_both!(<A> (A, A, A, A, A, A, A, A, A, A, A, A, A), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    impl_both!(<A> (A, A, A, A, A, A, A, A, A, A, A, A, A, A), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    impl_both!(<A> (A, A, A, A, A, A, A, A, A, A, A, A, A, A, A), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
    impl_both!(<A> (A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    impl_both!(<A> (A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

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

mod impl_builtin {
    mod lens_rs {
        pub use crate::*;
        pub mod optics {
            pub use lens_rs_generator::*;
        }
    }

    lens_rs_derive::derive_review_for_builtin! {
        enum Option<T> {
            #[optic]
            Some(T),
            #[optic]
            None,
        }
    }

    lens_rs_derive::derive_prism_for_builtin! {
        enum Option<T> {
            #[optic]
            Some(T),
            #[optic]
            None,
        }
    }

    lens_rs_derive::derive_review_for_builtin! {
        enum Result<T, E> {
            #[optic]
            Ok(T),
            #[optic]
            Err(E),
        }
    }

    lens_rs_derive::derive_prism_for_builtin! {
        enum Result<T, E> {
            #[optic]
            Ok(T),
            #[optic]
            Err(E),
        }
    }
}
