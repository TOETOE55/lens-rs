use proc_macro2::Span;
use syn::visit::Visit;
use syn::ItemStruct;

/// the basic optic, behave as the identity functor
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

/// behaves as `x.0`, implemented `Lens`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _0<Optic>(pub Optic);
/// behaves as `x.1`, implemented `Lens`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct _1<Optic>(pub Optic);
/// behaves as `x.2`, implemented `Lens`
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

/// to traverse all fields of tuples(0~6), implemented `Traversal`
///
/// ```rust
/// assert_eq!((1, 2).view(optics!(_both)), vec![1, 2])
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _both<Optic>(pub Optic);

/// to traverse items of collections, implemented `Traversal`
///
/// ```rust
/// assert_eq!(vec![vec![1,2], vec![3,4]].traverse(_mapped._mapped), vec![1, 2, 3, 4])
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _mapped<Optic>(pub Optic);

/// to create `Box`, `Rc` or `Arc` pointers, implemented `Review`
///
/// ```rust
/// assert_eq!(Review::review(optics!(_box), 0), Box::new(0));
/// assert_eq!(Review::review(optics!(_box), 0), Rc::new(0));
/// assert_eq!(Review::review(optics!(_box), 0), Arc::new(0));
/// ```
///
/// or to visit the data in `Box`, implemented `Lens`
///
/// ```rust
/// assert_eq!(Box::new(0).view(optic!(_box)), 0);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _box<Optic>(pub Optic);

/// to visit the data in pointers, implemented `LensRef`
///
/// ```rust
/// assert_eq!(Box::new(0).view_ref(optic!(_ref)), &0);
/// assert_eq!(Rc::new(0).view_ref(optic!(_ref)), &0);
///
/// let x = Foo {
///     a: &(1, 2),
///     b: ()
/// };
/// assert_eq!(x.view_ref(optics!(a._ref._1)), &2)
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _ref<Optic>(pub Optic);
/// to visit the data in pointers, implemented `LensMut`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _mut<Optic>(pub Optic);

/// behave as `xs[index]`
///
/// ```rust
/// assert_eq!(vec![1,2,3].view_ref(optics!([1])), &2);
/// assert_eq!(vec![1,2,3].view_ref(optics!([1..])), &[2, 3]);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct _ix<I, Optic>(pub Optic, pub I);

include!(concat!(env!("OUT_DIR"), "/optics.rs"));

struct OpticDefinitionsCollector<'a>(&'a mut Vec<syn::Ident>);

impl<'a> Visit<'_> for OpticDefinitionsCollector<'a> {
    fn visit_item_struct(&mut self, item_struct: &ItemStruct) {
        self.0.push(item_struct.ident.clone());
    }
}

pub fn all_optics() -> Vec<syn::Ident> {
    let contents = include_str!(concat!(env!("OUT_DIR"), "/optics.rs"));
    let syntax =
        syn::parse_file(contents).expect(".rs files should contain valid Rust source code.");

    let mut optics = vec![];
    let mut collector = OpticDefinitionsCollector(&mut optics);
    collector.visit_file(&syntax);
    optics.extend(
        vec![
            "Ok", "Err", "Some", "None", "_0", "_1", "_2", "_3", "_4", "_5", "_6",
        ]
        .into_iter()
        .map(|name| syn::Ident::new(name, Span::call_site())),
    );

    optics
}
