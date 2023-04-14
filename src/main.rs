
use pico_args::Arguments;

mod command;
mod error;
mod util;

fn main() {
	let mut args = Arguments::from_env();

	let subcommand = args.subcommand().unwrap();

	//let tmuxvar = var("TMUX");

	match subcommand.as_deref() {
		Some("h")		|	//	help text
		Some("help")	=>	command::help(&mut args),

		Some("a")		|	//	attach
		Some("attach")	=>	command::attach(&mut args),

		Some("has")		=>	command::has(&mut args),

		None			|
		Some("l")		|	//	list
		Some("ls")		|
		Some("list")	=>	command::list(),

		Some("n")		|	//	new
		Some("new")		=>	command::new(&mut args),

		_				=>	error::no_subcommand(subcommand.unwrap())
	}
}

