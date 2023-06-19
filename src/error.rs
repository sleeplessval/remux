use std::process::exit;

///	no subcommand that matches user input; code 1
pub fn no_subcommand(subcommand: String) {
	println!("remux: no command match for \"{subcommand}\"");
	exit(1);
}

///	target session not found; code 2
pub fn no_target(target: String) {
	println!("remux: no session \"{target}\" exists");
	exit(2);
}
///	no sessions exist; code 2
pub fn no_sessions() {
	println!("remux: no sessions running");
	println!("use 'remux n <title>' to create a new session");
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

