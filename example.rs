use lens_rs::*;
use Either::*;

#[derive(Copy, Clone, Debug, Review, Prism)]
enum Either<L, R> {
    #[optic]
    Left(L),
    #[optic]
    Right(R),
}

#[derive(Copy, Clone, Debug, Lens)]
struct Tuple<A, B>(#[optic] A, #[optic] B);

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


// T may have i32
fn may_have_i32<T, Pm: Prism<T, To=i32>>(t: &mut T, pm: Pm) {
    pm.pm_mut(t).map(|x| *x+=1);
}


fn main() {
    ////////////////////////////////////////////
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
    optics!(_1.Left._1).pm_mut(&mut x).map(|x| *x *= 2);
    optics!(_1.Right).pm_mut(&mut x).map(|x: &mut i32| *x *= 2);
    *optics!(_0).view_mut(&mut x) += 1;
    optics!(_1.Left._0.Mapped.Some.a)
        .traverse_mut(&mut x)
        .into_iter()
        .for_each(|s| *s = s.to_uppercase());

    println!("{:?}", x);

    ////////////////////////////////////////////////////////
    may_have_i32(&mut x, optics!(_0));
    may_have_i32(&mut x, optics!(_1.Left._1));
    println!("{:?}", x);

    ///////////////////////////////////////////////////////
    fn with_field<T>(t: T) -> String
        where
            field![a]: Lens<T, To=String>
    {
        optics!(a).view(t)
    }

    let foo = Foo {
        a: "this is Foo".to_string(),
        b: ()
    };
    let bar = Bar {
        a: "this is Bar".to_string(),
        c: 0
    };

    println!("{}", with_field(foo));
    println!("{}", with_field(bar));

    ///////////////////////////////////////////////////////

    let foo = Foo {
        a: &("foo0", Box::new("foo1")),
        b: ()
    };
    let s = optics!(a._ref._1._box).view_ref(&foo);
    println!("{}", s);
}
