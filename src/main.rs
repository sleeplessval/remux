use std::io::{ stdout, IsTerminal };

use pico_args::Arguments;

mod command;
mod error;
mod util;

fn main() {
	let mut args = Arguments::from_env();

	if args.contains(["-h", "--help"]) {
		command::help(&mut args);
		return;
	}

	if !stdout().is_terminal() { error::not_terminal(); }

	let subcommand = args.subcommand().unwrap();

	match subcommand.as_deref() {
		Some("h" | "help")
			=>	command::help(&mut args),

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
}

