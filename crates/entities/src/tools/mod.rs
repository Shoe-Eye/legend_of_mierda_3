use std::fmt;

use bevy::prelude::*;

pub mod shovel;
pub mod tool_pointer;
pub mod ui;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Reflect)]
pub enum Tool {
    #[default]
    None,
    Shovel,
    Axe,
    Hammer,
    Pickaxe,
    WateringCan,
}

impl fmt::Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tool::None => write!(f, "No Tool"),
            Tool::Shovel => write!(f, "Shovel"),
            Tool::Axe => write!(f, "Axe"),
            Tool::Hammer => write!(f, "Hammer"),
            Tool::Pickaxe => write!(f, "Pickaxe"),
            Tool::WateringCan => write!(f, "Wayering Can"),
        }
    }
}

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
