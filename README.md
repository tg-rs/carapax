# TGBOT

A Telegram Bot library

[![Travis](https://img.shields.io/travis/tg-rs/tgbot.svg?style=flat-square)](https://travis-ci.org/tg-rs/tgbot)
[![Codecov](https://img.shields.io/codecov/c/github/tg-rs/tgbot.svg?style=flat-square)](https://codecov.io/gh/tg-rs/tgbot)
[![Version](https://img.shields.io/crates/v/tgbot.svg?style=flat-square)](https://crates.io/crates/tgbot)
[![Downloads](https://img.shields.io/crates/d/tgbot.svg?style=flat-square)](https://crates.io/crates/tgbot)
[![Release Documentation](https://img.shields.io/badge/docs-release-brightgreen.svg?style=flat-square)](https://docs.rs/tgbot/)
[![Master Documentation](https://img.shields.io/badge/docs-master-brightgreen.svg?style=flat-square)](https://tg-rs.github.io/tgbot/tgbot/)
[![License](https://img.shields.io/crates/l/tgbot.svg?style=flat-square)](./LICENSE)

# Installation

```toml
[dependencies]
tgbot = "0.3"
```

# Example

See [examples](https://github.com/tg-rs/tgbot/tree/0.3.0/examples) directory.

# Changelog

## 0.3.0 (12.03.2019)

- Dispatcher moved to [carapax](https://github.com/tg-rs/carapax).
- Added `Update::get_chat_id()`, `Update::get_chat_username()` and `Update::get_user()` methods.
- Added `Message::is_edited()` and `Message::get_chat_username()` methods.
- Added `Message.commands` property.
- Added `UpdatesStreamOptions`.
- Removed `Api::create()` and `Api::with_proxy()` in favor of `Api::new()`.
- Removed `Api::get_updates()`, use `tgbot::handle_updates()` instead.
- `WebhookService` is public now.
- Respect `retry_after` parameter on polling error.

## 0.2.0 (27.02.2019)

- Migrated from curl to hyper.
- Added dispatcher.
- Added webhooks support.

## 0.1.0 (23.12.2018)

- First release.

# LICENSE

The MIT License (MIT)
