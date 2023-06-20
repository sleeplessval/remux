
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

```

## Libraries

- [pico-args](https://crates.io/crates/pico_args) — argument parsing
- [tmux_interface](https://crates.io/crates/tmux_interface) — tmux communication
- [termion](https://crates.io/crates/termion) — ANSI formatting

