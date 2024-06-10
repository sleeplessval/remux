use std::{
	env::{ current_dir, var },
	io::{ stdout, IsTerminal },
	path::PathBuf,
	process::exit
};

use tmux_interface::{
	Session, Tmux,

	commands,
	variables::session::SessionsCtl
};

use crate::error;

///	return a Vec of all sessions or None
pub fn get_sessions() -> Option<Vec<Session>> {
	let sessions = SessionsCtl::new().get_all();
	if let Ok(sessions) = sessions {
		return Some(sessions.0);
	} else { return None; }
}

///	show the tmux nest text if env var is not unset
pub fn prevent_nest() {
	let tmux = var("TMUX").ok();
	if tmux.is_some() && tmux.unwrap() != "" {
		println!("Sessions should be nested with care; unset TMUX or use the '-n' flag to allow.");
		exit(1);
	}
}

///	enforce a command is being used in-session
pub fn session_enforce(cmd: &'static str) {
	let tmux = var("TMUX").unwrap_or("".to_string());
	if tmux.is_empty() {
		error::not_in_session(cmd);
	}
}

///	check whether a target session exists
pub fn session_exists<S: Into<String>>(target: S) -> bool {
	let has_session = commands::HasSession::new()
		.target_session(target.into());
	Tmux::new().add_command(has_session)
		.status()
		.unwrap()
		.success()
}

///	enforce a command is being run in a terminal
pub fn terminal_enforce() {
	if !stdout().is_terminal() { error::not_terminal(); }
}

///	attempt to return the repo name or exit
pub fn repo_fallback() -> String {
	let repo = repo_root(current_dir().unwrap());
	if repo.is_none() { error::missing_target(); }

	let target = repo.unwrap().file_name().unwrap().to_string_lossy().to_string();
	target
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

