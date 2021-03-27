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
        Z(()),
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

    // T may have i32
    fn may_have_i32<T, Pm: PrismRef<T, To = i32>>(t: &T, pm: Pm) -> Option<i32> {
        pm.preview_ref(t).map(|x| *x)
    }

    fn with_field<T>(t: T) -> String
    where
        Optics![a]: Lens<T, To = String>,
    {
        optics!(a).view(t)
    }

    fn test_nested() -> Option<()> {
        let mut x = (
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

        optics!(_1.Left._1).preview_mut(&mut x).map(|x| *x *= 2);
        assert_eq!(optics!(_1.Left._1).preview_ref(&x), Some(&8));

        optics!(_1.Right)
            .preview_mut(&mut x)
            .map(|x: &mut i32| *x *= 2);
        assert_eq!(optics!(_1.Right).preview_ref(&x), None);

        *optics!(_0).view_mut(&mut x) += 1;
        assert_eq!(x.0, 2);

        optics!(_1.Left._0._mapped.Some.a)
            .traverse_mut(&mut x)
            .into_iter()
            .for_each(|s| *s = s.to_uppercase());
        assert_eq!(
            optics!(_1.Left._0._mapped.Some.a).traverse(x),
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

        assert_eq!(with_field(foo), "this is Foo".to_string());
        assert_eq!(with_field(bar), "this is Bar".to_string());
    }

    fn test_ptr() {
        let one = optics!(S._box.Z).review(());
        let mut two = optics!(S._box.S._box.Z).review(());
        let three = optics!(S._box.S._box.Z).review(());
        assert_eq!(one, S(Box::new(Z(()))));
        optics!(S._box).pm_mut(&mut two).map(move |x| *x = one); // 2+1
        assert_eq!(two, three);

        let foo = Foo {
            a: &("foo0", Box::new("foo1")),
            b: (),
        };
        let foo0 = optics!(a._ref._0).view_ref(&foo);
        let foo1 = optics!(a._ref._1._box).view_ref(&foo);
        assert_eq!(*foo0, "foo0");
        assert_eq!(*foo1, "foo1");
    }

    #[test]
    fn it_works() {
        test_nested();
        test_row();
        test_ptr();

        let mut x = (1, vec![2, 3]);
        *optics!(_1.[0]).view_mut(&mut x) *= 2;

        assert_eq!(x.1[0], 4);
    }
}
