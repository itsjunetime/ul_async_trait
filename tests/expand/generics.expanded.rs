struct Wrapper<A>(core::marker::PhantomData<A>);
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
trait __async_bodies_of_WithGenerics_13478430068544323400<N, T> {
    async fn make_multiple_13478430068544323400<'async_trait, A>(
        another: Wrapper<A>,
    ) -> [(T, A); N]
    where
        A: 'async_trait;
}
impl<const N: usize, T> __async_bodies_of_WithGenerics_13478430068544323400<N, T>
for MyStruct<T> {
    async fn make_multiple_13478430068544323400<'async_trait, A>(
        another: Wrapper<A>,
    ) -> [(T, A); N]
    where
        A: 'async_trait,
    {
        ::core::panicking::panic("not yet implemented")
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
        ::std::boxed::Box::pin(
            <Self as __async_bodies_of_WithGenerics_13478430068544323400<
                N,
                T,
            >>::make_multiple_13478430068544323400::<'async_trait, A>(another),
        )
    }
}
fn main() {}
