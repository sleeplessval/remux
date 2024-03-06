use std::{
	env::{ set_var, var },
	io::{ stdout, IsTerminal }
};

use pico_args::Arguments;

mod command;
mod error;
mod flag;
mod help;
mod util;

use help::{ help, version };

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
	//	collect args
	let mut args = Arguments::from_env();

	//	consume flags
	if args.contains(flag::HELP) {
		help(&mut args);
		return;
	}

	if args.contains(flag::VERSION) {
		version();
		return;
	}

	let nesting = args.contains(flag::NEST);
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
			=>	command::share::attach(&mut args),

		Some("d" | "detach")
			=>	command::share::detach(&mut args),

		Some("has")
			=>	command::share::has(&mut args),

		None |
		Some("l" | "ls" | "list")
			=>	command::share::list(),

		Some("n" | "new")
			=>	command::share::new(&mut args),

		_
			=>	error::no_subcommand(subcommand.unwrap())
	}

	//	re-set TMUX var if we unset it for nest mode
	if nesting {
		set_var("TMUX", tmux_var.unwrap());
	}
}

