use core::marker::PhantomData;
struct WithLt<'a>(PhantomData<&'a ()>);
trait AnotherOne {
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn another_one<'life0, 'life1, 'async_trait>(
        a: &'life0 WithLt<'life1>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = WithLt<'_>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait;
}
impl<'a> AnotherOne for WithLt<'a> {
    fn another_one<'life0, 'life1, 'async_trait>(
        _a: &'life0 WithLt<'life1>,
    ) -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<
                Output = WithLt<'_>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
    {
        #[allow(clippy::type_complexity)]
        fn inner<'ul_async_trait>(
            _a: &'ul_async_trait WithLt<'_>,
        ) -> ::core::pin::Pin<
            ::std::boxed::Box<
                dyn ::core::future::Future<
                    Output = WithLt<'ul_async_trait>,
                > + ::core::marker::Send + 'ul_async_trait,
            >,
        > {
            ::std::boxed::Box::pin(
                #[allow(clippy::async_yields_async, clippy::diverging_sub_expression)]
                async move {
                    let _a = _a;
                    if let ::core::option::Option::Some(ret) = ::core::option::Option::None::<
                        WithLt<'_>,
                    > {
                        return ret;
                    }
                    let ret: WithLt<'_> = { WithLt(PhantomData) };
                    #[allow(unreachable_code)] ret
                },
            )
        }
        inner(_a)
    }
}
fn main() {}
