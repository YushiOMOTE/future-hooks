mod hooks;
mod latency;
mod log;

pub mod prelude {
    pub use crate::hooks::FutureHookExt;
    pub use crate::info;
    pub use crate::latency::FutureLatencyExt;
    pub use crate::log::FutureLogExt;
}

pub use crate::hooks::{FutureHookExt, Hook, Hooked};
pub use crate::latency::{FutureLatencyExt, Latency};
pub use crate::log::{prefix, FutureLogExt, Logger};
