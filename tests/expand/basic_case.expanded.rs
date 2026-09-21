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
    fn my_fn<'async_trait, 'life0>(
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
        ::std::boxed::Box::pin(
            <Self as __async_bodies_of_MyTrait_14302471173908516815>::my_fn_14302471173908516815(
                self,
                num,
            ),
        )
    }
}
fn main() {}
