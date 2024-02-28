use std::process::exit;

///	no subcommand that matches user input; code 1
pub fn no_subcommand(subcommand: String) {
	println!("remux: no command match for \"{subcommand}\"");
	exit(1);
}

///	target session not found; code 2
pub fn no_target<S: Into<String>>(target: S) {
	let target = target.into();
	println!("remux: no session \"{target}\" exists");
	exit(2);
}

///	help topic doesn't exist; code 3
pub fn no_help(topic: String) {
	println!("remux: no help for \"{topic}\"");
	exit(3);
}

///	user provided no target; code 4
pub fn missing_target() {
	println!("remux: no target provided");
	exit(4);
}

///	non-terminal environment prevention; code 5
pub fn not_terminal() {
	println!("remux: not running from a terminal");
	exit(5);
}

///	tried to nest while not in a session; code 6
pub fn not_nesting() {
	println!("remux: cannot use nesting flag outside a TMUX session");
	exit(6);
}

