# TG-RS

[![Travis](https://img.shields.io/travis/tg-rs/tg-rs/master.svg?style=flat-square)](https://travis-ci.org/tg-rs/tg-rs)
[![Codecov](https://img.shields.io/codecov/c/github/tg-rs/tg-rs.svg?style=flat-square)](https://codecov.io/gh/tg-rs/tg-rs)

## Project layout

- [tgbot](tgbot) - A Telegram Bot API client
- [carapax](carapax) - A Telegram Bot framework
- [carapax-access](carapax-access) - An access handler for carapax
- [carapax-i18n](carapax-i18n) - An i18n handler for carapax
- [carapax-ratelimit](carapax-ratelimit) - A ratelimit handler for carapax
- [carapax-session](carapax-session) - A session handler for carapax

## Examples

In order to run examples you need to create a `.env` file:

```
# Logging settings
# See https://docs.rs/env_logger/ for more information
#
# Used in all examples
RUST_LOG=info

# A telegram bot token
#
# Used in all examples
TGRS_TOKEN=YOUR-BOT-TOKEN-HERE

# Proxy:
#
# * http://\[user:password\]@host:port
# * https://\[user:password\]@host:port
# * socks4://userid@host:port
# * socks5://\[user:password\]@host:port
#
# Used in all examples
# TGRS_PROXY='socks5://user:password@host:port'

# Updates will be denied for all except given username
# Specify a username without @
#
# Used in access example
TGRS_ACCESS_USERNAME=username

# A redis URL for session store
#
# Used in session_counter_redis example
TGRS_REDIS_URL=redis://127.0.0.1/0

# An URL to a random gif
#
# Used in send_file example
TGRS_GIF_URL='https://66.media.tumblr.com/3b2ae39de623518901cdbfe87ffde31c/tumblr_mjq1rm7O6Q1racqsfo1_400.gif'

# An URL to a random photo
#
# Used in media_group example
TGRS_PHOTO_URL='https://vignette.wikia.nocookie.net/ergoproxy/images/c/c5/Re-lmayer.png'

# A path to a random photo
#
# Used in send_file and media_group examples
TGRS_PHOTO_PATH='/home/user/data/photo.jpg'

# A path to a random video
#
# Used in send_file and media_group examples
TGRS_VIDEO_PATH='/home/user/data/video.mp4'

# A path to a random document thumbnail
#
# Used in send_file example
TGRS_DOCUMENT_THUMB_PATH='/home/user/data/document_thumb.jpg'

# A rate-limit strategy:
#
# * direct - limit all updates
# * all_users - limit updates per user ID for all users
# * all_chats - limit updates per chat ID for all chats
# * list - limit updates for specific chat id or user id
#
# Used in ratelimit example
TGRS_RATE_LIMIT_STRATEGY=list

# User ID to limit (available for list strategy only)
# Specify an integer user id or username string (without @)
#
# Used in ratelimit example
TGRS_RATE_LIMIT_USER_ID=userid

# Chat ID to limit (available for list strategy only)
# Specify an integer chat id or chat username string (without @)
#
# Used in ratelimit example
TGRS_RATE_LIMIT_CHAT_ID=chatid

# Chat ID for notifications
# Specify an integer chat id or chat username string (without @)
#
# Used in notify example
TGRS_NOTIFICATION_CHAT_ID
```

## Code of Conduct

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
