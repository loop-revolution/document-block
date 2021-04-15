use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::component::{
		atomic::{icon::Icon},
		layout::{
			card::{CardComponent, CardHeader},
			displaylist::{DisplayListComponent, DisplayListItem},
		},
		menus::menu::{CustomMenuItem, MenuComponent},
		DisplayComponent
	},
	models::Block,
	LoopError,
};

use crate::{blocks::document_block::DocumentBlock, delegation::display::delegate_embed_display};

impl DocumentBlock {
	pub fn handle_embed_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayComponent, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;

		let Self { name, items } = Self::from_id(block.id, user_id, conn)?;

		let name = name
			.and_then(|block| block.block_data)
			.unwrap_or_else(|| "Untitled Group".into());

		let items: Vec<DisplayListItem> = items
			.iter()
			.map(|block| DisplayListItem {
				component: delegate_embed_display(block, context),
				menu: None,
			})
			.collect();

		let list = DisplayListComponent {
			color: block.color.clone(),
			items,
		};

		let mut header = CardHeader {
			block_id: Some(block.id.to_string()),
			icon: Some(Icon::Folder),
			..CardHeader::new(name)
		};

		if let Some(user_id) = user_id {
			let mut menu = MenuComponent::from_block(block, user_id);
			if has_perm_level(user_id, block, PermLevel::Edit) {
				let mut item = CustomMenuItem::new("Add a Block", Icon::Plus);
				item.interact = None;
				menu.custom = Some(vec![item]);
			}
			header.menu = Some(menu);
		}

		Ok(CardComponent {
			color: block.color.clone(),
			content: box list.into(),
			header: box header,
		}
		.into())
	}
}
