use crate::settings::{Settings, SettingsMessage};
use iced::widget::{Toggler, Column, TextInput, Row, Text};
use crate::application::Element;
use iced_aw::Card;
use iced::Padding;
use iced_native::Alignment;

pub fn settings(settings: &Settings) -> Element {
	let tag_settings = &settings.tag_settings;
	let save_tags: Element = match tag_settings.save_tags {
		false => Toggler::new(Some("Save tags".into()), tag_settings.save_tags, |value| {
			SettingsMessage::ToggleSaveTags(value).into()
		})
		.into(),

		true => Column::with_children(vec![
			Toggler::new(Some("Save tags".into()), tag_settings.save_tags, |value| {
				SettingsMessage::ToggleSaveTags(value).into()
			})
			.into(),
			Column::with_children(vec![
				Toggler::new(Some("\tRemove underscores".into()), tag_settings.remove_underscores, |value| {
					SettingsMessage::ToggleRemoveUnderscores(value).into()
				}).into(),
				
				Toggler::new(Some("\tEscape parentheses".into()), tag_settings.escape_parentheses, |value| {
					SettingsMessage::ToggleEscapeParentheses(value).into()
				}).into(),

				Row::with_children(vec![
					Text::new("Ignored categories").into(),
					TextInput::new("", &tag_settings.ignore_categories, |value| {
						SettingsMessage::IgnoredCategoriesChanged(value).into()
					}).into()
				]).spacing(8).align_items(Alignment::Center).into(),
			])
			.padding(Padding {
				top: 0.0,
				right: 0.0,
				bottom: 8.0,
				left: 16.0,
			})
			.spacing(4)
			.into(),
		])
		.spacing(4)
		.into(),
	};

	let image_settings = &settings.image_settings;
	let apply_letterboxing: Element =
		Toggler::new(Some("Apply letterboxing".into()), image_settings.apply_letterboxing, |value| {
			SettingsMessage::ToggleApplyLetterboxing(value).into()
		})
		.into();

	let content = Column::with_children(vec![save_tags, apply_letterboxing]).spacing(4);

	Card::new("Settings", content)
		.on_close(SettingsMessage::SettingsClosed.into())
		.max_width(512.0)
		.into()
}
