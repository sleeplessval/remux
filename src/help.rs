
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
   help       Show help text for remux or a specific command
   attach     Attach to an existing tmux session
   detach     Detach clients from a tmux session
   has        Check if a tmux session exists
   list       Pretty-print all tmux sessions
   new        Create a new tmux session

Use 'remux help <command>' to see detailed help text for each command."),

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
   -r, --readonly     Attach the session as read-only"),

		Some("d" | "detach")
		=>
println!("remux detach
Detach all clients from a session.

usage: remux detach <session>
       remux d <session>

args:
   <session>      The session name to detach clients from"),

		Some("has")
		=>
println!("remux has
Check if the target session exists.

usage: remux has [flags] <session>

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

args:
   <title>                The title of the new session
   [command]              The shell command to run

flags:
   -n, --nest             Create the session inside another session.
   -t, --target <dir>     Sets the target directory for the new session."),

								//	not found
		_	=>	error::no_help(topic.unwrap())
	}
}

pub fn version() {
	println!("remux v{VERSION}");
}

