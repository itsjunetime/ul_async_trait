async fn other_future() {}

#[async_trait::async_trait]
trait Write {
    async fn write_str(&mut self, s: &str) -> String;
}

struct MyStruct {
    s: String
}

#[ul_async_trait::async_trait]
impl Write for MyStruct {
    async fn write_str(&mut self, s: &str) -> String {
        self.s.push_str(s);
        other_future().await;
        self.s.clone()
    }
}

struct MyStructCorrect {
    s: String
}

#[async_trait::async_trait]
impl Write for MyStructCorrect {
    async fn write_str(&mut self, s: &str) -> String {
        self.s.push_str(s);
        other_future().await;
        self.s.clone()
    }
}

fn main() {}
