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


// pub trait LensH<T> {
//     type To;
//     type ToRef<'a>;
//     type ToMut<'a>;
//     fn view(&self, source: T) -> Self::To;
//     fn view_ref<'a>(&self, source: &'a T) -> Self::ToRef<'a>;
//     fn view_mut<'a>(&self, source: &'a mut T) -> Self::ToMut<'a>;
// }
