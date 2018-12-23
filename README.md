# TGBOT

A Telegram Bot library

[![Travis](https://img.shields.io/travis/rossnomann/tgbot.svg?style=flat-square)](https://travis-ci.org/rossnomann/tgbot)

# Installation

```toml
[dependencies]
tgbot = "0.1"
```

# Example

```rust
use tgbot::{
    methods::{GetMe, SendMessage},
    types::{MessageKind, UpdateKind},
    Client,
};

fn main() {
    let mut client = Client::new("bot-token");
    // Set a proxy
    // client.proxy(r#"socks5h://user:password@host:port"#).unwrap();
    let r = client.execute(&GetMe);
    println!("{:?}", r);
    for update in client.get_updates().limit(10) {
        if let UpdateKind::Message(msg) = update.kind {
            if let MessageKind::Private { ref chat, ref from } = msg.kind {
                println!("CHAT: {:?}", chat);
                println!("FROM: {:?}", from);
                println!("MSG: {:?}", msg);
                let method = SendMessage::new(chat.id, "test");
                println!("RESULT: {:?}", client.execute(&method));
            }
        }
    }
}
```

# LICENSE

The MIT License (MIT)
