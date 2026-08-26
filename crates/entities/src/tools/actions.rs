use bevy::prelude::*;
use core::fmt;

use crate::tools::Tool;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Reflect, Component)]
pub enum Action {
    Dig,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Dig => write!(f, "Dig"),
        }
    }
}

pub fn get_tool_actions(tool: Tool) -> Vec<Action> {
    match tool {
        Tool::Shovel => {
            vec![Action::Dig]
        }
        _ => Vec::new(),
    }
}
