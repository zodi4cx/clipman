# clipman

Yet another simple keyboard manager. It is meant to be used with an X dameon such as [sxhkd](https://github.com/baskerville/sxhkd) for quick binding of clipboard values to hotkeys.

## Installation

This crate can be installed in the system using `cargo`:

```bash
git clone https://github.com/zodi4cx/clipman
cd clipman
cargo install --path .
```

## Configuration

Add to your `sxhkd` configuration file (`~/.config/sxhkd/sxhkdrc`) the following bindings:

```
super + ctrl + {F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12}
	clipman save {1,2,3,4,5,6,7,8,9,10,11,12}

super + {F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12}
	clipman load {1,2,3,4,5,6,7,8,9,10,11,12}
```

## Usage

```
Usage: clipman <COMMAND>

Commands:
  get   Retrieves clipboard content
  load  Loads content to clipboard
  save  Saves current clipboard
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
