use bevy::prelude::*;
use core::fmt;

use crate::{
    level::turret::{TURRET_SIZE_X, TURRET_SIZE_Y},
    tools::Tool,
};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Reflect, Component)]
pub enum Action {
    Dig { width: u32, height: u32 },
    Foundation { width: u32, height: u32 },
    Fence { width: u32, height: u32 },
    Turret { width: u32, height: u32 },
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Dig { width, height } => write!(f, "Hole {}x{}", width, height),
            Action::Foundation { width, height } => write!(f, "Foundation {}x{}", width, height),
            Action::Fence { width, height } => write!(f, "Fence {}x{}", width, height),
            Action::Turret { width, height } => {
                write!(f, "Turret {}x{}", width, height)
            }
        }
    }
}

pub fn get_tool_actions(tool: Tool) -> Vec<Action> {
    match tool {
        Tool::Shovel => {
            vec![
                Action::Dig {
                    width: 1,
                    height: 1,
                },
                Action::Foundation {
                    width: 1,
                    height: 1,
                },
            ]
        }
        Tool::Hammer => {
            vec![
                Action::Fence {
                    width: 1,
                    height: 1,
                },
                Action::Turret {
                    width: TURRET_SIZE_X,
                    height: TURRET_SIZE_Y,
                },
            ]
        }
        _ => Vec::new(),
    }
}
