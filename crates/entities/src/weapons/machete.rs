use crate::controls::ControlEvent;
use crate::player::Player;
use bevy::prelude::*;
use lom_game::GameState;
use rand::rngs::ThreadRng;
use rand::RngExt;

// note to self: attack happens every 1.3 seconds but there is delay
// for attack 0.3 secodns so i hinda hack this around

// ----------
// Components
// ----------

#[derive(Component, Clone, Copy, Default)]
pub struct Machete {}

#[derive(Default, Component, Clone)]
pub struct MacheteIndicatorMaterial(pub Handle<ColorMaterial>);

// -------
// Bundles
// -------

#[derive(Clone, Default, Bundle)]
pub struct MacheteIndictorBundle {
    pub machete_indicator: Machete,
    pub machete_indicator_material: MacheteIndicatorMaterial,
    pub timer_activation: MacheteTimer,
}

// ---------
// Resources
// ---------

#[derive(Resource, Default, Clone, Component)]
pub struct MacheteTimer(pub Timer);

// -------
// Systems
// -------

fn inject_machete_indicator(
    mut commands: Commands,
    q_players: Query<(Entity, &ChildOf, &Transform, &Player)>,
    mut q_machate_indicator: ParamSet<(Query<(&mut Transform, &Machete), Without<Player>>,)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut machete_timer: ResMut<MacheteTimer>,
) {
    for (player_entity, _, player_transform, _) in q_players.iter() {
        if q_machate_indicator.p0().iter().count() > 0 {
            continue;
        }

        let mut rng = ThreadRng::default();
        let random_offset_x = rng.random_range(-50.0..50.0);
        let random_offset_y = rng.random_range(-50.0..50.0);
        let offset = Vec3::new(random_offset_x, random_offset_y, 0.0);

        commands.entity(player_entity).with_children(|parent| {
            let mesh_handle = meshes.add(Circle::new(80.));
            let material_handle =
                materials.add(ColorMaterial::from(Color::srgba(0.5, 0.0, 0.5, 0.5)));
            parent.spawn((
                MacheteIndictorBundle {
                    machete_indicator: Machete {},
                    machete_indicator_material: MacheteIndicatorMaterial(material_handle),
                    timer_activation: machete_timer.clone(),
                },
                ZIndex(103),
                Name::new("machete radius indicator"),
                Mesh2d(mesh_handle),
                Transform::from_translation(player_transform.translation + offset), //
            ));
        });
    }
}

// -------
// Systems
// -------

pub fn handle_machete_attack(
    time: Res<Time>,
    mut q_machete: Query<(Entity, &Transform, &mut MacheteTimer), With<Machete>>,
    mut ev_control: MessageWriter<ControlEvent>,
) {
    for (_, _, mut machete_timer) in q_machete.iter_mut() {
        machete_timer.0.tick(time.delta());

        if machete_timer.0.just_finished() {
            ev_control.write(ControlEvent {
                attack: true,
                ..Default::default()
            });
        }
    }
}

fn animate_machete_indicator(
    mut q_machete: Query<(Entity, &mut MacheteIndicatorMaterial, &mut MacheteTimer), With<Machete>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (_, mut indicator_mat, timer) in q_machete.iter_mut() {
        let elapsed = timer.0.elapsed_secs();
        let mut percentage = (1.0 - elapsed) / (1.0 - 0.3);
        if elapsed < 0.3 {
            percentage = 0.0;
        }

        indicator_mat.0 =
            materials.add(ColorMaterial::from(Color::srgba(0.5, 0.0, 0.5, percentage)));
    }
}

// ------
// Plugin
// ------

// I--------I--------I
//   x        x

pub struct MachetePlugin;

impl Plugin for MachetePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MacheteTimer>().add_systems(
            Update,
            (
                inject_machete_indicator.run_if(in_state(GameState::GamePlay)),
                handle_machete_attack.run_if(in_state(GameState::GamePlay)),
                animate_machete_indicator.run_if(in_state(GameState::GamePlay)),
            ),
        );
    }
}
