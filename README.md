
# ReMux: a friendlier tmux wrapper

Pronounced \[ ɹ̠i məks \], like "remix."

A tmux wrapper and command shortener written in Rust. ReMux's
goal is to wrap tmux commands to be both shorter, and oriented
around session names instead of session IDs.

To further simplify developer usage, the `attach`, `detach`, `has`, and `new`
commands can be used without a target field, and will default to the name of
the Git repository root directory, if one is found.

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

### Manual Install

<details>
<summary>Release Binary</summary>
Copy the compiled binary from the <a href="https://git.vwolfe.io/valerie/remux/releases">releases page</a>
to a directory in <code>$PATH</code>, such as <code>/usr/bin/</code>.
</details>

<details>
<summary>Compile from Source</summary>
Compile using cargo with the command <code>cargo build --release</code> and copy
the file from <code>target/release/</code> to a directory in <code>$PATH</code>,
such as <code>/usr/bin/</code>.
</details>

<details>
<summary>makepkg (AUR)</summary>
Clone the <a href="https://aur.archlinux.org/remux.git">AUR Repository</a> and
run the command <code>makepkg --install</code>.
</details>

### Package Managers

<details>
<summary>Arch Linux (AUR): <code>remux</code></summary>
Install the package from the <a href="https://aur.archlinux.org/packages/remux"><code>remux</code> AUR Package</a>
using an AUR package manager such as <a href="https://github.com/Morganamilo/paru"><code>paru</code></a>.
</details>

<details>
<summary>Cargo: <code>tmux-remux</code></summary>
Install the package using Cargo with the command <code>cargo install tmux-remux</code>.
</details>

## Configuration

The pretty-print attached symbol (default: `*`) can be set manually by setting `REMUX_ATTACH_SYMBOL`.

## Libraries

- [pico-args](https://crates.io/crates/pico_args) — argument parsing
- [termion](https://crates.io/crates/termion) — ANSI formatting
- [tmux_interface](https://crates.io/crates/tmux_interface) — tmux communication

