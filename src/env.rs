use std::env::{ set_var, var };

pub type ENV_VAR = (&'static str, &'static str);

pub static ATTACH_SYMBOL: ENV_VAR = ("ATTACH_SYMBOL", "*");

pub fn env_var(var: ENV_VAR) -> String {
	var(var.0).unwrap_or(var.1.to_string())
}

