struct Wrapper<A>(core::marker::PhantomData<A>);

#[async_trait::async_trait]
trait WithGenerics<const N: usize, T> {
    async fn make_multiple<A>(another: Wrapper<A>) -> [(T, A); N];
}

struct MyStruct<T>(core::marker::PhantomData<T>);

#[ul_async_trait::async_trait]
impl<const N: usize, T> WithGenerics<N, T> for MyStruct<T> {
    async fn make_multiple<A>(another: Wrapper<A>) -> [(T, A); N] {
        todo!()
    }
}

fn main() {}
