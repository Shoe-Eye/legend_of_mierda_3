use bevy::prelude::*;

pub mod item;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((item::ItemPlugin));
    }
}
