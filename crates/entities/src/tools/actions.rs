use bevy::prelude::*;
use core::fmt;

use crate::tools::Tool;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Reflect, Component)]
pub enum Action {
    Dig,
    Foundation,
    Fence,
    Turret,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Dig => write!(f, "Hole 1x1"),
            Action::Foundation => write!(f, "Foundation 1x1"),
            Action::Fence => write!(f, "Fence 1x1"),
            Action::Turret => write!(f, "Turret 5x5"),
        }
    }
}

pub fn get_tool_actions(tool: Tool) -> Vec<Action> {
    match tool {
        Tool::Shovel => {
            vec![Action::Dig, Action::Foundation]
        }
        Tool::Hammer => {
            vec![Action::Fence, Action::Turret]
        }
        _ => Vec::new(),
    }
}
