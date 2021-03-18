use std::iter::once;
macro_rules! optics {
    () => { __ };
    ($optic:ident) => { $optic(__) };
    ($optic:ident . $($optics:tt)*) => {
        $optic(optics!($($optics)*))
    }
}

pub trait Review<T> {
    type From;
    fn review(&self, from: Self::From) -> T;
}

pub trait Traversal<T> {
    type To;
    fn traverse(&self, source: T) -> Vec<Self::To>;
    fn traverse_ref<'a>(&self, source: &'a T) -> Vec<&'a Self::To>;
    fn traverse_mut<'a>(&self, source: &'a mut T) -> Vec<&'a mut Self::To>;
}

pub trait Prism<T> {
    type To;
    fn pm(&self, source: T) -> Option<Self::To>;
    fn pm_ref<'a>(&self, source: &'a T) -> Option<&'a Self::To>;
    fn pm_mut<'a>(&self, source: &'a mut T) -> Option<&'a mut Self::To>;
}

pub trait Lens<T> {
    type To;
    fn view(&self, source: T) -> Self::To;
    fn view_ref<'a>(&self, source: &'a T) -> &'a Self::To;
    fn view_mut<'a>(&self, source: &'a mut T) -> &'a mut Self::To;
}

// impl<Pm: Prism<T>, T> Traversal<T> for Pm {
//     type To = Pm::To;
//
//     fn traverse(&self, source: T) -> Vec<Self::To> {
//         self.pm(source).into_iter().collect()
//     }
//
//     fn traverse_ref<'a>(&self, source: &'a T) -> Vec<&'a Self::To> {
//         self.pm_ref(source).into_iter().collect()
//     }
//
//     fn traverse_mut<'a>(&self, source: &'a mut T) -> Vec<&'a mut Self::To> {
//         self.pm_mut(source).into_iter().collect()
//     }
// }
//
// impl<Ls: Lens<T>, T> Prism<T> for Ls {
//     type To = Ls::To;
//
//     fn pm(&self, source: T) -> Option<Self::To> {
//         Some(self.view(source))
//     }
//
//     fn pm_ref<'a>(&self, source: &'a T) -> Option<&'a Self::To> {
//         Some(self.view_ref(source))
//     }
//
//     fn pm_mut<'a>(&self, source: &'a mut T) -> Option<&'a mut Self::To> {
//         Some(self.view_mut(source))
//     }
// }

#[derive(Copy, Clone)]
pub struct __;

#[derive(Copy, Clone)]
pub struct _Ok<Optic>(pub Optic);
#[derive(Copy, Clone)]
pub struct _Err<Optic>(pub Optic);

#[derive(Copy, Clone)]
pub struct _Some<Optic>(pub Optic);
#[derive(Copy, Clone)]
pub struct _None;

#[derive(Copy, Clone)]
pub struct _0<Optic>(pub Optic);
#[derive(Copy, Clone)]
pub struct _1<Optic>(pub Optic);
#[derive(Copy, Clone)]
pub struct _2<Optic>(pub Optic);

#[derive(Copy, Clone)]
pub struct Mapped<Optic>(pub Optic);

/***********************************************************
* impl for __
************************************************************/
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

/***********************************************************
* impl for Result
************************************************************/
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

/***********************************************************
* impl for Option
************************************************************/
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

/***********************************************************
* impl for tuple
************************************************************/
/************************** (A,) ***************************/
impl<Tr, A> Traversal<(A,)> for _0<Tr>
where
    Tr: Traversal<A>,
{
    type To = Tr::To;

    fn traverse(&self, source: (A,)) -> Vec<Self::To> {
        once(source.0).flat_map(|t| self.0.traverse(t)).collect()
    }

    fn traverse_ref<'a>(&self, source: &'a (A,)) -> Vec<&'a Self::To> {
        once(&source.0)
            .flat_map(|t| self.0.traverse_ref(t))
            .collect()
    }

    fn traverse_mut<'a>(&self, source: &'a mut (A,)) -> Vec<&'a mut Self::To> {
        once(&mut source.0)
            .flat_map(|t| self.0.traverse_mut(t))
            .collect()
    }
}

