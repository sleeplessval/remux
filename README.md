
# ReMux: a friendlier tmux wrapper

The main point of this project is shortening commonly-used `tmux`
commands.
Attaching, pretty lists, and creating new sessions with group names
are currently implemented:

```bash

#	new session
tmux new-session -t foo
remux n foo

#	lists
tmux ls
remux l

#	attach
tmux a -t foo
remux a foo

```

