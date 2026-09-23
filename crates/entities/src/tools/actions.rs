use bevy::prelude::*;
use core::fmt;

use crate::{
    level::{
        plant::PlantType,
        turret::{TURRET_SIZE_X, TURRET_SIZE_Y},
    },
    tools::Tool,
};

#[derive(PartialEq, Debug, Clone, Copy, Component)]
pub enum Action {
    Dig {
        width: u32,
        height: u32,
    },
    Trail {
        width: u32,
        height: u32,
    },
    Foundation {
        width: u32,
        height: u32,
    },
    Fence {
        width: u32,
        height: u32,
    },
    Turret {
        width: u32,
        height: u32,
    },
    Plant {
        plant_type: PlantType,
        width: u32,
        height: u32,
    },
    Harvest,
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
            Action::Trail { width, height } => write!(f, "Trail {}x{}", width, height),
            Action::Plant {
                plant_type,
                width,
                height,
            } => {
                write!(f, "Plant {:?} {}x{}", plant_type, width, height)
            }
            Action::Harvest => write!(f, "Harvest"),
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
                Action::Trail {
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
        Tool::WateringCan => {
            vec![Action::Plant {
                plant_type: PlantType::Watermelon,
                width: 1,
                height: 1,
            }]
        }
        Tool::NoTool => {
            vec![Action::Harvest]
        }
        _ => Vec::new(),
    }
}
