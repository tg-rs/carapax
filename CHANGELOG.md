# Changelog

## 0.24.0 (07.09.2024)

- Updated dependencies:
  - seance 0.14
  - tgbot 0.30
  - tokio 1.40

## 0.23.0 (18.08.2024)

- Updated dependencies:
  - tgbot 0.29

## 0.22.0 (31.07.2024)

- Updated dependencies:
  - seance 0.13
  - tgbot 0.28
  - tokio 1.39

## 0.21.0 (07.07.2024)

- Updated dependencies:
  - tgbot 0.27

## 0.20.0 (02.07.2024)

- Updated dependencies:
  - tgbot 0.26

## 0.19.0 (18.06.2024)

- Updated dependencies:
  - seance 0.12
  - tgbot 0.25
  - tokio 1.38
- Inner value of `Ref` is public now.
  This allows to use pattern matching syntax in handler arguments when you need direct access to the underlying data.

## 0.18.0 (29.05.2024)

- Updated dependencies:
  - tgbot 0.24

## 0.17.0 (07.05.2024)

- Updated dependencies:
  - tgbot 0.23

## 0.16.0 (01.04.2024)

- Updated dependencies:
  - seance 0.11
  - tgbot 0.22
  - tokio 1.37

## 0.15.0 (18.02.2024)

- Updated dependencies:
  - seance 0.10
  - tgbot 0.21
  - tokio 1.36

## 0.14.0 (01.01.2024)

- Updated dependencies:
  - seance 0.9
  - tgbot 0.20
  - tokio 1.35
- `async fn` in traits:
  - Removed `TryFromInput::Future` associated type.
  - Removed `Handler::Future` associated type.

## 0.13.0 (05.12.2023)

- Updated dependencies:
  - governor 0.6
  - seance 0.8
  - tgbot 0.19
  - tokio 1.34
- Renamed `add` method of `Chain` struct to `with`.
- Updated `TryFromInput` implementations according to changes in tgbot.
- Renamed shortcuts:
  - `AccessExt`: `access` to `with_access_policy`.
  - `PredicateExt`: `predicate` to `with_predicate`.
  - `CommandExt`: `command` to `with_command`.
  - `DialogueExt`: `dialogue` to `with_dialogue`.
- Extracted a predicate from `DialogueDecorator`.
  This allows to skip a dialogue handler in `Chain` using a first-found strategy.
  As a result, you can now use multiple dialogue handlers.
  Previously, only one handler could be used, and it had to be the last handler in a chain.

## 0.12.0 (10.02.2022)

- Updated tgbot version to 0.18.
- Added `Chain::once` method which allows to run first found handler only.
- Removed `PrincipalUser` and `PrincipalChat` in favor of `UserId` and `ChatId`.

## 0.11.0 (02.02.2022)

- Tokio 1.16 and tgbot 0.17 support.
- New handlers API.
  - Removed `async_trait` and `carapax-codegen` dependencies.
  - Removed `Dispatcher` in favor of `App` and `Chain`.
  - `HandlerResult` is the alias to `Result<(), HandlerError>` now.
  - `HandlerError` now wraps `Box<dyn Error>`.
  - Changed signature of `Handler` trait.
  - Added `HandlerInput` struct containing `Context` and `Update`.
  - Renamed `TryFromUpdate` trait to `TryFromInput`.
  - Removed `ErrorPolicy`.
  - Added `Ref<T>` to allow to pass objects from context to handlers directly.
  - Added `Predicate` handler to allow to wrap handlers with predicates.
  - Added `CommandPredicate` handler to allow to run a handler only for a specific command.
- Replaced `ratelimit_meter` by `governor`.
- Removed i18n support.
- And other breaking changes, see examples for more information.

## 0.10.0 (09.01.2020)

- Added tokio 1.0 and tgbot 0.12 support.

## 0.9.0 (15.11.2020)

- Added tgbot 0.11.0 support.

## 0.8.0 (20.06.2020)

- Added tgbot 0.10.0 support.

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
