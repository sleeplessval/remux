use std::env::var;

pub type EnvVar = (&'static str, &'static str);

pub static ATTACH_SYMBOL: EnvVar = ("ATTACH_SYMBOL", "*");

pub fn env_var(envvar: EnvVar) -> String {
	var(envvar.0).unwrap_or(envvar.1.to_string())
}

pub fn tmux() -> bool { !var("TMUX").unwrap_or("".to_string()).is_empty() }

