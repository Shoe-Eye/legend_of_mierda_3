use bevy::prelude::*;
use core::fmt;

use crate::tools::Tool;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Reflect, Component)]
pub enum Action {
    Dig,
    Fence,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Dig => write!(f, "Dig"),
            Action::Fence => write!(f, "Fence"),
        }
    }
}

pub fn get_tool_actions(tool: Tool) -> Vec<Action> {
    match tool {
        Tool::Shovel => {
            vec![Action::Dig]
        }
        Tool::Hammer => {
            vec![Action::Fence]
        }
        _ => Vec::new(),
    }
}
