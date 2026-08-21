use bevy::prelude::*;

pub mod shovel;
pub mod tool_pointer;
pub mod ui;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ui::ToolUIPlugin,
            shovel::ShovelPlugin,
            tool_pointer::ToolPointerPlugin,
        ));
    }
}
