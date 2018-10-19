[![Crates.io](https://img.shields.io/crates/v/rust_hawktracer.svg)](https://crates.io/crates/rust_hawktracer)
[![Build Status](https://travis-ci.org/AlexEne/rust_hawktracer.svg?branch=master)](https://travis-ci.org/AlexEne/rust_hawktracer)
[![Build status](https://ci.appveyor.com/api/projects/status/3nejp7wvwddq5wnq?svg=true)](https://ci.appveyor.com/project/AlexEne/rust-hawktracer)

# rust_hawktracer
Rust bindings for Amazon's Hawktracer profiler.
This offers simple, minimal bindings to help you profile your programs.


![alt text](https://github.com/AlexEne/alexene.github.io/raw/master/images/rust_hawktracer/demo.png)


## Warning
You need an external tool in order to transform bindings from a binary format to something that can be interpreted by __chrome:://tracing__
This tool can be build for now from the main hawktracer repo (client/hawktracer-to-json).
I recommend taking the binaries from the official rust_hawktracer releases: https://github.com/amzn/hawktracer/releases/tag/v0.6.0

## How to use
In Cargo.toml:
```toml
[dependencies.rust_hawktracer]
version = "0.3.0"
features=["profiling_enabled"]
```

In your main.rs:

```rust
#[macro_use]
extern crate rust_hawktracer;
use rust_hawktracer::*;
use std::{thread, time};

fn main() {
    let _listener = create_hawktracer_listener(HawktracerListenerType::ToFile {
        file_path: "trace.bin".into(),
        buffer_size: 4096,
    });

    // For a networked listner
    // let _listener = create_hawktracer_listener(HawktracerListenerType::TCP {
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

Download _hawktracer-converter.exe_ and use it like this:  
If you use ```HawktracerListenerType::ToFile```:  
```
.\hawktracer-converter.exe --source trace.bin --output trace.json
```

If you use ```HawktracerListenerType::TCP``` you can listen and capture traces by specifying the IP:port as the ```--source``` parameter:  
```
.\hawktracer-converter.exe --source 127.0.0.1:12345 --output trace_12345.json
```

Open a chrome browser and go to this address: ```chrome://tracing/```

For the program above you should see the following trace:

![alt text](https://github.com/AlexEne/alexene.github.io/raw/master/images/rust_hawktracer/trace_demo.png)


## Things to watch out for.

In rust macros I can't create new variable names right now, this means that if you want to avoid warnings, the tracepoint variable names have to start with a leading ```_```, as in ```scoped_tracepoint!(_second_tracepoint)```.
If you figure out a way to do this, feel free to raise a PR / issue.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

at your option.
