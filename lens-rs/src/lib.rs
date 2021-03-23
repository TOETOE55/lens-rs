pub mod optics;
pub mod traits;

pub use traits::{
    Lens, LensMut, LensRef, Prism, PrismMut, PrismRef, Review, Traversal, TraversalMut,
    TraversalRef,
};

pub use optics::{
    _both, _mapped, _arc, _box, Err, _mut, _rc, _ref, _0, _1, _2, _3, _4, _5, _6,
    __,
};

pub use lens_rs_derive::{Lens, Prism, Review};

#[macro_export]
macro_rules! optics {
    () => { lens_rs::optics::__ };
    ($optic:ident) => { lens_rs::optics::$optic(lens_rs::optics::__) };
    ($optic:ident . $($optics:tt)*) => {
        lens_rs::optics::$optic(optics!($($optics)*))
    }
}

#[macro_export]
macro_rules! field {
    [] => { lens_rs::optics::__ };
    [$optic:ident] => { lens_rs::optics::$optic<lens_rs::optics::__> };
    [$optic:ident . $($optics:tt)*] => {
        lens_rs::optics::$optic<field![$($optics)*]>
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test() -> Result<(), ()> {
        let mut nested: Result<Result<_, ()>, ()> = optics!(Ok.Ok).review((1, 2));
        *optics!(Ok.Ok._0).pm_mut(&mut nested).ok_or(())? += 1;
        assert_eq!(optics!(Ok.Ok._0).pm(nested).ok_or(())?, 2);

        let mut x = (1, (2, (3, 4)));
        *optics!(_1._1._1).view_mut(&mut x) *= 2;
        assert_eq!(optics!(_1._1._1).view(x), 8);

        let mut x: (_, Result<_, ()>) = (1, Ok((2, 3)));
        *optics!(_1.Ok._1).pm_mut(&mut x).ok_or(())? *= 2;
        assert_eq!(optics!(_1.Ok._1).pm(x).ok_or(())?, 6);

        let mut x = (1, vec![Some((2, 3)), None]);
        optics!(_1.Mapped._Some._0)
            .traverse_mut(&mut x)
            .into_iter()
            .for_each(|i| *i += 1);
        assert_eq!(optics!(_1.Mapped._Some._0).traverse(x), vec![3]);

        Ok(())
    }
}
