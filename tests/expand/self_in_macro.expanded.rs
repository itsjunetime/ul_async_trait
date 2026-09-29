#![expect(dead_code, renamed_and_removed_lints)]
trait SelfInMacro {
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn times_two<'async_trait>(
        self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = usize,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        Self: 'async_trait;
}
struct MyStruct(usize);
impl SelfInMacro for MyStruct {
    fn times_two<'async_trait>(
        self,
    ) -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<
                Output = usize,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        Self: 'async_trait,
    {
        #[allow(clippy::type_complexity)]
        fn inner<'ul_async_trait>(
            slf: MyStruct,
        ) -> ::core::pin::Pin<
            ::std::boxed::Box<
                dyn ::core::future::Future<
                    Output = usize,
                > + ::core::marker::Send + 'ul_async_trait,
            >,
        > {
            ::std::boxed::Box::pin(
                #[allow(clippy::async_yields_async, clippy::diverging_sub_expression)]
                async move {
                    let slf = slf;
                    if let ::core::option::Option::Some(ret) = ::core::option::Option::None::<
                        usize,
                    > {
                        return ret;
                    }
                    let ret: usize = { slf.0 + slf.0 };
                    #[allow(unreachable_code)] ret
                },
            )
        }
        inner(self)
    }
}
fn main() {}
