#![expect(renamed_and_removed_lints)]
use std::{
    sync::{Arc, atomic::{AtomicU8, Ordering}},
    thread, task::{Wake, Waker, Context, Poll},
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
trait Incrementer {
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn increment<'async_trait>(
        _store_on_drop: StoreOnDrop,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait,
        >,
    >;
}
struct Inc;
impl Incrementer for Inc {
    fn increment<'async_trait>(
        _store_on_drop: StoreOnDrop,
    ) -> ::core::pin::Pin<
        ::std::boxed::Box<
            dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait,
        >,
    > {
        #[allow(clippy::type_complexity)]
        fn inner<'a>(
            _store_on_drop: StoreOnDrop,
        ) -> ::core::pin::Pin<
            ::std::boxed::Box<
                dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'a,
            >,
        > {
            ::std::boxed::Box::pin(
                #[allow(clippy::async_yields_async, clippy::diverging_sub_expression)]
                async move {
                    let _store_on_drop = _store_on_drop;
                    if let ::core::option::Option::Some(ret) = ::core::option::Option::None::<
                        (),
                    > {
                        return ret;
                    }
                    let ret: () = {
                        COUNTER.store(1, Ordering::Relaxed);
                    };
                    #[allow(unreachable_code)] ret
                },
            )
        }
        inner(_store_on_drop)
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
            Poll::Ready(()) => break,
        }
    }
    if STORED_VALUE.load(Ordering::Relaxed) != 1u8 {
        std::process::exit(1);
    }
}
