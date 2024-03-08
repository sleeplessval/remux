
# ReMux: a friendlier tmux wrapper

Pronounced \[ ɹ̠i məks \], like "remix."

A tmux wrapper and command shortener written in Rust. ReMux's
goal is to wrap tmux commands to be both shorter, and oriented
around session names instead of session IDs.

To further simplify developer workflows, the `attach`, `detach`, `has`, and
`new` commands will default to the name of the root directory if used inside
a Git repository.

## Goals

- Accelerating: Makes simple tmux workflows faster.
- Friendly: Easy to start using.
- Short: Every ReMux command is as short or shorter than its raw tmux equivalent.

<details>
<summary><h2>Examples</h2></summary>

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

#	switch
tmux switch-client -t foo
remux s foo

```

</details>

## Dependencies

ReMux depends on [tmux](https://github.com/tmux/tmux).

## Installation

### Manual Install

<details>
<summary>Release Binary</summary>

Copy the compiled binary from the [releases page](https://git.vwolfe.io/valerie/remux/releases)
to a directory in `$PATH`, such as `/usr/bin/`.

</details>

<details>
<summary>Compile from Source</summary>

Compile using cargo with the command `cargo build --release` and copy the file
from `target/release/` to a directory in `$PATH`, such as `/usr/bin/`.

</details>

<details>
<summary>makepkg (AUR)</summary>

Clone the [AUR Repository](https://aur.archlinux.org/remux.git) and run the
command `makepkg --install`.

</details>

### Package Managers

<details>
<summary>Arch Linux (AUR): <code>remux</code></summary>

Install the package from the [`remux` AUR Package](https://aur.archlinux.org/packages/remux)
using an AUR package manager such as [`paru`](https://github.com/Morganamilo/paru").

</details>

<details>
<summary>Cargo: <code>tmux-remux</code></summary>

Install the package using Cargo with the command `cargo install tmux-remux`.

</details>

## Configuration

The pretty-print attached symbol (default: `*`) can be set manually by setting `REMUX_ATTACH_SYMBOL`.

## Libraries

- [pico-args](https://crates.io/crates/pico_args) — argument parsing
- [termion](https://crates.io/crates/termion) — ANSI formatting
- [tmux_interface](https://crates.io/crates/tmux_interface) — tmux communication

