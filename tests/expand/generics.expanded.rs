struct Wrapper<A>(core::marker::PhantomData<A>, !);
trait WithGenerics<const N: usize, T> {
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn make_multiple<'async_trait, A>(
        another: Wrapper<A>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = [(T, A); N],
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        A: 'async_trait;
}
struct MyStruct<T>(core::marker::PhantomData<T>);
impl<const N: usize, T> WithGenerics<N, T> for MyStruct<T> {
    fn make_multiple<'async_trait, A>(
        another: Wrapper<A>,
    ) -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<
                Output = [(T, A); N],
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        A: 'async_trait,
    {
        fn inner<'a, const N: usize, T, A>(
            another: Wrapper<A>,
        ) -> ::core::pin::Pin<
            ::std::boxed::Box<
                dyn ::core::future::Future<
                    Output = [(T, A); N],
                > + ::core::marker::Send + 'a,
            >,
        >
        where
            MyStruct<T>: WithGenerics<N, T>,
        {
            ::std::boxed::Box::pin(
                #[allow(clippy::async_yields_async, clippy::diverging_sub_expression)]
                async move {
                    if let ::core::option::Option::Some(ret) = ::core::option::Option::None::<
                        [(T, A); N],
                    > {
                        return ret;
                    }
                    let ret: [(T, A); N] = { another.1 };
                    #[allow(unreachable_code)] ret
                },
            )
        }
        inner::<N, T, A>(another)
    }
}
fn main() {}
