pub mod optics;
pub mod traits;

pub use traits::{
    Lens, LensMut, LensRef, Prism, PrismMut, PrismRef, Review, Traversal, TraversalMut,
    TraversalRef,
};

pub use optics::{
    _arc, _both, _box, _ix, _mapped, _mut, _rc, _ref, _0, _1, _2, _3, _4, _5, _6, __,
};

pub use lens_rs_derive::{Lens, Prism, Review};

#[macro_export]
macro_rules! optics {
    () => { lens_rs::optics::__ };
    ($optic:ident) => { lens_rs::optics::$optic(lens_rs::optics::__) };
    ([$ix:expr]) => {
        lens_rs::optics::_ix(lens_rs::optics::__, $ix)
    };
    ($optic:ident . $($optics:tt)*) => {
        lens_rs::optics::$optic(optics!($($optics)*))
    };
    ([$ix:expr] . $($optics:tt)*) => {
        lens_rs::optics::_ix(optics!($($optics)*), $ix)
    }
}

#[macro_export]
macro_rules! field {
    [] => { lens_rs::optics::__ };
    [$optic:ident] => { lens_rs::optics::$optic<lens_rs::optics::__> };
    [[$ix:ty]] => {
        lens_rs::optics::_ix<$ix, lens_rs::optics::__>
    };
    [$optic:ident . $($optics:tt)*] => {
        lens_rs::optics::$optic<field![$($optics)*]>
    };
    [[$ix:ty] . $($optics:tt)*] => {
        lens_rs::optics::_ix<$ix, field![$($optics)*]>
    }
}
