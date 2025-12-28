use colored::*;

pub enum LogLevel { Info, Warning, Error, Success }

impl LogLevel {
	pub fn get_level_string(&self) -> ColoredString {
		match self {
			LogLevel::Info => "info".cyan(),
			LogLevel::Warning => "warning".yellow(),
			LogLevel::Error => "error".red(),
			LogLevel::Success => "success".green(),
		}
	}
}

pub fn log(message: &str, level: LogLevel) {
	let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
	println!("[{}] [{}]: {}", timestamp.bright_black(), level.get_level_string(), message);
}
