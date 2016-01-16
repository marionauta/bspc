# bspc

[![Travis Build Status](https://travis-ci.org/marionauta/bspc.svg)][2]
[![Latest Release](https://img.shields.io/github/release/marionauta/bspc.svg)][3]

Alternative bspc command implementation in Rust. You can control [bspwm][1] with it.

This is mostly a learning project. Nothing should break, but if you see an error please tell me.

## Installing

### Releases

Go to the [latest release][2], download it, extract it and move the binary somewhere in your PATH.

### Cargo install

    cargo install bspc

This is the easiest way. Also you can specify the root path with `--root DIR` so for example

    cargo install --root /usr/local bspc

will install `bspc` in `/usr/local/bin`.

## Contributing

Anyone is welcome to contribute.

[1]: https://github.com/baskerville/bspwm
[2]: https://travis-ci.org/marionauta/bspc
[3]: https://github.com/marionauta/bspc/releases/latest
