#[allow(dead_code)]
mod internals;

#[allow(unused_imports)]
use crate::internals::hawktracer_listener::HawktracerListener;

pub use crate::internals::scoped_tracepoint::ScopedTracepoint;
pub use crate::internals::hawktracer_instance::HawktracerListenerType;
pub use crate::internals::hawktracer_instance::HawktracerInstance;

#[macro_export]
#[cfg(feature = "profiling_enabled")]
macro_rules! scoped_tracepoint {
    ($name:ident) => {
        let tracepoint_name = concat!(stringify!($name), "\0");
        ScopedTracepoint::start_trace(tracepoint_name.as_ptr() as _);
        let $name = ScopedTracepoint {};
    };
}

#[macro_export]
#[cfg(not(feature = "profiling_enabled"))]
macro_rules! scoped_tracepoint {
    ($name:ident) => {
        ()
    };
}
