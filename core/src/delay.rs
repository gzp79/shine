use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

#[pin_project]
pub struct Delay {
    #[cfg(target_arch = "wasm32")]
    #[pin]
    delay: gloo_timers::future::TimeoutFuture,

    #[cfg(not(target_arch = "wasm32"))]
    #[pin]
    delay: tokio::time::Sleep,
}

impl Delay {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(duration: Duration) -> Delay {
        Delay {
            delay: tokio::time::sleep(duration),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(duration: Duration) -> Delay {
        Delay {
            delay: gloo_timers::future::sleep(duration),
        }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().delay.poll(cx)
    }
}
