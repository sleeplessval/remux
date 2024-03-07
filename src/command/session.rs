//!	commands accessible from within a session

use pico_args::Arguments;
use tmux_interface::{
	Tmux,
	commands
};

use crate::{ error, flag, util };

pub fn switch(pargs: &mut Arguments) {
	//	consume optional flags
	let read_only = pargs.contains(flag::READ_ONLY);
	let detach_other = pargs.contains(flag::DETACHED);

	let args = pargs.clone().finish();
	if args.len() < 1 { error::missing_target(); }
	let target = args.get(0).unwrap().to_string_lossy().to_string();
	println!("{target}");

	let exists = util::session_exists(target.clone());
	if !exists { error::no_target(target.clone()); }

	let mut switch = commands::SwitchClient::new();
	switch = switch.target_session(target);

	Tmux::new()
		.add_command(switch)
		.output().ok();
}

