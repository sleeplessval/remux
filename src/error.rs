use std::process::exit;

pub fn no_subcommand(subcommand: String) {
	println!("remux: no command match for \"{subcommand}\"");
	exit(1);
}

pub fn no_target(target: String) {
	println!("remux: no session \"{target}\" exists");
	exit(2);
}
pub fn no_sessions() {
	println!("remux: no sessions running");
	println!("use 'remux n <title>' to create a new session");
	exit(2);
}

pub fn no_help(topic: String) {
	println!("remux: no help for \"{topic}\"");
	exit(3);
}

