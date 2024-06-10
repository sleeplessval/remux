//!	commands accessible from within a session
use std::fs::read_to_string;

use pico_args::Arguments;
use tmux_interface::{
	Tmux,
	commands
};

use crate::{ error, flag, util };

const TMP_ROOT: &str = "/tmp/remux_root";

pub fn switch(pargs: &mut Arguments) {
	util::terminal_enforce();
	//	refuse to run outside a session
	util::session_enforce("switch");

	//	consume optional flags
	let read_only = pargs.contains(flag::READ_ONLY);
	//TODO: -d flag handling needs to be done manually

	let args = pargs.clone().finish();
	if args.len() < 1 { error::missing_target(); }
	let target = args.get(0).unwrap().to_string_lossy().to_string();

	let exists = util::session_exists(target.clone());
	if !exists { error::no_target(target.clone()); }

	let mut switch = commands::SwitchClient::new();
	switch = switch.target_session(target);
	if read_only { switch.read_only = true; }

	Tmux::new()
		.add_command(switch)
		.output().ok();
}

pub fn root() {
	util::session_enforce("root");

	let exec = commands::Run::new().shell_command("printf '#{session_path}' > ".to_string() + TMP_ROOT);
	Tmux::new()
		.add_command(exec)
		.output().ok();

	if let Ok(text) = read_to_string(TMP_ROOT) {
		println!("{text}");
		std::fs::remove_file(TMP_ROOT).ok();
	}
}

