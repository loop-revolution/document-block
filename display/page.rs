use crate::{blocks::{data_block::DataBlock, document_block::DocumentBlock}, delegation::display::delegate_embed_display};
use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::{
		component::{
			atomic::{icon::Icon, text::TextComponent},
			form::input::{InputComponent, InputSize},
			layout::stack::StackComponent,
			menus::menu::{CustomMenuItem, MenuComponent},
			DisplayComponent, WrappedComponent,
		},
		DisplayMeta, DisplayObject, PageMeta,
	},
	models::{Block, User},
	LoopError,
};

impl DocumentBlock {
	pub fn handle_page_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayObject, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;
		let user = if let Some(id) = user_id {
			User::by_id(id, conn)?
		} else {
			None
		};
		let is_root = match &user {
			None => false,
			Some(user) => user.root_id == Some(block.id),
		};

		// Get all the blocks properties
		let Self {
			name,
			items,
		} = Self::from_id(block.id, user_id, conn)?;

		let name_string = name.clone().and_then(|block| block.block_data);
		let items: Vec<WrappedComponent> = items
			.into_iter()
			.map(|block| WrappedComponent::from(delegate_embed_display(&block, context)))
			.collect();

		let stack: DisplayComponent = if items.is_empty() {
			TextComponent::info("No items in group").into()
		} else {
			StackComponent::new(items).into()
		};
		let mut content = StackComponent::vertical();

		content.push(stack);

		let mut page = PageMeta::default();
		let header_backup = name_string.unwrap_or_else(|| "Untitled Group".into());

		if let Some(user) = user {
			page.menu = Some(MenuComponent::from_block(block, user.id));
			if !is_root {
				if let Some(name) = name {
					if has_perm_level(user.id, &name, PermLevel::Edit) {
						page.header_component = Some(
							InputComponent {
								label: Some("Group Name".into()),
								size: Some(InputSize::Medium),
								..DataBlock::masked_editable_data(
									name.id.to_string(),
									name.block_data,
									true,
								)
							}
							.into(),
						);
					} else {
						page.header = Some(header_backup)
					}
				}
			}
			if let Some(mut menu) = page.menu.clone() {
				if has_perm_level(user.id, &block, PermLevel::Edit) {
					let item = CustomMenuItem {
						interact: None,
						..CustomMenuItem::new("Add a Block", Icon::Plus)
					};
					menu.custom = Some(vec![item]);
					page.menu = Some(menu)
				}
			}
		} else {
			page.header = Some(header_backup)
		}

		let meta = DisplayMeta {
			page: Some(page),
			..Default::default()
		};
		Ok(DisplayObject {
			meta: Some(meta),
			..DisplayObject::new(content.into())
		})
	}
}
