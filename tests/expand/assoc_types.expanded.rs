trait UsesAssocTypes {
    type Assoc;
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn takes_ty<'async_trait>(
        a: Self::Assoc,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        Self: 'async_trait;
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn gives_ty<'async_trait>() -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Self::Assoc,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        Self: 'async_trait;
}
struct AssocIsStaticStr;
impl UsesAssocTypes for AssocIsStaticStr {
    type Assoc = &'static str;
    fn takes_ty<'async_trait>(
        _a: Self::Assoc,
    ) -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait,
        >,
    > {
        fn inner<'a>(
            _a: <AssocIsStaticStr as UsesAssocTypes>::Assoc,
        ) -> ::core::pin::Pin<
            ::std::boxed::Box<
                dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'a,
            >,
        >
        where
            AssocIsStaticStr: UsesAssocTypes,
        {
            ::std::boxed::Box::pin(
                #[allow(clippy::async_yields_async, clippy::diverging_sub_expression)]
                async move {
                    if let ::core::option::Option::Some(ret) = ::core::option::Option::None::<
                        (),
                    > {
                        return ret;
                    }
                    let ret: () = {};
                    #[allow(unreachable_code)] ret
                },
            )
        }
        inner(_a)
    }
    fn gives_ty<'async_trait>() -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<
                Output = Self::Assoc,
            > + ::core::marker::Send + 'async_trait,
        >,
    > {
        fn inner<'a>() -> ::core::pin::Pin<
            ::std::boxed::Box<
                dyn ::core::future::Future<
                    Output = <AssocIsStaticStr as UsesAssocTypes>::Assoc,
                > + ::core::marker::Send + 'a,
            >,
        >
        where
            AssocIsStaticStr: UsesAssocTypes,
        {
            ::std::boxed::Box::pin(
                #[allow(clippy::async_yields_async, clippy::diverging_sub_expression)]
                async move {
                    if let ::core::option::Option::Some(ret) = ::core::option::Option::None::<
                        <AssocIsStaticStr as UsesAssocTypes>::Assoc,
                    > {
                        return ret;
                    }
                    let ret: <AssocIsStaticStr as UsesAssocTypes>::Assoc = { "" };
                    #[allow(unreachable_code)] ret
                },
            )
        }
        inner()
    }
}
struct AssocIsUsize;
impl UsesAssocTypes for AssocIsUsize {
    type Assoc = usize;
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
    fn takes_ty<'async_trait>(
        _a: Self::Assoc,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        Self: 'async_trait,
    {
        Box::pin(async move {
            let _a = _a;
            let _: () = {};
        })
    }
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
    fn gives_ty<'async_trait>() -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Self::Assoc,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                Self::Assoc,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let __ret: Self::Assoc = { 0 };
            #[allow(unreachable_code)] __ret
        })
    }
}
fn main() {}
