// #![windows_subsystem = "windows"]

extern crate core;

mod tags;
mod style;
mod search;
mod post_viewer;
mod download;
mod settings;
mod application;

use iced::window::Icon;
use crate::application::Philia;
use crate::settings::SETTINGS_PATH;
use iced::{Application, Settings as WindowSettings};

const ICON_DATA: &[u8] = include_bytes!("../icon.ico");

fn main() {
	let settings = std::fs::read_to_string(SETTINGS_PATH).unwrap_or_default();
	let settings = serde_json::from_str(&settings).unwrap_or_default();

	let mut window_settings = WindowSettings::with_flags(settings);
	window_settings.window.icon = Some(Icon::from_file_data(ICON_DATA, None).unwrap());
	window_settings.window.min_size = Some((1280, 720));
	window_settings.antialiasing = true;
	Philia::run(window_settings).unwrap()
}