impl<Pm, A> Prism<(A,)> for _0<Pm>
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

impl<Ls, A> Lens<(A,)> for _0<Ls>
where
    Ls: Lens<A>,
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

/************************** (A, B) ***************************/
impl<Tr, A, B> Traversal<(A, B)> for _0<Tr>
where
    Tr: Traversal<A>,
{
    type To = Tr::To;

    fn traverse(&self, source: (A, B)) -> Vec<Self::To> {
        once(source.0).flat_map(|t| self.0.traverse(t)).collect()
    }

    fn traverse_ref<'a>(&self, source: &'a (A, B)) -> Vec<&'a Self::To> {
        once(&source.0)
            .flat_map(|t| self.0.traverse_ref(t))
            .collect()
    }

    fn traverse_mut<'a>(&self, source: &'a mut (A, B)) -> Vec<&'a mut Self::To> {
        once(&mut source.0)
            .flat_map(|t| self.0.traverse_mut(t))
            .collect()
    }
}

impl<Pm, A, B> Prism<(A, B)> for _0<Pm>
where
    Pm: Prism<A>,
{
    type To = Pm::To;

    fn pm(&self, source: (A, B)) -> Option<Self::To> {
        self.0.pm(source.0)
    }

    fn pm_ref<'a>(&self, source: &'a (A, B)) -> Option<&'a Self::To> {
        self.0.pm_ref(&source.0)
    }

    fn pm_mut<'a>(&self, source: &'a mut (A, B)) -> Option<&'a mut Self::To> {
        self.0.pm_mut(&mut source.0)
    }
}

impl<Ls, A, B> Lens<(A, B)> for _0<Ls>
where
    Ls: Lens<A>,
{
    type To = Ls::To;

    fn view(&self, source: (A, B)) -> Self::To {
        self.0.view(source.0)
    }

    fn view_ref<'a>(&self, source: &'a (A, B)) -> &'a Self::To {
        self.0.view_ref(&source.0)
    }

    fn view_mut<'a>(&self, source: &'a mut (A, B)) -> &'a mut Self::To {
        self.0.view_mut(&mut source.0)
    }
}

impl<Tr, A, B> Traversal<(A, B)> for _1<Tr>
where
    Tr: Traversal<B>,
{
    type To = Tr::To;

    fn traverse(&self, source: (A, B)) -> Vec<Self::To> {
        once(source.1).flat_map(|t| self.0.traverse(t)).collect()
    }

    fn traverse_ref<'a>(&self, source: &'a (A, B)) -> Vec<&'a Self::To> {
        once(&source.1)
            .flat_map(|t| self.0.traverse_ref(t))
            .collect()
    }

    fn traverse_mut<'a>(&self, source: &'a mut (A, B)) -> Vec<&'a mut Self::To> {
        once(&mut source.1)
            .flat_map(|t| self.0.traverse_mut(t))
            .collect()
    }
}

impl<Pm, A, B> Prism<(A, B)> for _1<Pm>
where
    Pm: Prism<B>,
{
    type To = Pm::To;

    fn pm(&self, source: (A, B)) -> Option<Self::To> {
        self.0.pm(source.1)
    }

    fn pm_ref<'a>(&self, source: &'a (A, B)) -> Option<&'a Self::To> {
        self.0.pm_ref(&source.1)
    }

    fn pm_mut<'a>(&self, source: &'a mut (A, B)) -> Option<&'a mut Self::To> {
        self.0.pm_mut(&mut source.1)
    }
}

impl<Ls, A, B> Lens<(A, B)> for _1<Ls>
where
    Ls: Lens<B>,
{
    type To = Ls::To;

    fn view(&self, source: (A, B)) -> Self::To {
        self.0.view(source.1)
    }

    fn view_ref<'a>(&self, source: &'a (A, B)) -> &'a Self::To {
        self.0.view_ref(&source.1)
    }

    fn view_mut<'a>(&self, source: &'a mut (A, B)) -> &'a mut Self::To {
        self.0.view_mut(&mut source.1)
    }
}

