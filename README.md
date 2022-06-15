# Title Case (titlecase)

`titlecase` is a small tool and library (crate) that capitalizes English text
[according to a style][style] defined by John Gruber for post titles on his
website [Daring Fireball]. `titlecase` should run on all platforms supported
by Rust including Linux, macOS, FreeBSD, NetBSD, OpenBSD, and Windows.

[![Build Status](https://api.cirrus-ci.com/github/wezm/titlecase.svg)](https://cirrus-ci.com/github/wezm/titlecase)
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

## Building

**Minimum Supported Rust Version:** 1.40.0

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
[MIT]: https://github.com/wezm/titlecase/blob/master/LICENSE
[crate-docs]: https://docs.rs/titlecase
