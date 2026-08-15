use bevy::prelude::*;

pub mod biboran;
pub mod item;
pub mod pizza;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((item::ItemPlugin, pizza::PizzaPlugin, biboran::BiboranPlugin));
    }
}
