enum Whatever {
    Thing(usize)
}

enum OtherWhatever {
    Thing(usize)
}

#[async_trait::async_trait]
trait AddOne {
    async fn add_one(&self) -> usize;
}

#[ul_async_trait::async_trait]
impl AddOne for Whatever {
    async fn add_one(&self) -> usize {
        match self {
            Self::Thing(o) => o + 1
        }
    }
}

#[async_trait::async_trait]
impl AddOne for OtherWhatever {
    async fn add_one(&self) -> usize {
        match self {
            Self::Thing(o) => o + 1
        }
    }
}

fn main() {}
