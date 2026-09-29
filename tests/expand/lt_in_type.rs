use core::marker::PhantomData;

struct WithLt<'a>(PhantomData<&'a ()>);

#[async_trait::async_trait]
trait AnotherOne {
    async fn another_one(a: &WithLt<'_>) -> WithLt<'_>;
}

#[ul_async_trait::async_trait]
impl<'a> AnotherOne for WithLt<'a> {
    async fn another_one(_a: &WithLt<'_>) -> WithLt<'_> {
        WithLt(PhantomData)
    }
}

fn main() {}
