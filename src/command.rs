use std::{
	env::current_dir,
	process::exit
};

use pico_args::Arguments;
use termion::{ color, style };
use tmux_interface::TmuxCommand;

use crate::util;

pub fn attach(pargs: &mut Arguments) {
	let args = pargs.clone().finish();
	let target = args.get(0).unwrap().to_string_lossy();

	let tmux = TmuxCommand::new();
	let exists = tmux
		.has_session()
		.target_session(target.clone())
		.output().unwrap();
	if !exists.success() {
		println!("no session \"{target}\" exists!");
		exit(2);
	}
	tmux
		.attach_session()
		.target_session(target)
		.output().ok();
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

	let target_dir: Result<String, Error> = pargs.value_from_str("--target");
	let command: Result<String, Error> = pargs.value_from_str("-c");

	let args = pargs.clone().finish();
	let title = args.get(0).unwrap().to_string_lossy();

	let tmux = TmuxCommand::new();
	let mut new = tmux.new_session();

	if command.is_ok() { new.shell_command(command.unwrap()) } else { &mut new }
		.group_name(title)
		.attach()
		.start_directory(target_dir.unwrap_or(current_dir().unwrap().to_string_lossy().to_string()))
		.output().ok();
}