/************************** (A, B, C) ***************************/
impl<Tr, A, B, C> Traversal<(A, B, C)> for _0<Tr>
where
    Tr: Traversal<A>,
{
    type To = Tr::To;

    fn traverse(&self, source: (A, B, C)) -> Vec<Self::To> {
        once(source.0).flat_map(|t| self.0.traverse(t)).collect()
    }

    fn traverse_ref<'a>(&self, source: &'a (A, B, C)) -> Vec<&'a Self::To> {
        once(&source.0)
            .flat_map(|t| self.0.traverse_ref(t))
            .collect()
    }

    fn traverse_mut<'a>(&self, source: &'a mut (A, B, C)) -> Vec<&'a mut Self::To> {
        once(&mut source.0)
            .flat_map(|t| self.0.traverse_mut(t))
            .collect()
    }
}

impl<Pm, A, B, C> Prism<(A, B, C)> for _0<Pm>
where
    Pm: Prism<A>,
{
    type To = Pm::To;

    fn pm(&self, source: (A, B, C)) -> Option<Self::To> {
        self.0.pm(source.0)
    }

    fn pm_ref<'a>(&self, source: &'a (A, B, C)) -> Option<&'a Self::To> {
        self.0.pm_ref(&source.0)
    }

    fn pm_mut<'a>(&self, source: &'a mut (A, B, C)) -> Option<&'a mut Self::To> {
        self.0.pm_mut(&mut source.0)
    }
}

impl<Ls, A, B, C> Lens<(A, B, C)> for _0<Ls>
where
    Ls: Lens<A>,
{
    type To = Ls::To;

    fn view(&self, source: (A, B, C)) -> Self::To {
        self.0.view(source.0)
    }

    fn view_ref<'a>(&self, source: &'a (A, B, C)) -> &'a Self::To {
        self.0.view_ref(&source.0)
    }

    fn view_mut<'a>(&self, source: &'a mut (A, B, C)) -> &'a mut Self::To {
        self.0.view_mut(&mut source.0)
    }
}

impl<Tr, A, B, C> Traversal<(A, B, C)> for _1<Tr>
where
    Tr: Traversal<B>,
{
    type To = Tr::To;

    fn traverse(&self, source: (A, B, C)) -> Vec<Self::To> {
        once(source.1).flat_map(|t| self.0.traverse(t)).collect()
    }

    fn traverse_ref<'a>(&self, source: &'a (A, B, C)) -> Vec<&'a Self::To> {
        once(&source.1)
            .flat_map(|t| self.0.traverse_ref(t))
            .collect()
    }

    fn traverse_mut<'a>(&self, source: &'a mut (A, B, C)) -> Vec<&'a mut Self::To> {
        once(&mut source.1)
            .flat_map(|t| self.0.traverse_mut(t))
            .collect()
    }
}

impl<Pm, A, B, C> Prism<(A, B, C)> for _1<Pm>
where
    Pm: Prism<B>,
{
    type To = Pm::To;

    fn pm(&self, source: (A, B, C)) -> Option<Self::To> {
        self.0.pm(source.1)
    }

    fn pm_ref<'a>(&self, source: &'a (A, B, C)) -> Option<&'a Self::To> {
        self.0.pm_ref(&source.1)
    }

    fn pm_mut<'a>(&self, source: &'a mut (A, B, C)) -> Option<&'a mut Self::To> {
        self.0.pm_mut(&mut source.1)
    }
}

impl<Ls, A, B, C> Lens<(A, B, C)> for _1<Ls>
where
    Ls: Lens<B>,
{
    type To = Ls::To;

    fn view(&self, source: (A, B, C)) -> Self::To {
        self.0.view(source.1)
    }

    fn view_ref<'a>(&self, source: &'a (A, B, C)) -> &'a Self::To {
        self.0.view_ref(&source.1)
    }

    fn view_mut<'a>(&self, source: &'a mut (A, B, C)) -> &'a mut Self::To {
        self.0.view_mut(&mut source.1)
    }
}

