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
impl<const N: usize, T> MyStruct<T> {
    async fn make_multiple_13478430068544323400<A>(another: Wrapper<A>) -> [(T, A); N] {
        another.1
    }
    fn make_multiple_13478430068544323400_boxed<'a, A>(
        another: Wrapper<A>,
    ) -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<Output = [(T, A); N]> + ::core::marker::Send + 'a,
        >,
    > {
        ::std::boxed::Box::pin(Self::make_multiple_13478430068544323400(another))
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
        Self::make_multiple_13478430068544323400_boxed::<A>(another)
    }
}
fn main() {}
