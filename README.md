[![Crates.io](https://img.shields.io/crates/v/rust_hawktracer.svg)](https://crates.io/crates/rust_hawktracer)
[![](https://github.com/AlexEne/rust_hawktracer/workflows/Build/badge.svg)](https://github.com/AlexEne/rust_hawktracer/actions)
[![](https://github.com/AlexEne/rust_hawktracer/workflows/Test/badge.svg)](https://github.com/AlexEne/rust_hawktracer/actions)


# rust_hawktracer
Rust bindings for the [Hawktracer](https://github.com/loganek/hawktracer) profiler.  
This crate offers simple, minimal bindings to help you profile your rust programs.  
If profiling is not enabled by specifying ```features=["profiling_enabled"]```, having tracepoints in your code has absolutely no overhead (everything gets removed at compile time).


![alt text](https://github.com/AlexEne/alexene.github.io/raw/master/images/rust_hawktracer/demo.png)


## Tools needed

You need an external tool in order to transform captured profiling data from a binary format to something that can be interpreted by __chrome:://tracing__ (or other clients).

I recommend downloading the binaries from the official [hawktracer release](https://github.com/loganek/hawktracer/releases/tag/v0.7.0).
 
For platforms that don't have a binary release you can build it from the main [hawktracer repo](https://github.com/loganek/hawktracer).  

## Profiling code
In `Cargo.toml`:
```toml
[dependencies.rust_hawktracer]
version = "0.5.0"
features=["profiling_enabled"]
```

If the bindings that come with it don't match what your platform expects change it to:
```toml
features=["profiling_enabled", "generate_bindings"]
```

In your main.rs:

```rust
#[macro_use]
extern crate rust_hawktracer;
use rust_hawktracer::*;
use std::{thread, time};

#[hawktracer(trace_this)]
fn method_to_trace() {
    thread::sleep(time::Duration::from_millis(1));
}

fn main() {
    let instance = HawktracerInstance::new();
    let _listener = instance.create_listener(HawktracerListenerType::ToFile {
        file_path: "trace.bin".into(),
        buffer_size: 4096,
    });

    // For a networked listner
    // let _listener = instance.create_listener(HawktracerListenerType::TCP {
    //     port: 12345,
    //     buffer_size: 4096,
    // });

    println!("Hello, world!");
    {
        scoped_tracepoint!(_test);
        thread::sleep(time::Duration::from_millis(10));

        {
            for _ in 0..10 {
                scoped_tracepoint!(_second_tracepoint);
                thread::sleep(time::Duration::from_millis(10));
            }
        }
    }
}
```

## Visualization

If you use ```HawktracerListenerType::ToFile```:  
```
.\hawktracer-converter.exe --source trace.bin --output trace.json
```

If you use ```HawktracerListenerType::TCP``` you can listen and capture traces by specifying the IP:port as the ```--source``` parameter:  
```
.\hawktracer-converter.exe --source 127.0.0.1:12345 --output trace.json
```

Open a chrome browser and go to this address: ```chrome://tracing/```

By opening the ```trace.json``` for the program above you should see something like:

![alt text](https://github.com/AlexEne/alexene.github.io/raw/master/images/rust_hawktracer/trace_demo.png)


## Things to watch out for

In rust macros I can't create new identifier names. This means that if you want to avoid warnings, the tracepoint names have to start with a leading ```_```, as in ```scoped_tracepoint!(_my_tracepoint_name)```.  
This doesn't apply to the function annotations.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

at your option.
