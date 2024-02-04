use std::io::{ stdout, IsTerminal };

use pico_args::Arguments;

mod command;
mod error;
mod help;
mod util;

use help::{ help, version };

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
	let mut args = Arguments::from_env();


	if args.contains(["-h", "--help"]) {
		help(&mut args);
		return;
	}

	if args.contains(["-v", "--version"]) {
		version();
		return;
	}

	if !stdout().is_terminal() { error::not_terminal(); }

	let subcommand = args.subcommand().unwrap();

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
}

