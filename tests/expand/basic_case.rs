#![expect(dead_code, renamed_and_removed_lints)]

#[async_trait::async_trait]
trait MyTrait {
    async fn my_fn(&self, num: usize) -> usize;
}

struct MyStruct;

#[ul_async_trait::async_trait]
impl MyTrait for MyStruct {
    #[expect(unused_variables)]
    async fn my_fn(&self, num: usize) -> usize {
        num + 1
    }
}

struct MyStruct2;

#[async_trait::async_trait]
impl MyTrait for MyStruct2 {
    async fn my_fn(&self, num: usize) -> usize {
        num + 1
    }
}

fn main() {}
