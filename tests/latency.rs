use future_hooks::prelude::*;
use futures::prelude::*;

#[test]
fn latency() {
    env_logger::init();

    futures::future::ok::<_, ()>(3)
        .and_then(|_| Ok(()))
        .and_then(|_| Ok(()))
        .latency("ok")
        .wait()
        .unwrap();
}
