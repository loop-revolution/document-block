use block_tools::{
	blocks::Context,
	display_api::{
		component::{atomic::text::TextComponent, form::input::InputComponent},
		CreationObject,
	},
	LoopError,
};

use crate::blocks::document_block::DocumentBlock;

impl DocumentBlock {
	pub fn handle_create_display(
		_context: &Context,
		_user_id: i32,
	) -> Result<CreationObject, LoopError> {
		let header = TextComponent::heading("New Document Block");

		let name_input = InputComponent {
			label: Some("Name".to_string()),
			name: Some("NAME".to_string()),
			..InputComponent::default()
		};

		let template: String = r#"{"name": $[NAME]$}"#.to_string();

		let object = CreationObject {
			header_component: header.into(),
			main_component: name_input.into(),
			input_template: template,
		};
		Ok(object)
	}
}
