use std::{
	env::current_dir,
	process::exit
};

use pico_args::Arguments;
use termion::{ color, style };
use tmux_interface::TmuxCommand;

use crate::error;
use crate::util;

pub fn help(pargs: &mut Arguments) {
	let topic = pargs.subcommand().unwrap();

	match topic.as_deref() {
		None		=> {		//	program
			println!("remux v{}", env!("CARGO_PKG_VERSION"));
			println!("Valerie Wolfe <sleeplessval@gmail.com>");
			println!("A command wrapper for tmux written in Rust.\n");

			println!("usage: remux <command> [<args>]\n");

			println!("Commands:");
			println!("   help       Show help text for remux or a specific command");
			println!("   attach     Attach to an existing tmux session");
			println!("   has        Check if a tmux session exists");
			println!("   list       Pretty-print all tmux sessions");
			println!("   new        Create a new tmux session");

			println!("\nUse 'remux help <command>' to see detailed help text for each command.");
		},


		Some("a")		|		//	attach
		Some("attach")	=> {
			println!("remux attach");
			println!("Attach to an existing session.\n");

			println!("usage: remux attach <session> [window]\n");

			println!("args:");
			println!("   <session>      The session to attach to");
			println!("   [window]       Optionally focus a window in the given session");
		},

								//	has
		Some("has")		=> {
			println!("remux has");
			println!("Check if the target session exists.\n");

			println!("usage: remux has [flags] <session>\n");

			println!("args:");
			println!("   <session>      The session to check for\n");

			println!("flags:");
			println!("   -q, --quiet    Display no text; exit code only");
		},

		Some("l")		|		//	list
		Some("ls")		|
		Some("list")	=> {
			println!("remux list");
			println!("Pretty-print all tmux sessions.\n");

			println!("usage: remux list");
		},

		Some("n")		|		//	new
		Some("new")		=> {
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
		_			=> error::no_help(topic.unwrap())
	}
}

pub fn attach(pargs: &mut Arguments) {
	let args = pargs.clone().finish();
	let target = args.get(0).unwrap().to_string_lossy();
	let window = args.get(1);

	if window.is_some() {
		let target = window.unwrap().to_string_lossy();
		let tmux = TmuxCommand::new();
		tmux
			.select_window()
			.target_window(target)
			.output().ok();
	}

	let tmux = TmuxCommand::new();
	let exists = tmux
		.has_session()
		.target_session(target.clone())
		.output().unwrap();
	if !exists.success() { error::no_target(target.to_string()); }

	tmux
		.attach_session()
		.target_session(target)
		.output().ok();
}

pub fn has(pargs: &mut Arguments) {
	let quiet = pargs.contains(["-q", "--quiet"]);

	let args = pargs.clone().finish();
	let target = args.get(0).unwrap().to_string_lossy();

	let tmux = TmuxCommand::new();
	let exists = tmux
		.has_session()
		.target_session(target.clone())
		.output().unwrap();

	let success = exists.success();
	if !quiet { println!("session \"{target}\" {}.", if success { "exists" } else { "does not exist" }); }
	exit( if success { 0 } else { 1 });
}

pub fn list() {
	let sessions = util::get_sessions();

	println!("sessions:");

	for session in sessions.unwrap().into_iter() {
		let group = session.group.unwrap_or("[untitled]".to_string());
		let id = session.id.unwrap();
		let attached = session.attached.unwrap_or(0) > 0;

		println!(
			"  {group} ({bold}{blue}{id}{reset}) {bold}{green}{attach_sym}{reset}",
			//	values
			attach_sym = if attached { "" } else {""},
			//	formatting
			bold = style::Bold,
			blue = color::Fg(color::Blue),
			green = color::Fg(color::LightGreen),
			reset = style::Reset
		);
	}
}

pub fn new(pargs: &mut Arguments) {
	use pico_args::Error;

	let target_dir: Result<String, Error> = pargs.value_from_str(["-t", "--target"]);

	let args = pargs.clone().finish();
	let title = args.get(0).unwrap().to_string_lossy();
	let command = args.get(1);

	let tmux = TmuxCommand::new();
	let mut new = tmux.new_session();

	if command.is_some() { new.shell_command(command.unwrap().to_string_lossy()) } else { &mut new }
		.group_name(title)
		.attach()
		.start_directory(target_dir.unwrap_or(current_dir().unwrap().to_string_lossy().to_string()))
		.output().ok();
}

