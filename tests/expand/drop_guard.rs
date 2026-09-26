#![expect(renamed_and_removed_lints)]

use std::{
    sync::{Arc, atomic::{AtomicU8, Ordering}},
    thread,
    task::{Wake, Waker, Context, Poll}
};

struct MyWaker(thread::Thread);

impl Wake for MyWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}


static COUNTER: AtomicU8 = AtomicU8::new(0);
static STORED_VALUE: AtomicU8 = AtomicU8::new(0);

struct StoreOnDrop {}
impl Drop for StoreOnDrop {
    fn drop(&mut self) {
        STORED_VALUE.store(COUNTER.load(Ordering::Relaxed), Ordering::Relaxed);
    }
}

#[async_trait::async_trait]
trait Incrementer {
    async fn increment(_store_on_drop: StoreOnDrop);
}

struct Inc;
#[ul_async_trait::async_trait]
impl Incrementer for Inc {
    async fn increment(_store_on_drop: StoreOnDrop) {
        COUNTER.store(1, Ordering::Relaxed);
    }
}

fn main() {
    let waker = Waker::from(Arc::new(MyWaker(thread::current())));
    let mut context = Context::from_waker(&waker);

    let orig = StoreOnDrop {};
    let mut fut = Inc::increment(orig);

    loop {
        match fut.as_mut().poll(&mut context) {
            Poll::Pending => thread::park(),
            Poll::Ready(()) => break
        }
    }

    if STORED_VALUE.load(Ordering::Relaxed) != 1u8 {
        std::process::exit(1);
    }
}
