trait MyTrait {
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn my_fn<'life0, 'async_trait>(
        &'life0 self,
        num: usize,
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
struct MyStruct;
trait __async_bodies_of_MyTrait_14302471173908516815 {
    async fn my_fn_14302471173908516815(&self, num: usize) -> usize;
}
impl __async_bodies_of_MyTrait_14302471173908516815 for MyStruct {
    async fn my_fn_14302471173908516815(&self, num: usize) -> usize {
        num + 1
    }
}
impl MyTrait for MyStruct {
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
    fn my_fn<'life0, 'async_trait>(
        &'life0 self,
        num: usize,
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
            let num = num;
            let __ret: usize = {
                <Self as __async_bodies_of_MyTrait_14302471173908516815>::my_fn_14302471173908516815(
                        __self,
                        num,
                    )
                    .await
            };
            #[allow(unreachable_code)] __ret
        })
    }
}
