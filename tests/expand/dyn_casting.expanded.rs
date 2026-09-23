use std::{any::Any, sync::Arc};
trait ReturnsDyn {
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn make_dyn<'async_trait>() -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Result<Arc<dyn Any>, &'static str>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >;
}
struct ReturnsString;
trait __async_impl_14124350369018977111: ReturnsDyn {
    fn make_dyn_14124350369018977111<'a>() -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<
                Output = Result<Arc<dyn Any>, &'a str>,
            > + ::core::marker::Send + 'a,
        >,
    >;
}
impl __async_impl_14124350369018977111 for ReturnsString {
    fn make_dyn_14124350369018977111<'a>() -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<
                Output = Result<Arc<dyn Any>, &'a str>,
            > + ::core::marker::Send + 'a,
        >,
    > {
        ::std::boxed::Box::pin(
            #[allow(clippy::async_yields_async, clippy::diverging_sub_expression)]
            async move {
                if let ::core::option::Option::Some(ret) = ::core::option::Option::None::<
                    Result<Arc<dyn Any>, &'a str>,
                > {
                    return ret;
                }
                let ret: Result<Arc<dyn Any>, &'a str> = { Ok(Arc::new(String::new())) };
                #[allow(unreachable_code)] ret
            },
        )
    }
}
impl ReturnsDyn for ReturnsString {
    fn make_dyn<'async_trait>() -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<
                Output = Result<Arc<dyn Any>, &'static str>,
            > + ::core::marker::Send + 'async_trait,
        >,
    > {
        <Self as __async_impl_14124350369018977111>::make_dyn_14124350369018977111()
    }
}
struct ReturnsUsize;
impl ReturnsDyn for ReturnsUsize {
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
    fn make_dyn<'async_trait>() -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Result<Arc<dyn Any>, &'static str>,
            > + ::core::marker::Send + 'async_trait,
        >,
    > {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                Result<Arc<dyn Any>, &'static str>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let __ret: Result<Arc<dyn Any>, &'static str> = { Ok(Arc::new(0usize)) };
            #[allow(unreachable_code)] __ret
        })
    }
}
fn returns_dyn() -> Result<Arc<dyn Any>, &'static str> {
    Ok(Arc::new(String::new()))
}
fn main() {}
