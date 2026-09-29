#![expect(dead_code, renamed_and_removed_lints)]

#[async_trait::async_trait]
trait SelfInMacro {
    async fn times_two(self) -> usize;
}

macro_rules! slf_0 {
    ($slf:ident) => {
        $slf.0
    }
}

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    }
}

struct MyStruct(usize);

#[ul_async_trait::async_trait]
impl SelfInMacro for MyStruct {
    async fn times_two(self) -> usize {
        add!(self.0, slf_0!(self))
    }
}

fn main() {}
