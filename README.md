# grammers-friendly

A set of Rust libraries to make writing bots to Telegram with [grammers] easier.

## Current status

Just working.


## How to use

### Installing

Just put the code below to your `Cargo.toml`:

```toml
grammers-friendly = { git = "https://github.com/AndrielFR/grammers-friendly" }
```

### Dispatcher

You can use the dispatcher like this:
```rust
use grammers_friendly::prelude::*;

use crate::modules::I18n;

    ...
    Dispatcher::default()
        .add_module(I18n::new("pt-BR"))
        .add_handler(Handler::new_message(test_handler, filters::text("hi!")))
        .run(client.clone())
        .await?;
    ...
```

It will just listen to every update sent by Telegram.

### Handlers

You can create handlers easy peazy.
```rust
use grammers_client::{Client, InputMessage, Update};
use grammers_friendly::prelude::*;

use crate::modules::I18n;

    ...
    let handler = Handler::new_message(test_handler, filters::text("hi!"))
    ...

async fn test_handler(_client: Client, update: Update, data: Data) -> Result<(), Box<dyn std::error::Error> {
    // Get the I18n module
    let i18n = data.get_module::<I18n>().unwrap();
    let t = |key: &str| = i18n.get(key);

    let message = update.get_message().unwrap();

    message.reply(InputMessage::text(t("hello"))).await?;

    Ok(())
}
```

## License

All the libraries and binaries contained in this repository are licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE] or
  http://www.apache.org/licenses/LICENSE-2.0)

* MIT license ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Thank you for considering to contribute! I'll try my best to provide quick, constructive feedback
on your issues or pull requests. Please do call me out if you think my behaviour is not acceptable
at any time. I will try to keep the discussion as technical as possible. Similarly, I will not
tolerate poor behaviour from your side towards other people (including myself).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[async-trait]: https://github.com/dtolnay/async-trait

[grammers]: https://github.com/Lonami/grammers
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
