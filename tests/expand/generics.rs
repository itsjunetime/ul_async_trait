#![expect(dead_code, renamed_and_removed_lints)]

struct Wrapper<A: Send + 'static>(core::marker::PhantomData<A>, !);

#[async_trait::async_trait]
trait WithGenerics<const N: usize, T> {
    async fn make_multiple<A: Send + 'static>(another: Wrapper<A>) -> [(T, A); N];
}

struct MyStruct<T>(core::marker::PhantomData<T>);

#[ul_async_trait::async_trait]
impl<const N: usize, T> WithGenerics<N, T> for MyStruct<T> {
    async fn make_multiple<A: Send + 'static>(another: Wrapper<A>) -> [(T, A); N] {
        another.1
    }
}

fn main() {}
