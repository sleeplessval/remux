
type Flag = [&'static str;2];

pub static DETACHED: Flag	= ["-d", "--detached"];
pub static HELP: Flag		= ["-h", "--help"];
pub static NEST: Flag		= ["-n", "--nest"];
pub static QUIET: Flag		= ["-q", "--quiet"];
pub static READ_ONLY: Flag	= ["-r", "--read-only"];
pub static TARGET: Flag		= ["-t", "--target"];
pub static VERSION: Flag	= ["-v", "--version"];

