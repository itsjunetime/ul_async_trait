#![expect(dead_code, renamed_and_removed_lints)]
enum Whatever {
    Thing(usize),
}
enum OtherWhatever {
    Thing(usize),
}
trait AddOne {
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn add_one<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = usize,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
}
impl AddOne for Whatever {
    fn add_one<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<
                Output = usize,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        Self: 'async_trait,
        'life0: 'async_trait,
    {
        #[allow(clippy::type_complexity)]
        fn inner<'a>(
            slf: &'a Whatever,
        ) -> ::core::pin::Pin<
            ::std::boxed::Box<
                dyn ::core::future::Future<Output = usize> + ::core::marker::Send + 'a,
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
                    let ret: usize = {
                        match slf {
                            Whatever::Thing(o) => o + 1,
                        }
                    };
                    #[allow(unreachable_code)] ret
                },
            )
        }
        inner(self)
    }
}
impl AddOne for OtherWhatever {
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
    fn add_one<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = usize,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                usize,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let __self = self;
            let __ret: usize = {
                match __self {
                    Self::Thing(o) => o + 1,
                }
            };
            #[allow(unreachable_code)] __ret
        })
    }
}
fn main() {}
