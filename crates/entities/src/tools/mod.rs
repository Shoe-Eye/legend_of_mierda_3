use std::fmt;

use bevy::prelude::*;
use lom_assets::{
    loading::CharacterSpritesheets,
    sprites::{AnimatedCharacterSprite, AnimationTimer, CharacterAnimation},
};
use lom_game::GameState;

use crate::{
    player::Player,
    tools::{
        actions::{get_tool_actions, Action},
        ui::{spawn_tool_action_ui, UIToolActionMenu},
    },
};

pub mod actions;
pub mod hammer;
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
            Tool::WateringCan => write!(f, "Watering Can"),
        }
    }
}

#[derive(Message, Clone, Copy)]
pub struct ChooseTool {
    pub tool: Tool,
}

#[derive(Message, Clone, Copy)]
pub struct ChooseAction {
    pub action: Action,
}

pub fn on_choose_tool(
    mut commands: Commands,
    mut er_choose_tool: MessageReader<ChooseTool>,
    mut q_player: Query<(Entity, &mut Sprite, &mut Player)>,
    spritesheets: Res<CharacterSpritesheets>,
    asset_server: Res<AssetServer>,
    q_action_menus: Query<(Entity, &UIToolActionMenu)>,
) {
    for event in er_choose_tool.read() {
        for (_, mut sprite, mut player) in q_player.iter_mut() {
            player.choose_tool(event.tool);
            let actions = get_tool_actions(event.tool);

            spawn_tool_action_ui(&mut commands, &asset_server, actions, q_action_menus);

            sprite.image = match event.tool {
                Tool::None => spritesheets.gennadij_no_tool.clone(),
                Tool::Shovel => spritesheets.gennadij_shovel.clone(),
                Tool::Axe => spritesheets.gennadij_axe.clone(),
                Tool::Hammer => spritesheets.gennadij_hammer.clone(),
                Tool::Pickaxe => spritesheets.gennadij_pickaxe.clone(),
                Tool::WateringCan => spritesheets.gennadij_watering_can.clone(),
            };
        }
    }
}

pub fn on_choose_action(
    mut er_choose_action: MessageReader<ChooseAction>,
    mut q_player: Query<(Entity, &mut Player)>,
) {
    for event in er_choose_action.read() {
        for (_, mut player) in q_player.iter_mut() {
            player.choose_action(Some(event.action));
        }
    }
}

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ui::ToolUIPlugin,
            shovel::ShovelPlugin,
            hammer::HammerPlugin,
            tool_pointer::ToolPointerPlugin,
        ))
        .add_message::<ChooseTool>()
        .add_message::<ChooseAction>()
        .add_systems(
            Update,
            (on_choose_tool, on_choose_action).run_if(in_state(GameState::GamePlay)),
        );
    }
}
