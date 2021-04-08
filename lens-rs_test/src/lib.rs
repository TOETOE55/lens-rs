#[cfg(test)]
mod tests {
    use lens_rs::*;
    use Either::*;
    use Nat::*;

    // derive enum
    #[derive(Copy, Clone, Debug, Review, Prism)]
    enum Either<L, R> {
        #[optic]
        Left(L),
        #[optic]
        Right(R),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Review, Prism)]
    enum Nat {
        #[optic]
        S(Box<Nat>),
        #[optic]
        Z,
    }

    #[derive(Clone, Debug, Prism, Review)]
    enum IsSome<T> {
        #[optic]
        Some(T),
    }

    #[derive(Clone, Debug, Prism, Review)]
    enum Unit {
        #[optic]
        Unit,
        #[optic]
        Unnamed(),
        #[optic]
        Named {},
    }

    // derive struct
    #[derive(Copy, Clone, Debug, Lens)]
    struct Foo<A, B> {
        #[optic]
        a: A,
        #[optic]
        b: B,
    }

    #[derive(Clone, Debug, Lens)]
    struct Bar {
        #[optic]
        a: String,
        #[optic]
        c: i32,
    }

    #[derive(Debug, Lens)]
    struct Shit<'a> {
        #[optic(ref)]
        a: &'a str,
        #[optic(mut)]
        b: &'a mut Vec<i32>,
    }

    // derive tuple
    #[derive(Copy, Clone, Debug, Lens)]
    struct Tuple<A, B>(#[optic] A, #[optic] B);

    #[derive(Copy, Clone, Debug, Lens)]
    struct Empty;

    // T may have i32
    fn may_have_i32<T: PrismRef<Pm, i32>, Pm>(t: &T, pm: Pm) -> Option<i32> {
        t.preview_ref(pm).map(|x| *x)
    }

    fn with_field_a<T>(t: &T) -> &str
    where
        T: LensRef<Optics![a], String>,
    {
        t.view_ref(optics!(a))
    }

    fn test_nested() -> Option<()> {
        let mut x: (i32, Either<Tuple<Vec<Option<Foo<String, i32>>>, i32>, i32>) = (
            1,
            Left(Tuple(
                vec![
                    Some(Foo {
                        a: "a".to_string(),
                        b: 2,
                    }),
                    None,
                    Some(Foo {
                        a: 'c'.to_string(),
                        b: 3,
                    }),
                ],
                4,
            )),
        );

        x.preview_mut(optics!(_1.Left._1)).map(|x| *x *= 2);
        assert_eq!(x.preview_ref(optics!(_1.Left._1)), Some(&8));

        x.preview_mut(optics!(_1.Right)).map(|x: &mut i32| *x *= 2);
        assert_eq!(x.preview_ref(optics!(_1.Right)), None);

        *x.view_mut(optics!(_0)) += 1;
        assert_eq!(x.0, 2);

        x.traverse_mut(optics!(_1.Left._0._mapped.Some.a))
            .into_iter()
            .for_each(|s| *s = s.to_uppercase());
        assert_eq!(
            x.traverse(optics!(_1.Left._0._mapped.Some.a)),
            vec!["A".to_string(), "C".to_string()]
        );

        Some(())
    }

    fn test_row() {
        let x: (i32, Result<_, (i32, i32)>) = (0, Ok((1, 3)));
        assert_eq!(may_have_i32(&x, optics!(_0)), Some(x.0));
        assert_eq!(may_have_i32(&x, optics!(_1.Ok._1)), Some(3));
        assert_eq!(may_have_i32(&x, optics!(_1.Err._0)), None);

        let foo = Foo {
            a: "this is Foo".to_string(),
            b: (),
        };
        let bar = Bar {
            a: "this is Bar".to_string(),
            c: 0,
        };

        assert_eq!(with_field_a(&foo), "this is Foo");
        assert_eq!(with_field_a(&bar), "this is Bar");
    }

    fn test_ptr() {
        let one: Nat = Review::review(optics!(S.lens_rs::optics::_box.Z), ());
        let mut two: Nat = Review::review(optics!(S._box.S._box.Z), ());
        let three: Nat = Review::review(optics!(S._box.S._box.Z), ());
        assert_eq!(one, S(Box::new(Z)));
        two.preview_mut(optics!(S._box)).map(move |x| *x = one); // 2+1
        assert_eq!(two, three);

        let foo = Foo {
            a: &("foo0", Box::new("foo1")),
            b: (),
        };
        let foo0 = foo.view_ref(optics!(a._ref._0));
        let foo1 = foo.view_ref(optics!(a._ref._1._box));
        assert_eq!(*foo0, "foo0");
        assert_eq!(*foo1, "foo1");
    }

    fn test_index() {
        let mut x = (1, vec![2, 3]);
        *x.view_mut(optics!(_1.[0])) *= 2;

        assert_eq!(x.1[0], 4);
    }

    fn test_absent() {
        fn may_has_c<T>(t: T) -> Option<i32>
        where
            T: Prism<Optics![c], i32>,
        {
            t.preview(optics!(c))
        }

        let foo = Foo {
            a: "this is Foo".to_string(),
            b: (),
        };
        let bar = Bar {
            a: "this is Bar".to_string(),
            c: 0,
        };
        let left: Either<i32, i32> = Left(0);

        assert_eq!(may_has_c(foo), None);
        assert_eq!(may_has_c(bar), Some(0));
        assert_eq!(may_has_c(left), None);
        assert_eq!(may_has_c((1, 2, 3)), None);
    }

    #[test]
    fn it_works() {
        test_nested();
        test_row();
        test_ptr();
        test_index();
        test_absent();
    }
}
