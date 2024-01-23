use std::{
	env::current_dir,
	process::exit
};

use pico_args::Arguments;
use termion::{ color, style };
use tmux_interface::TmuxCommand;

use crate::error;
use crate::util;

pub fn attach(pargs: &mut Arguments) {
	util::prevent_nest();

	//	get optional flags
	let read_only = pargs.contains(["-r", "--readonly"]);
	let detach_other = pargs.contains(["-d", "--detach"]);

	//	get target and error out if not provided
	let args = pargs.clone().finish();
	if args.len() < 1 { error::missing_target(); }
	let target = args.get(0).unwrap().to_string_lossy();
	let window = args.get(1);

	//	focus window if provided
	if window.is_some() {
		let target = window.unwrap().to_string_lossy();
		let tmux = TmuxCommand::new();
		tmux
			.select_window()
			.target_window(target)
			.output().ok();
	}

	//	command
	let tmux = TmuxCommand::new();
	let exists = tmux
		.has_session()
		.target_session(target.clone())
		.output().unwrap();
	if !exists.success() { error::no_target(target.to_string()); }

	let mut attach = tmux.attach_session();

	//	handle optional flags
	if read_only { attach.read_only(); }
	if detach_other { attach.detach_other(); }

	attach
		.target_session(target)
		.output().ok();
}

pub fn detach(pargs: &mut Arguments) {
	//	get target and error out if not provided
	let args = pargs.clone().finish();
	if args.len() < 1 { error::missing_target(); }
	let target = args.get(0).unwrap().to_string_lossy();

	//	command
	let tmux = TmuxCommand::new();
	let exists = tmux
		.has_session()
		.target_session(target.clone())
		.output().unwrap();
	if !exists.success() { error::no_target(target.to_string()); }

	tmux
		.detach_client()
		.target_session(target)
		.output().ok();
}

pub fn has(pargs: &mut Arguments) {
	//	get optional flag
	let quiet = pargs.contains(["-q", "--quiet"]);

	//	get target and error out if not provided
	let args = pargs.clone().finish();
	if args.len() < 1 { error::missing_target(); }
	let target = args.get(0).unwrap().to_string_lossy();

	//	command
	let tmux = TmuxCommand::new();
	let exists = tmux
		.has_session()
		.target_session(target.clone())
		.output().unwrap();

	let success = exists.success();

	//	handle optional flag
	//	inverted; print text if NOT quiet
	if !quiet { println!("session \"{target}\" {}.", if success { "exists" } else { "does not exist" }); }

	//	exit codes for scripts to use
	exit( if success { 0 } else { 1 });
}

pub fn list() {
	//	get session list
	let sessions = util::get_sessions().unwrap_or(Vec::new());

	//	handle empty case
	if sessions.len() == 0 {
		println!("no sessions");
		return;
	}

	//	iterate over pretty print
	println!("sessions:");
	for session in sessions.into_iter() {
		let group = session.group.unwrap_or("[untitled]".to_string());
		let id = session.id.unwrap();
		let attached = session.attached.unwrap_or(0) > 0;

		println!(
			"  {group} ({bold}{blue}{id}{reset}) {bold}{green}{attach_sym}{reset}",
			//	values
			attach_sym = if attached { "󰌹" } else {""},
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

	//	show nest message
	util::prevent_nest();

	//	get optional flag
	let target_dir: Result<String, Error> = pargs.value_from_str(["-t", "--target"]);

	//	get target and error out if not provided
	let args = pargs.clone().finish();
	if args.len() < 1 { error::missing_target(); }

	//	get target session and optional command
	let title = args.get(0).unwrap().to_string_lossy();
	let command = args.get(1);

	let tmux = TmuxCommand::new();
	let mut new = tmux.new_session();

	if command.is_some() { new.shell_command(command.unwrap().to_string_lossy()); }

	new
		.group_name(title)
		.attach()
		.start_directory(target_dir.unwrap_or(current_dir().unwrap().to_string_lossy().to_string()))
		.output().ok();
}

