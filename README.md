
# ReMux: a friendlier tmux wrapper

Pronounced \[ ɹ̠i məks \], like "remix."

A tmux wrapper and command shortener written in Rust. ReMux's
goal is to wrap tmux commands to be both shorter, and oriented
around session names instead of session IDs.

In their shortest forms, *every* ReMux command is as short or
shorter than its equivalent tmux command:

```sh

#	new session
tmux new-session -t foo
remux n foo

#	lists
tmux ls
remux l
remux

#	attach
tmux a -t foo
remux a foo

#	has
tmux has -t foo
remux has foo

#	detach
tmux detach-client -t foo
remux d foo

#	nesting sessions with '-n' flag
TMUX='' tmux a -t foo
remux a -n foo
TMUX='' tmux new-session -t foo
remux n -n foo

```

## Dependencies

ReMux depends on [tmux](https://github.com/tmux/tmux).

## Installation

### From Binary

Copy the compiled binary from the [releases page](https://git.vwolfe.io/valerie/remux/releases)
to a directory in `$PATH`, such as `/usr/bin/`.

### From Source

Compile using cargo with the command `cargo build --release` and copy the file
from `target/release/` to a directory in `$PATH`, such as `/usr/bin/`.

### Arch Linux (AUR)

Install the package from the [`remux` AUR Package](https://aur.archlinux.org/packages/remux),
either using an AUR package manager, or by cloning the [AUR Repository](https://aur.archlinux.org/remux.git)
and running the command `makepkg --install`.

## Libraries

- [pico-args](https://crates.io/crates/pico_args) — argument parsing
- [termion](https://crates.io/crates/termion) — ANSI formatting
- [tmux_interface](https://crates.io/crates/tmux_interface) — tmux communication

