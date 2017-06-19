# Title Case (titlecase)

`titlecase` is a small tool that capitlizes text according to a style defined
by John Gruber for post titles on his website [Daring Fireball]. `titlecase`
runs on Linux, macOS, FreeBSD, NetBSD, and Windows. A dependency free, single
file binary is built for each supported platform for [every release][releases].

[![Travis CI Badge](https://travis-ci.org/wezm/titlecase.svg?branch=master)](https://travis-ci.org/wezm/titlecase)
[![Windows build status](https://ci.appveyor.com/api/projects/status/github/wezm/titlecase?svg=true)](https://ci.appveyor.com/project/wezm/titlecase)
[![crates.io Badge](https://img.shields.io/crates/v/titlecase.svg)](https://crates.io/crates/titlecase)

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

## Usage

`titlecase` reads lines of text from **stdin** and prints title cased versions
to **stdout**.

## Installation

[Pre-built `titlecase` binaries][releases] are available for Linux, macOS,
FreeBSD, NetBSD, and Windows.

### Arch Linux

`titlecase` is available in the Arch User Repository (AUR):

```
% git clone https://aur.example.com/titlecase.git
% cd titlecase
% makepkg -si
```

### Rust Developer/Cargo

If you have a stable Rust compiler toolchain installed you can install
`titlecase` with cargo:

```
% cargo install titlecase
```

## Style

Instead of
simply capitalizing each word it does the following (amongst other things):

* Lower case small words like an, of, or in.
* Don't capitalize words like iPhone.
* Don't interfere with file paths, URLs, domains, and email addresses.
* Always capitalize the first and last words, even if they are small words
  or surrounded by quotes.
* Don't interfere with terms like "Q&A", or "AT&T".
* Capitalize small words after a colon.

## Credits

This tool makes use of prior work by [John Gruber], [Aristotle Pagaltzis], and
[David Gouch].



[Daring Fireball]: https://daringfireball.net/
[John Gruber]: https://daringfireball.net/2008/05/title_case
[Aristotle Pagaltzis]: http://plasmasturm.org/code/titlecase/
[David Gouch]: http://individed.com/code/to-title-case/
[releases]: https://github.com/wezm/titlecase/releases
[MIT]: https://github.com/wezm/titlecase/blob/master/LICENSE
