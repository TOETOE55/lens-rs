macro_rules! optics {
    () => { __ };
    ($optic:ident) => { $optic(__) };
    ($optic:ident . $($optics:tt)*) => {
        $optic(optics!($($optics)*))
    }
}

trait Review<T> {
    type From;
    fn review(&self, from: Self::From) -> T;
}

trait Traversal<T> {
    type To;
    fn traverse(&self, source: T) -> Vec<Self::To>;
    fn traverse_ref<'a>(&self, source: &'a T) -> Vec<&'a Self::To>;
    fn traverse_mut<'a>(&self, source: &'a mut T) -> Vec<&'a mut Self::To>;
}

trait Prism<T> {
    type To;
    fn pm(&self, source: T) -> Option<Self::To>;
    fn pm_ref<'a>(&self, source: &'a T) -> Option<&'a Self::To>;
    fn pm_mut<'a>(&self, source: &'a mut T) -> Option<&'a mut Self::To>;
}

trait Lens<T> {
    type To;
    fn view(&self, source: T) -> Self::To;
    fn view_ref<'a>(&self, source: &'a T) -> &'a Self::To;
    fn view_mut<'a>(&self, source: &'a mut T) -> &'a mut Self::To;
}

impl<Pm: Prism<T>, T> Traversal<T> for Pm {
    type To = Pm::To;

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

impl<Ls: Lens<T>, T> Prism<T> for Ls {
    type To = Ls::To;

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

#[derive(Copy, Clone)]
struct __;

#[derive(Copy, Clone)]
struct _Ok<Optic>(pub Optic);
#[derive(Copy, Clone)]
struct _Err<Optic>(pub Optic);

#[derive(Copy, Clone)]
struct _Some<Optic>(pub Optic);
#[derive(Copy, Clone)]
struct _None;

#[derive(Copy, Clone)]
struct _0<Optic>(pub Optic);
#[derive(Copy, Clone)]
struct _1<Optic>(pub Optic);
#[derive(Copy, Clone)]
struct _2<Optic>(pub Optic);

#[derive(Copy, Clone)]
struct Mapped<Optic>(pub Optic);

/***********************************************************
* impl for __
************************************************************/
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

impl<T> Review<T> for __ {
    type From = T;

    fn review(&self, from: Self::From) -> T {
        from
    }
}

/***********************************************************
* impl for Result
************************************************************/
impl<Pm, T, E> Prism<Result<T, E>> for _Ok<Pm>
where
    Pm: Prism<T>,
{
    type To = Pm::To;

    fn pm(&self, source: Result<T, E>) -> Option<Self::To> {
        self.0.pm(source.ok()?)
    }

    fn pm_ref<'a>(&self, source: &'a Result<T, E>) -> Option<&'a Self::To> {
        self.0.pm_ref(source.as_ref().ok()?)
    }

    fn pm_mut<'a>(&self, source: &'a mut Result<T, E>) -> Option<&'a mut Self::To> {
        self.0.pm_mut(source.as_mut().ok()?)
    }
}

impl<Rv, T, E> Review<Result<T, E>> for _Ok<Rv>
where
    Rv: Review<T>,
{
    type From = Rv::From;

    fn review(&self, from: Self::From) -> Result<T, E> {
        Ok(self.0.review(from))
    }
}

impl<Pm, T, E> Prism<Result<T, E>> for _Err<Pm>
where
    Pm: Prism<E>,
{
    type To = Pm::To;

    fn pm(&self, source: Result<T, E>) -> Option<Self::To> {
        self.0.pm(source.err()?)
    }

    fn pm_ref<'a>(&self, source: &'a Result<T, E>) -> Option<&'a Self::To> {
        self.0.pm_ref(source.as_ref().err()?)
    }

    fn pm_mut<'a>(&self, source: &'a mut Result<T, E>) -> Option<&'a mut Self::To> {
        self.0.pm_mut(source.as_mut().err()?)
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

/***********************************************************
* impl for Option
************************************************************/
impl<Pm, T> Prism<Option<T>> for _Some<Pm>
where
    Pm: Prism<T>,
{
    type To = Pm::To;

    fn pm(&self, source: Option<T>) -> Option<Self::To> {
        self.0.pm(source?)
    }

    fn pm_ref<'a>(&self, source: &'a Option<T>) -> Option<&'a Self::To> {
        self.0.pm_ref(source.as_ref()?)
    }

    fn pm_mut<'a>(&self, source: &'a mut Option<T>) -> Option<&'a mut Self::To> {
        self.0.pm_mut(source.as_mut()?)
    }
}

impl<Rv, T> Review<Option<T>> for _Some<Rv>
where
    Rv: Review<T>,
{
    type From = Rv::From;

    fn review(&self, from: Self::From) -> Option<T> {
        Some(self.0.review(from))
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

    None
}

fn main() {
    test();
}
