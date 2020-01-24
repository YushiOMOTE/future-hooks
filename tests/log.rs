use future_hooks::prelude::*;
use futures::prelude::*;

#[test]
fn logging() {
    env_logger::init();

    futures::future::ok::<_, ()>(3)
        .and_then(|_| {
            info!("hello");
            Ok(())
        })
        .and_then(|_| {
            info!("hello");
            Ok(())
        })
        .log("ok")
        .wait()
        .unwrap();
}
