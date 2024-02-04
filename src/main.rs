use std::{
	env::{ set_var, var },
	io::{ stdout, IsTerminal }
};

use pico_args::Arguments;

mod command;
mod error;
mod help;
mod util;

use help::{ help, version };

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
	//	collect args
	let mut args = Arguments::from_env();

	//	consume flags
	if args.contains(["-h", "--help"]) {
		help(&mut args);
		return;
	}

	if args.contains(["-v", "--version"]) {
		version();
		return;
	}

	let nesting = args.contains(["-n", "--nest"]);
	let tmux_var = var("TMUX").ok();
	if nesting {
		if tmux_var.is_none() {
			error::not_nesting();
		}
		set_var("TMUX", "");
	}

	if !stdout().is_terminal() { error::not_terminal(); }

	let subcommand = args.subcommand().unwrap();

	//	invoke subcommand function
	match subcommand.as_deref() {
		Some("h" | "help")
			=>	help(&mut args),

		Some("a" | "attach")
			=>	command::attach(&mut args),

		Some("d" | "detach")
			=>	command::detach(&mut args),

		Some("has")
			=>	command::has(&mut args),

		None |
		Some("l" | "ls" | "list")
			=>	command::list(),

		Some("n" | "new")
			=>	command::new(&mut args),

		_
			=>	error::no_subcommand(subcommand.unwrap())
	}

	//	re-set TMUX var if we unset it for nest mode
	if nesting {
		set_var("TMUX", tmux_var.unwrap());
	}
}

