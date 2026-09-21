async fn other_future() {}
trait Write {
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn write_str<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        s: &'life1 str,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = String,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;
}
struct MyStruct {
    s: String,
}
trait __async_bodies_of_Write_14302471173908516815 {
    async fn write_str_14302471173908516815<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        s: &'life1 str,
    ) -> String
    where
        Self: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait;
}
impl __async_bodies_of_Write_14302471173908516815 for MyStruct {
    async fn write_str_14302471173908516815<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        s: &'life1 str,
    ) -> String
    where
        Self: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
    {
        self.s.push_str(s);
        other_future().await;
        self.s.clone()
    }
}
impl Write for MyStruct {
    fn write_str<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        s: &'life1 str,
    ) -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<
                Output = String,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        Self: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
    {
        ::std::boxed::Box::pin(
            <Self as __async_bodies_of_Write_14302471173908516815>::write_str_14302471173908516815::<
                'life0,
                'life1,
                'async_trait,
            >(self, s),
        )
    }
}
struct MyStructCorrect {
    s: String,
}
impl Write for MyStructCorrect {
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
    fn write_str<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        s: &'life1 str,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = String,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                String,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let mut __self = self;
            let __ret: String = {
                __self.s.push_str(s);
                other_future().await;
                __self.s.clone()
            };
            #[allow(unreachable_code)] __ret
        })
    }
}
fn main() {}
