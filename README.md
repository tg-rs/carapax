# CARAPAX-SESSION

Session middleware for [carapax](https://github.com/tg-rs/carapax)

[![Travis](https://img.shields.io/travis/tg-rs/carapax-session.svg?style=flat-square)](https://travis-ci.org/tg-rs/carapax-session)
[![Version](https://img.shields.io/crates/v/carapax-session.svg?style=flat-square)](https://crates.io/crates/carapax-session)
[![Downloads](https://img.shields.io/crates/d/carapax-session.svg?style=flat-square)](https://crates.io/crates/carapax-session)
[![Documentation](https://img.shields.io/badge/docs-API-brightgreen.svg?style=flat-square)](https://docs.rs/carapax-session/)
[![License](https://img.shields.io/crates/l/carapax-session.svg?style=flat-square)](./LICENSE)

# Installation

```toml
[dependencies]
carapax-session = "0.1"
```

# Example

```rust
use carapax_session::SessionMiddleware;
let session = SessionMiddleware::new();
app.add_middleware(session);
```

# LICENSE

The MIT License (MIT)