impl<Tr, A, B, C> Traversal<(A, B, C)> for _2<Tr>
where
    Tr: Traversal<C>,
{
    type To = Tr::To;

    fn traverse(&self, source: (A, B, C)) -> Vec<Self::To> {
        once(source.2).flat_map(|t| self.0.traverse(t)).collect()
    }

    fn traverse_ref<'a>(&self, source: &'a (A, B, C)) -> Vec<&'a Self::To> {
        once(&source.2)
            .flat_map(|t| self.0.traverse_ref(t))
            .collect()
    }

    fn traverse_mut<'a>(&self, source: &'a mut (A, B, C)) -> Vec<&'a mut Self::To> {
        once(&mut source.2)
            .flat_map(|t| self.0.traverse_mut(t))
            .collect()
    }
}

impl<Pm, A, B, C> Prism<(A, B, C)> for _2<Pm>
where
    Pm: Prism<C>,
{
    type To = Pm::To;

    fn pm(&self, source: (A, B, C)) -> Option<Self::To> {
        self.0.pm(source.2)
    }

    fn pm_ref<'a>(&self, source: &'a (A, B, C)) -> Option<&'a Self::To> {
        self.0.pm_ref(&source.2)
    }

    fn pm_mut<'a>(&self, source: &'a mut (A, B, C)) -> Option<&'a mut Self::To> {
        self.0.pm_mut(&mut source.2)
    }
}

impl<Ls, A, B, C> Lens<(A, B, C)> for _2<Ls>
where
    Ls: Lens<C>,
{
    type To = Ls::To;

    fn view(&self, source: (A, B, C)) -> Self::To {
        self.0.view(source.2)
    }

    fn view_ref<'a>(&self, source: &'a (A, B, C)) -> &'a Self::To {
        self.0.view_ref(&source.2)
    }

    fn view_mut<'a>(&self, source: &'a mut (A, B, C)) -> &'a mut Self::To {
        self.0.view_mut(&mut source.2)
    }
}

/***********************************************************
* impl for iter
************************************************************/
impl<Tr, T> Traversal<Vec<T>> for Mapped<Tr>
where
    Tr: Traversal<T>,
{
    type To = Tr::To;

    fn traverse(&self, source: Vec<T>) -> Vec<Self::To> {
        source
            .into_iter()
            .flat_map(|t| self.0.traverse(t))
            .collect()
    }

    fn traverse_ref<'a>(&self, source: &'a Vec<T>) -> Vec<&'a Self::To> {
        source.iter().flat_map(|t| self.0.traverse_ref(t)).collect()
    }

    fn traverse_mut<'a>(&self, source: &'a mut Vec<T>) -> Vec<&'a mut Self::To> {
        source
            .iter_mut()
            .flat_map(|t| self.0.traverse_mut(t))
            .collect()
    }
}

//////////////////////////////

fn test() -> Option<()> {
    let mut nested: Result<Result<_, ()>, ()> = optics!(_Ok._Ok).review((1, 2));
    *optics!(_Ok._Ok._0).pm_mut(&mut nested)? += 1;
    assert_eq!(optics!(_Ok._Ok._0).pm(nested)?, 2);

    let mut x = (1, (2, (3, 4)));
    *optics!(_1._1._1).view_mut(&mut x) *= 2;
    assert_eq!(optics!(_1._1._1).view(x), 8);

    let mut x: (_, Result<_, ()>) = (1, Ok((2, 3)));
    *optics!(_1._Ok._1).pm_mut(&mut x)? *= 2;
    assert_eq!(optics!(_1._Ok._1).pm(x)?, 6);

    let mut x = (1, vec![Some((2, 3)), None]);
    optics!(_1.Mapped._Some._0)
        .traverse_mut(&mut x)
        .into_iter()
        .for_each(|i| *i += 1);
    assert_eq!(optics!(_1.Mapped._Some._0).traverse(x), vec![3]);

    Some(())
}

fn main() {
    test();
}
