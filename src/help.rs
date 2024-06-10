
use pico_args::Arguments;

use crate::error;
use crate::VERSION;

pub fn help(pargs: &mut Arguments) {
	let topic = pargs.subcommand().unwrap();

	match topic.as_deref() {
		None =>
println!("remux v{VERSION}
Valerie Wolfe <sleeplessval@gmail.com>
A command wrapper for tmux written in Rust.

usage: remux <command> [<args>]

commands:
   help       Show help text for remux, a command, or a help topic.
   attach     Attach to an existing tmux session
   detach     Detach clients from a tmux session
   has        Check if a tmux session exists
   list       Pretty-print all tmux sessions
   new        Create a new tmux session

Use 'remux help <command>' to see detailed help text for each command.

help topics:
   env        Environment variables"),


//	COMMAND HELP

	Some("a" | "attach")
		=>
println!("remux attach
Attach to an existing session.

usage: remux attach [flags] <session> [window]
       remux a [flags] <session> [window]

args:
   <session>          The session to attach to
   [window]           Optionally focus a window in the given session

flags:
   -d, --detach       Detach other attached clients from the session
   -n, --nest         Attach the session inside another session.
   -r, --read-only    Attach the session as read-only"),

		Some("d" | "detach")
		=>
println!("remux detach
Detach all clients from a session.

usage: remux detach <session>
       remux d <session>

args:
   <session>      The session name to detach clients from"),

		Some("h" | "has")
		=>
println!("remux has
Check if the target session exists.

usage: remux has [flags] <session>
       rmux h [flags] session

args:
   <session>      The session to check for

flags:
   -q, --quiet    Display no text; exit code only"),

		Some("l" | "ls" | "list")
		=>
println!("remux list
Pretty-print all tmux sessions.

usage: remux list
       remux ls
       remux l"),

		Some("n" | "new")
		=>
println!("remux new
Create a new tmux session.

usage: remux new [flags] <title> [command]
       remux n [flags] <title> [command]

args:
   <title>                The title of the new session
   [command]              The shell command to run

flags:
   -n, --nest             Create the session inside another session.
   -t, --target <dir>     Sets the target directory for the new session."),

		Some("root")
		=>
println!("remux root
Print the session path (#{{session_path}}) to standard output.
Must be run from inside a session.

usage: remux root"),

		Some("s" | "switch")
		=>
println!("remux switch
Switch to a different tmux session.
Must be run from inside a session.

usage: remux switch [flags] <title>
       remux s [flags] <title>

args:
   <title>                The title of the session to switch to.

flags:
   -r, --read-only        Attach the target session as read-only."),


//	TOPIC HELP

		Some("env" | "vars")
			=>
println!("remux environment variables

REMUX_ATTACH_SYMBOL
   Changes the symbol displayed for attached sessions displayed
   by the 'list' command.
   Default: '*'

REMUX_NEW_WINDOW
   Provides a default window name when creating a session with
   the 'new' command, if not empty.
   Default: ''"),

		//	not found
		_	=>	error::no_help(topic.unwrap())
	}
}

pub fn version() {
	println!("remux v{VERSION}");
}

