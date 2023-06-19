use std::{
	env::{ set_var, var },
	io::{
		self,
		
		Read, Write
	},
	process::exit
};

use tmux_interface::{
	Session, Sessions,
	variables::session::session::SESSION_ALL
};

use crate::error;

///	return a Vec of all sessions or None
pub fn get_sessions() -> Option<Vec<Session>> {
	let i_sessions = Sessions::get(SESSION_ALL);
	if i_sessions.is_err() { return None; }
	let sessions = i_sessions.ok();
	if sessions.is_none() { return None; }

	Some(sessions.unwrap().0)
}

///	show the tmux nest text if env var is not unset
pub fn prevent_nest() {
	let tmux = var("TMUX").ok();
	if tmux.is_some() && tmux.unwrap() != "" {
		//	ask the user if they want to nest (default: no)
		print!("Are you sure you want to nest sessions? (y/N) ");
		let _ = io::stdout().flush();

		let mut input = [0];
		let _ = io::stdin().read(&mut input);
		match input[0] as char {
			'y' | 'Y'
				=> {},
			_ => error::nest_declined()
		}
		set_var("TMUX", "");
	}
}

