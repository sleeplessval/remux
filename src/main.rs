use std::process::exit;

use pico_args::Arguments;

mod command;
mod util;

fn main() {
	let mut args = Arguments::from_env();

	let subcommand = args.subcommand().unwrap();

	//let tmuxvar = var("TMUX");

	match subcommand.as_deref() {
		Some("h")		|	//	help text
		Some("help")	=>	help_text(),

		Some("a")		|	//	attach
		Some("attach")	=>	command::attach(&mut args),

		Some("l")		|	//	list
		Some("ls")		|
		Some("list")	=>	command::list(),

		Some("n")		|	//	new
		Some("new")		=>	command::new(&mut args),

		None			|	//	none should do something else later
		_				=>	{
			println!("no command match for \"{}\"\n", subcommand.unwrap());
			help_text();
			exit(1);
		}
	}
}

fn help_text() {
	println!("remux v{}", env!("CARGO_PKG_VERSION"));
	println!("Valerie Wolfe <sleeplessval@gmail.com>");
	println!("A command wrapper for tmux written in Rust.");
}

