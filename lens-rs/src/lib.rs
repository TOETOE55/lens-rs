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
