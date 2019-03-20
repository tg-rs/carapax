# CARAPAX-RATELIMIT

Rate limit handler for [carapax](https://github.com/tg-rs/carapax)

[![Travis](https://img.shields.io/travis/tg-rs/carapax-ratelimit.svg?style=flat-square)](https://travis-ci.org/tg-rs/carapax-ratelimit)
[![Version](https://img.shields.io/crates/v/carapax-ratelimit.svg?style=flat-square)](https://crates.io/crates/carapax-ratelimit)
[![Downloads](https://img.shields.io/crates/d/carapax-ratelimit.svg?style=flat-square)](https://crates.io/crates/carapax-ratelimit)
[![Documentation](https://img.shields.io/badge/docs-API-brightgreen.svg?style=flat-square)](https://docs.rs/carapax-ratelimit/)
[![License](https://img.shields.io/crates/l/carapax-ratelimit.svg?style=flat-square)](./LICENSE)

# Installation

```toml
[dependencies]
carapax-ratelimit = "0.1"
```

# Example

```rust
use carapax_ratelimit::{RateLimitMiddleware, nonzero};
// take 1 update per 5 seconds
let rate_limit = RateLimitMiddleware::direct(nonzero!(1u32), 5);
app.add_handler(Handler::update(rate_limit))
```

# LICENSE

The MIT License (MIT)
