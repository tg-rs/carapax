# Changelog

## 0.2.0 (xx.05.2019)

- `App::new()` now takes no arguments.
- Added `api` argument to `App::run()` method.
- `App::run()` now returns a future.
- Changed API for handlers.
- Removed middlewares support, use handlers instead.
- Removed `Dispatcher` and `DispatcherFuture` from public API.
- Access middleware moved to carapax-access crate.
- Rate limit middleware moved to carapax-ratelimit crate.

## 0.1.0 (12.03.2019)

- First release.
