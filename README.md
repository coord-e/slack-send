# `slack-send`

[![Actions Status](https://github.com/coord-e/slack-send/workflows/CI/badge.svg)](https://github.com/coord-e/slack-send/actions?workflow=CI)
[![Actions Status](https://github.com/coord-e/slack-send/workflows/Release/badge.svg)](https://github.com/coord-e/slack-send/actions?workflow=Release)

`slack-send` sends message to Slack via webhook.

## Usage

```shell
# supply content from the argument
$ slack-send -w <HOOK_URL> -c "Hello!"

# supply content from stdin
$ cat content.txt | slack-send -w <HOOK_URL>

# help
$ slack-send --help
slack-send 0.1.0

USAGE:
    slack-send [OPTIONS] --webhook-url <webhook-url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --content <content>
    -m, --mention <mention>             [env: SLACK_SEND_MENTION=]
    -s, --style <style>                 [env: SLACK_SEND_STYLE=]  [default: text]
    -w, --webhook-url <webhook-url>     [env: SLACK_SEND_WEBHOOK_URL=]
```

## Download

Platform|Download
--------|--------
Linux 64-bit|[slack-send-x86_64-unknown-linux-musl](https://github.com/coord-e/slack-send/releases/latest/download/slack-send-x86_64-unknown-linux-musl)
macOS 64-bit|[slack-send-x86_64-apple-darwin](https://github.com/coord-e/slack-send/releases/latest/download/slack-send-x86_64-apple-darwin)
Windows 64-bit|[slack-send-x86_64-pc-windows-msvc.exe](https://github.com/coord-e/slack-send/releases/latest/download/slack-send-x86_64-pc-windows-msvc.exe)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
