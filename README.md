# Title Case (titlecase)

`titlecase` is a small tool that capitalizes English text [according to a
style][style] defined by John Gruber for post titles on his website [Daring
Fireball].  `titlecase` runs on Linux, macOS, FreeBSD, NetBSD, OpenBSD, and Windows.
Pre-built binaries are available for download below.

[![Travis CI](https://travis-ci.org/wezm/titlecase.svg?branch=master)](https://travis-ci.org/wezm/titlecase)
[![crates.io](https://img.shields.io/crates/v/titlecase.svg)](https://crates.io/crates/titlecase)
[![Documentation](https://docs.rs/titlecase/badge.svg)][crate-docs]

`titlecase` is licensed under the [MIT license][MIT].

## Examples

```
% echo 'Being productive on linux' | titlecase
Being Productive on Linux

% echo 'Finding an alternative to Mac OS X — part 2' | titlecase
Finding an Alternative to Mac OS X — Part 2

% echo 'an example with small words and sub-phrases: "the example"' | titlecase
An Example With Small Words and Sub-Phrases: "The Example"

```

## Command Line Usage

`titlecase` reads lines of text from **stdin** and prints title cased versions
to **stdout**.

## Usage as a Rust Crate

See the [crate documentation][crate-docs].

## Download

Pre-built `titlecase` binaries are available:

* [FreeBSD 12.1 amd64](https://releases.wezm.net/titlecase/1.1.0/titlecase-1.1.0-x86_64-unknown-freebsd.tar.gz)
* [NetBSD 7+ amd64](https://releases.wezm.net/titlecase/1.1.0/titlecase-1.1.0-x86_64-unknown-netbsd.tar.gz)
* [OpenBSD 6.6 amd64](https://releases.wezm.net/titlecase/1.1.0/titlecase-1.1.0-x86_64-unknown-openbsd.tar.gz)
* [Linux x86\_64](https://releases.wezm.net/titlecase/1.1.0/titlecase-1.1.0-x86_64-unknown-linux-musl.tar.gz)
* [Mac OS](https://releases.wezm.net/titlecase/1.1.0/titlecase-1.1.0-x86_64-apple-darwin.tar.gz)
* [Raspberry Pi](https://releases.wezm.net/titlecase/1.1.0/titlecase-1.1.0-arm-unknown-linux-gnueabihf.tar.gz)
* [Windows](https://releases.wezm.net/titlecase/1.1.0/titlecase-1.1.0-x86_64-pc-windows-gnu.zip)

## Building

**Minimum Supported Rust Version:** 1.30.1

If you have a stable Rust compiler toolchain installed you can install
the most recently released `titlecase` with cargo:

```
% cargo install titlecase
```

## Style

Instead of simply capitalizing each word `titlecase` does the following
([amongst other things][style]):

* Lower case small words like an, of, or in.
* Don't capitalize words like iPhone.
* Don't interfere with file paths, URLs, domains, and email addresses.
* Always capitalize the first and last words, even if they are small words
  or surrounded by quotes.
* Don't interfere with terms like "Q&A", or "AT&T".
* Capitalize small words after a colon.

## Credits

This tool makes use of prior work by [John Gruber][style], [Aristotle
Pagaltzis], and [David Gouch].

[Daring Fireball]: https://daringfireball.net/
[style]: https://daringfireball.net/2008/05/title_case
[Aristotle Pagaltzis]: http://plasmasturm.org/code/titlecase/
[David Gouch]: http://individed.com/code/to-title-case/
[releases]: https://github.com/wezm/titlecase/releases
[MIT]: https://github.com/wezm/titlecase/blob/master/LICENSE
[crate-docs]: https://docs.rs/crate/titlecase
