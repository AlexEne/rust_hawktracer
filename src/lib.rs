#[macro_use]
extern crate lazy_static;

#[allow(dead_code)]
mod internals;

#[allow(unused_imports)]
use internals::hawktracer_listener::HawktracerListener;

use std::path::PathBuf;
use internals::hawktracer_instance::HawktracerInstance;

pub use internals::scoped_tracepoint::ScopedTracepoint;

lazy_static! {
    static ref HT_INSTANCE: HawktracerInstance = {
        HawktracerInstance::new()
    };
}

pub enum HawktracerListenerType {
    ToFile {
        file_path: PathBuf,
        buffer_size: usize,
    },
    TCP {
        port: u32,
        buffer_size: usize,
    },
}

#[cfg(feature = "profiling_enabled")]
pub fn create_hawktracer_listener(
    listener_type: HawktracerListenerType,
) -> Box<HawktracerListener> {
    use std::boxed::Box;
    use internals::hawktracer_listener_file::HawktracerListenerFile;
    use internals::hawktracer_listener_tcp::HawktracerListenerTCP;
    
    //I need to trigger the static global initialization somehow.
    let _touch = HT_INSTANCE.touch();

    let listener: Box<HawktracerListener> = match listener_type {
        HawktracerListenerType::ToFile {
            file_path,
            buffer_size,
        } => Box::new(HawktracerListenerFile::new(file_path, buffer_size)),
        HawktracerListenerType::TCP { port, buffer_size } => {
            Box::new(HawktracerListenerTCP::new(port, buffer_size))
        }
    };

    listener
}

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

#[cfg(not(feature = "profiling_enabled"))]
pub fn create_hawktracer_listener(_instance_type: HawktracerListenerType) {}
