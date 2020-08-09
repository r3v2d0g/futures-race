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

> This Source Code Form is subject to the terms of the Mozilla Public
> License, v. 2.0. If a copy of the MPL was not distributed with this
> file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
