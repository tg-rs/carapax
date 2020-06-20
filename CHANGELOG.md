# Changelog

## 0.8.0 (20.06.2020)

- Added tgbot 0.10.0 support

## 0.7.0 (26.04.2020)

- Added tgbot 0.9 support.

## 0.6.0 (01.04.2020)

- Added tgbot 0.8 support.

## 0.5.1 (28.03.2020)

- Fixed docs.rs build.

## 0.5.0 (08.03.2020)

- All `carapax-*` crates was merged into one `carapax` crate.
  Now you need to enable a corresponding feature in order
  to get access to features provided by those crates.
- Added `Dispatcher::set_error_handler` method.
  Introduced `LoggingErrorHandler` as default error handler.
  Now `ErrorPolicy` is available in public API.
  So that you can easily override error handler and/or change update propagation behavior.
- `seance` dependency was upgraded to 0.3 version.
- Added dialogues support.
- `HandlerError` now is a type alias for `Box<dyn Error>`.
- `CommandDispatcher` was removed, use `#[handler(command = "/name")]` instead.
- `#[handler]` proc macro emits a clear error message when function is not async.
- Value of `command` argument in `#[handler]` proc macro now always requires a leading slash.
- Use `TryFrom/TryInto` when converting an `Update` to `SessionId`.
  We must be sure that `SessionId` always contains `chat_id` and `user_id` in order to prevent bugs.
- `Command` type was moved to `types` module.
- Added tgbot 0.7.0 support.

## 0.4.0 (27.01.2020)

- Added tgbot 0.6 support.

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
