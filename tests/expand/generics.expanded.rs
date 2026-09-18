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
    async fn make_multiple_13478430068544323400<A>(another: Wrapper<A>) -> [(T, A); N];
}
impl<const N: usize, T> __async_bodies_of_WithGenerics_13478430068544323400<N, T>
for MyStruct<T> {
    async fn make_multiple_13478430068544323400<A>(another: Wrapper<A>) -> [(T, A); N] {
        ::core::panicking::panic("not yet implemented")
    }
}
impl<const N: usize, T> WithGenerics<N, T> for MyStruct<T> {
    #[allow(
        elided_named_lifetimes,
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::needless_arbitrary_self_type,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
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
        A: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                [(T, A); N],
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let another = another;
            let __ret: [(T, A); N] = {
                <Self as __async_bodies_of_WithGenerics_13478430068544323400<
                    N,
                    T,
                >>::make_multiple_13478430068544323400::<A>(another)
                    .await
            };
            #[allow(unreachable_code)] __ret
        })
    }
}
