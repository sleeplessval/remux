//! globally available tmux commands.
use std::{
	env::var,
	ffi::OsString,
	process::exit
};

use pico_args::{ Arguments, Error };
use termion::{ color, style };
use tmux_interface::{
	Tmux,
	commands
};

use crate::{ error, flag, util };

pub fn attach(pargs: &mut Arguments) {
	//	don't allow unflagged nests
	util::prevent_nest();

	//	consume optional flags
	let read_only = pargs.contains(flag::READ_ONLY);
	let detach_other = pargs.contains(flag::DETACHED);

	let args = pargs.clone().finish();
	let target: String;
	let window: Option<&OsString>;
	if args.len() < 1 {
		//	missing name will attempt to fall back to repository
		target = util::repo_fallback();
		if !util::session_exists(target.clone()) { error::missing_target(); }
		window = None;
	} else {
		target = args.get(0).unwrap().to_string_lossy().to_string();
		window = args.get(1);
	}

	//	make sure the session exists
	let exists = util::session_exists(target.clone());
	if !exists { error::no_target(target.clone()); }

	//	 build attach command
	let mut attach = commands::AttachSession::new();
	attach = attach.target_session(target);
	if read_only { attach.read_only = true; }
	if detach_other { attach.detach_other = true; }

	let select_window: Option<commands::SelectWindow>;
	if let Some(window) = window {
		let mut command = commands::SelectWindow::new();
		command.target_window = Some(window.to_string_lossy());
		select_window = Some(command);
	} else { select_window = None; }

	//	build dispatch
	let mut tmux = Tmux::new().add_command(attach);
	if let Some(select_window) = select_window { tmux = tmux.add_command(select_window); }
	tmux.disable_echo().output().ok();
}

pub fn detach(pargs: &mut Arguments) {
	//	get target or fallback
	let args = pargs.clone().finish();
	let target: String;
	if args.len() < 1 {
		target = util::repo_fallback();
	} else {
		target = args.get(0).unwrap().to_string_lossy().to_string();
	}

	//	make sure the session exists
	let exists = util::session_exists(target.clone());
	if !exists { error::no_target(target.clone()); }

	//	build and dispatch
	let detach = commands::DetachClient::new()
		.target_session(target);
	Tmux::new()
		.add_command(detach)
		.disable_echo().output().ok();
}

pub fn has(pargs: &mut Arguments) {
	//	consume optional flags
	let quiet = pargs.contains(flag::QUIET);

	//	get target or fallback
	let args = pargs.clone().finish();
	let target: String;
	if args.len() < 1 {
		target = util::repo_fallback();
	} else {
		target = args.get(0).unwrap().to_string_lossy().to_string();
	}

	//	run command
	let success = util::session_exists(target.clone());

	//	print if not quiet
	if !quiet {
		println!("session \"{target}\" {}.",
			if success { "exists" }
			else { "does not exist" }
		);
	}

	//	emit exit code
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

	//	get attached session symbol
	let attach_symbol = var("REMUX_ATTACH_SYMBOL").unwrap_or("*".to_string());

	//	pretty print session list
	println!("sessions:");
	for session in sessions.into_iter() {
		let group = session.group.unwrap_or("[untitled]".to_string());
		let id = session.id.unwrap();
		let attached = session.attached.unwrap_or(0) > 0;

		println!(
			"   {group} ({bold}{blue}{id}{reset}) {bold}{green}{attach}{reset}",
			//	values
			attach		= if attached { attach_symbol.clone() } else { "".to_string() },
			//	formatting
			bold		= style::Bold,
			blue		= color::Fg(color::Blue),
			green		= color::Fg(color::LightGreen),
			reset		= style::Reset
		);
	}
}

pub fn new(pargs: &mut Arguments) {
	//	don't allow unflagged nesting
	util::prevent_nest();

	//	get optional flag
	let target_dir: Result<String, Error> = pargs.value_from_str(flag::TARGET);

	//	get target or fallback
	let args = pargs.clone().finish();
	let title: String;
	let command: Option<&OsString>;
	if args.len() < 1 {
		//	attempt repo fallback
		title = util::repo_fallback();
		command = None;
	} else {
		title = args.get(0).unwrap().to_string_lossy().to_string();
		command = args.get(1);
	}

	let mut new = commands::NewSession::new();
	new = new.group_name(title);
	if let Some(command) = command { new.shell_command = Some(command.to_string_lossy()); }
	if let Ok(target_dir) = target_dir { new = new.start_directory(target_dir); }

	Tmux::new()
		.add_command(new)
		.disable_echo().output().ok();
}

