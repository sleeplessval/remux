use std::process::exit;

use pico_args::Arguments;

use crate::error;
use crate::VERSION;

pub fn help(pargs: &mut Arguments) {
	let topic = pargs.subcommand().unwrap();

	match topic.as_deref() {
		None => {
			println!("remux v{VERSION}");
			println!("Valerie Wolfe <sleeplessval@gmail.com>");
			println!("A command wrapper for tmux written in Rust.\n");

			println!("usage: remux <command> [<args>]\n");

			println!("Commands:");
			println!("   help       Show help text for remux or a specific command");
			println!("   attach     Attach to an existing tmux session");
			println!("   detach     Detach clients from a tmux session");
			println!("   has        Check if a tmux session exists");
			println!("   list       Pretty-print all tmux sessions");
			println!("   new        Create a new tmux session");

			println!("\nUse 'remux help <command>' to see detailed help text for each command.");
		},


		Some("a" | "attach")
		=> {
			println!("remux attach");
			println!("Attach to an existing session.\n");

			println!("usage: remux attach [flags] <session> [window]\n");

			println!("args:");
			println!("   <session>          The session to attach to");
			println!("   [window]           Optionally focus a window in the given session\n");

			println!("flags:");
			println!("   -d, --detach       Detach other attached clients from the session");
			println!("   -r, --readonly     Attach the session as read-only");
		},

		Some("d" | "detach")
		=> {
			println!("remux detach");
			println!("Detach all clients from a session.\n");

			println!("usage: remux detach <session>\n");

			println!("args:");
			println!("   <session>      The session name to detach clients from");
		},

		Some("has")
		=> {
			println!("remux has");
			println!("Check if the target session exists.\n");

			println!("usage: remux has [flags] <session>\n");

			println!("args:");
			println!("   <session>      The session to check for\n");

			println!("flags:");
			println!("   -q, --quiet    Display no text; exit code only");
		},

		Some("l" | "ls" | "list")
		=> {
			println!("remux list");
			println!("Pretty-print all tmux sessions.\n");

			println!("usage: remux list");
		},

		Some("n" | "new")
		=> {
			println!("remux new");
			println!("Create a new tmux session.\n");

			println!("usage: remux new [flags] <title> [command]\n");

			println!("args:");
			println!("   <title>                The title of the new session");
			println!("   [command]              The shell command to run\n");

			println!("flags:");
			println!("   -t, --target <dir>     Sets the target directory for the new session.");
		},

								//	not found
		_	=>	error::no_help(topic.unwrap())
	}
}

pub fn version() {
	println!("remux v{VERSION}");
}

