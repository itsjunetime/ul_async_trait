#[async_trait::async_trait]
trait MyTrait {
    async fn my_fn(&self, num: usize) -> usize;
}

struct MyStruct;

#[faster_async_trait::async_trait]
impl MyTrait for MyStruct {
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
