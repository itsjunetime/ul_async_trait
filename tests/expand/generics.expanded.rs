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
trait __async_impl_452534731895916786<const N: usize, T> {
    fn make_multiple_452534731895916786<'a, A>(
        another: Wrapper<A>,
    ) -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<Output = [(T, A); N]> + ::core::marker::Send + 'a,
        >,
    >;
}
impl<const N: usize, T> __async_impl_452534731895916786<N, T> for MyStruct<T> {
    fn make_multiple_452534731895916786<'a, A>(
        another: Wrapper<A>,
    ) -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<Output = [(T, A); N]> + ::core::marker::Send + 'a,
        >,
    > {
        ::std::boxed::Box::pin(async move {
            if let ::core::option::Option::Some(ret) = ::core::option::Option::None::<
                [(T, A); N],
            > {
                return ret;
            }
            let ret: [(T, A); N] = { another.1 };
            ret
        })
    }
}
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
        <Self as __async_impl_452534731895916786<
            N,
            T,
        >>::make_multiple_452534731895916786::<A>(another)
    }
}
fn main() {}
