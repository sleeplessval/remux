use std::{
	env::var,
	process::exit
};

use tmux_interface::{
	Session, Sessions,
	variables::session::session::SESSION_ALL
};

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
		println!("Sessions should be nested with care; unset TMUX to allow.");
		exit(1);
	}
}

