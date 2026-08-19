use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::characters::enemy::Enemy;
use crate::items::item::Item;

pub fn hide_dummy_entities(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(Entity, &mut Visibility, &Enemy)>,
        Query<(Entity, &mut Visibility, &Item)>,
    )>,
) {
    for (entity, mut visibility, enemy) in set.p0().iter_mut() {
        if enemy.is_dummy {
            *visibility = Visibility::Hidden;
            commands.entity(entity).remove::<Collider>();
        }
    }

    for (entity, mut visibility, enemy) in set.p1().iter_mut() {
        if enemy.is_dummy {
            *visibility = Visibility::Hidden;
            commands.entity(entity).remove::<Collider>();
        }
    }
}

pub fn fix_missing_ldtk_entities() {}
