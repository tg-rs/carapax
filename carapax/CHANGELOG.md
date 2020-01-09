# Changelog

## 0.3.1 (10.01.2020)

- Added `CommandDispatcher::new()` method in order to support context without Default impl.
- Fixed handler visibility when using proc macro.

## 0.3.0 (07.01.2020)

- Added async/await support.
- Removed App struct, use Dispatcher instead.
- Function handlers can be implemented using proc macro only.
- Now context is global and generic.
- Added Error variant to HandlerResult.
- Removed CommandHandler trait in favor of Command struct.
- Removed TextRule-based handlers.

## 0.2.0 (07.05.2019)

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
