use std::{any::Any, sync::Arc};

#[async_trait::async_trait]
trait ReturnsDyn {
    async fn make_dyn() -> Result<Arc<dyn Any>, &'static str>;
}

struct ReturnsString;

#[ul_async_trait::async_trait]
impl ReturnsDyn for ReturnsString {
    async fn make_dyn() -> Result<Arc<dyn Any>, &'static str> {
        Ok(Arc::new(String::new()))
    }
}

struct ReturnsUsize;

#[async_trait::async_trait]
impl ReturnsDyn for ReturnsUsize {
    async fn make_dyn() -> Result<Arc<dyn Any>, &'static str> {
        Ok(Arc::new(0usize))
    }
}

// to make sure that the implicit casting actually works
fn returns_dyn() -> Result<Arc<dyn Any>, &'static str> {
    Ok(Arc::new(String::new()))
}

fn main() {}
