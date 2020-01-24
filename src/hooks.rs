use futures;
use futures::prelude::*;

pub trait Hook {
    fn on_prepoll(&mut self) {}

    fn on_postpoll(&mut self) {}

    fn on_ready(&mut self) {}

    fn on_notready(&mut self) {}

    fn on_err(&mut self) {}
}

pub struct Hooked<T, H> {
    inner: T,
    hook: H,
}

impl<T, H> Future for Hooked<T, H>
where
    T: Future,
    H: Hook,
{
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.hook.on_prepoll();

        let item = match self.inner.poll() {
            Ok(Async::Ready(item)) => {
                self.hook.on_ready();
                Ok(Async::Ready(item))
            }
            Ok(Async::NotReady) => {
                self.hook.on_notready();
                Ok(Async::NotReady)
            }
            Err(e) => {
                self.hook.on_err();
                Err(e)
            }
        };

        self.hook.on_postpoll();

        item
    }
}

pub trait FutureHookExt: Future {
    fn hook<T: Hook>(self, hook: T) -> Hooked<Self, T>
    where
        Self: Sized,
    {
        Hooked { inner: self, hook }
    }
}

impl<T> FutureHookExt for T where T: Future {}
