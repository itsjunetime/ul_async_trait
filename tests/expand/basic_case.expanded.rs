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
trait __async_impl_15866891917791490909 {
    fn my_fn_15866891917791490909<'a>(
        &'a self,
        num: usize,
    ) -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<Output = usize> + ::core::marker::Send + 'a,
        >,
    >;
}
impl __async_impl_15866891917791490909 for MyStruct {
    fn my_fn_15866891917791490909<'a>(
        &'a self,
        num: usize,
    ) -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<Output = usize> + ::core::marker::Send + 'a,
        >,
    > {
        ::std::boxed::Box::pin(async move {
            if let ::core::option::Option::Some(ret) = ::core::option::Option::None::<
                usize,
            > {
                return ret;
            }
            let ret: usize = { num + 1 };
            #[allow(unreachable_code)] ret
        })
    }
}
impl MyTrait for MyStruct {
    fn my_fn<'life0, 'async_trait>(
        &'life0 self,
        num: usize,
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
        <Self as __async_impl_15866891917791490909>::my_fn_15866891917791490909(
            self,
            num,
        )
    }
}
struct MyStruct2;
impl MyTrait for MyStruct2 {
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
            let __ret: usize = { num + 1 };
            #[allow(unreachable_code)] __ret
        })
    }
}
fn main() {}
