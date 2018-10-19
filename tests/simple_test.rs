extern crate rust_hawktracer;
use rust_hawktracer::*;
use std::fs;
mod utils;
use utils::*;

// TODO figure out how to allow this. for now it requires a mutable borrow on instance
#[test]
fn tracing_test_two_listeners() {
    let file_name = "file_name.htdump";
    fs::remove_file(file_name);
    
    let hawktracer_instance = HawktracerInstance::new();
    let _file_listener = hawktracer_instance.create_listener(HawktracerListenerType::ToFile {
        file_path: file_name.into(),
        buffer_size: 4096,
    });

    let _network_listener = hawktracer_instance.create_listener(HawktracerListenerType::TCP {
        port: 12344,
        buffer_size: 4096,
    });

    do_work();

    #[cfg(feature = "profiling_enabled")]
    {
        assert!(std::path::Path::new(file_name).exists());
        fs::remove_file(file_name);
    }
}

// Listeners can't outlive the hawktracer instance.
// If we try this we get the following nice error.
//error[E0597]: `hawktracer_instance` does not live long enough
//   --> tests\simple_test.rs:83:9
//    |
// 83 |         hawktracer_instance.create_listener(HawktracerListenerType::ToFile {
//    |         ^^^^^^^^^^^^^^^^^^^ borrowed value does not live long enough
// ...
// 87 |     };
//    |     - `hawktracer_instance` dropped here while still borrowed
// 88 | }
//    | - borrowed value needs to live until here
// fn should_not_compile() {
//     let listener = {
//         let hawktracer_instance = HawktracerInstance::new();
//         hawktracer_instance.create_listener(HawktracerListenerType::ToFile {
//             file_path: "a".into(),
//             buffer_size: 4096,
//         })
//     };
// }
