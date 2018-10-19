extern crate rust_hawktracer;
use rust_hawktracer::*;
mod utils;
use utils::*;

#[test]
fn tracing_test_network() {
    let hawktracer_instance = HawktracerInstance::new();
    let _listener = hawktracer_instance.create_listener(HawktracerListenerType::TCP {
        port: 12345,
        buffer_size: 4096,
    });

    do_work();
}