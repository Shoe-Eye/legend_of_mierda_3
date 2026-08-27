use bevy::prelude::*;
use lom_game::GameState;

#[derive(Component, Clone, Copy)]
pub struct FenceTile {
    pub x: u32,
    pub y: u32,
}

pub fn get_sprite_index(tile: FenceTile, all_tiles: Vec<FenceTile>) -> usize {
    let exists_left = all_tiles
        .iter()
        .filter(|t| t.x == (tile.x - 1) && t.y == tile.y)
        .count()
        > 0;

    let exists_right = all_tiles
        .iter()
        .filter(|t| t.x == (tile.x + 1) && t.y == tile.y)
        .count()
        > 0;

    let exists_top = all_tiles
        .iter()
        .filter(|t| t.x == tile.x && t.y == (tile.y + 1))
        .count()
        > 0;

    let exists_bottom = all_tiles
        .iter()
        .filter(|t| t.x == tile.x && t.y == (tile.y - 1))
        .count()
        > 0;

    if exists_left && exists_right && exists_top && exists_bottom {
        return 7;
    }

    if !exists_left && !exists_right && !exists_top && !exists_bottom {
        return 3;
    }

    if !exists_left && exists_right && exists_bottom {
        return 0;
    }

    if !exists_left && exists_right && !exists_bottom {
        return 6;
    }

    // ---

    if exists_left && !exists_right && exists_bottom {
        return 2;
    }

    if exists_left && !exists_right && !exists_bottom {
        return 8;
    }

    if exists_bottom && exists_top {
        return 3;
    }

    if exists_left && exists_right {
        return 7;
    }

    if !exists_left && !exists_right && exists_bottom {
        return 3;
    }

    if !exists_left && !exists_right && exists_top {
        return 3;
    }

    return 0;
}

pub fn adjust_fence_sprites(
    mut q_fences: ParamSet<(
        Query<(&mut Sprite, &FenceTile)>,
        Query<(&mut Sprite, &FenceTile), Added<FenceTile>>,
    )>,
) {
    let is_fence_built = q_fences.p1().iter().count() > 0;
    if !is_fence_built {
        return;
    }

    let fences: Vec<FenceTile> = q_fences
        .p0()
        .iter()
        .map(|(_, fence)| fence.clone())
        .collect();

    for (mut sprite, fence) in q_fences.p0().iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = get_sprite_index(fence.clone(), fences.clone());
        }
    }
}

pub struct FencePlugin;

impl Plugin for FencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (adjust_fence_sprites,).run_if(in_state(GameState::GamePlay)),
        );
    }
}
