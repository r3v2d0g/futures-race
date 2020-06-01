# `futures-race`

A way to poll two futures and get the output of the first one to complete.

## Example

```rust
use futures_race::{Race, RaceExt};
use smol::Timer;
use std::time::Duration;

smol::run(async {
    let foo = async {
        Timer::after(Duration::from_millis(100)).await;
        42
    };

    let bar = async {
        Timer::after(Duration::from_millis(250)).await;
        24
    };

    let foobar = foo.race(bar).await;
    assert_eq!(foobar, 42);
})
```

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
