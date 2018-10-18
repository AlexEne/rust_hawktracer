#[allow(dead_code)]
mod internals;

#[allow(unused_imports)]
use internals::base_instance::HawktracerInstance;

use std::path::PathBuf;

pub use internals::scoped_tracepoint::ScopedTracepoint;

pub enum HawktracerInstanceType {
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
pub fn create_hawktracer_instance(
    instance_type: HawktracerInstanceType,
) -> Box<HawktracerInstance> {
    use std::boxed::Box;
    use internals::hawktracer_instance_file::HawktracerInstanceFile;
    use internals::hawktracer_instance_tcp::HawktracerInstanceTCP;

    let instance: Box<HawktracerInstance> = match instance_type {
        HawktracerInstanceType::ToFile {
            file_path,
            buffer_size,
        } => Box::new(HawktracerInstanceFile::new(file_path, buffer_size)),
        HawktracerInstanceType::TCP { port, buffer_size } => {
            Box::new(HawktracerInstanceTCP::new(port, buffer_size))
        }
    };

    instance
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
pub fn create_hawktracer_instance(_instance_type: HawktracerInstanceType) {}
