use std::{
	env::var,
	path::PathBuf,
	process::exit
};

use tmux_interface::{
	Session, Sessions, TmuxCommand,
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
		println!("Sessions should be nested with care; unset TMUX or use the '-n' flag to allow.");
		exit(1);
	}
}

///	check whether a target session exists
pub fn session_exists<S: Into<String>>(target: S) -> bool {
	TmuxCommand::new()
		.has_session()
		.target_session(target.into())
		.output().unwrap()
		.success()
}

///	recursively attempt to find a git root directory
pub fn repo_root(path: PathBuf) -> Option<PathBuf> {
	//	if .git dir is found, return
	if path.join(".git").exists() { return Some(path); }

	//	otherwise, attempt to traverse
	let parent = path.parent();
	if let Some(parent) = parent { repo_root(parent.to_path_buf()) }
	else { None }
}

