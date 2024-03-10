use std::borrow::{Borrow, BorrowMut};

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{map::Map, utils::dijkstra::Pathfind};

const TIME_BETWEEN_WAVES: f32 = 3.0;

#[derive(Component)]
pub struct Waves {
    active_wave: Option<Wave>,
    previous_waves: Vec<Wave>,
    time_since_last_wave_spawned: f32,
}

pub struct Wave {
    pub num_enemies: usize,
    pub spawned_enemies: usize,
    pub finished_enemies: usize,
}

impl Waves {
    pub fn next_wave(&mut self) {
        if self.active_wave.is_some() {
            warn!("A wave is already active!");
            return;
        }

        let num_enemies = self.previous_waves.len() + 1;
        self.active_wave = Some(Wave {
            num_enemies,
            spawned_enemies: 0,
            finished_enemies: 0,
        });

        let wave_num = num_enemies;
        info!("Started wave {wave_num}");
    }

    pub fn enemy_finished(&mut self) {
        let Some(wave) = self.active_wave.borrow_mut() else {
            warn!("Enemy reached the goal but no wave is active!");
            return;
        };

        wave.finished_enemies += 1;
        if wave.finished_enemies >= wave.num_enemies {
            self.end_wave()
        }
    }

    pub fn end_wave(&mut self) {
        let Some(wave) = self.active_wave.borrow() else {
            warn!("Tried to end wave when none is active?");
            return;
        };

        if wave.spawned_enemies < wave.num_enemies {
            warn!(
                "Tried to end wave but not all enemies have been spawned ({}/{} spawned)",
                wave.spawned_enemies, wave.num_enemies
            );
            return;
        }

        if wave.finished_enemies < wave.num_enemies {
            warn!(
                "Tried to end wave but not all enemies have reached the goal ({}/{} have reached goal)", 
                wave.finished_enemies, wave.num_enemies);
            return;
        }

        let wave = self
            .active_wave
            .take()
            .expect("The wave no longer exists but did a moment ago?");

        self.previous_waves.push(wave);
    }

    pub fn is_active(&self) -> bool {
        return self.active_wave.is_some();
    }
}

#[derive(Component)]
pub struct EnemyVisuals {
    enemy_mesh: Mesh2dHandle,
    enemy_mat: Handle<ColorMaterial>,
}

pub fn waves_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Waves {
        active_wave: None,
        previous_waves: vec![],
        time_since_last_wave_spawned: 0.,
    });

    let enemy_mesh = Mesh2dHandle(meshes.add(Torus::new(0.4, 0.8)));
    let enemy_mat = materials.add(Color::rgb(0.8, 0.2, 0.2));

    commands.spawn(EnemyVisuals {
        enemy_mesh,
        enemy_mat,
    });
}

pub fn wave_spawner(
    mut commands: Commands,
    map_query: Query<&Map>,
    mut waves_query: Query<&mut Waves>,
    enemy_visuals_query: Query<&EnemyVisuals>,
    time: Res<Time>,
) {
    let mut waves = waves_query.single_mut();
    if waves.time_since_last_wave_spawned < TIME_BETWEEN_WAVES {
        waves.time_since_last_wave_spawned += time.delta_seconds();
        return;
    }
    waves.time_since_last_wave_spawned = 0.;

    let Some(active_wave) = waves.active_wave.borrow_mut() else {
        return;
    };

    if active_wave.spawned_enemies == active_wave.num_enemies {
        return;
    }

    info!("Spawning new enemy");

    active_wave.spawned_enemies += 1;
    let enemy_visuals = enemy_visuals_query.single();
    let map = map_query.single();
    let (start_x, start_y) = map.start;
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: enemy_visuals.enemy_mesh.clone(),
            material: enemy_visuals.enemy_mat.clone(),
            transform: Transform::from_xyz(start_x as f32, start_y as f32, 1.0)
                .with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    (90.0 as f32).to_radians(),
                    0.,
                    0.,
                ))
                .with_scale(Vec3::new(0.5, 0.2, 0.5)),
            ..default()
        },
        Enemy {
            target_x: start_x,
            target_y: start_y,
            current_x: start_x,
            current_y: start_y,
            progress: 0.0,
        },
    ));
}

#[derive(Component)]
pub struct Enemy {
    pub target_x: usize,
    pub target_y: usize,
    pub current_x: usize,
    pub current_y: usize,
    pub progress: f32,
}

pub fn move_enemies(
    mut commands: Commands,
    map_query: Query<&Map>,
    mut waves_query: Query<&mut Waves>,
    mut enemies_query: Query<(Entity, &mut Enemy, &mut Transform)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    let mut waves = waves_query.single_mut();

    let map = map_query.single();
    for (entity, mut enemy, mut transform) in enemies_query.iter_mut() {
        let progress = enemy.progress + delta;

        if progress >= 4.0 {
            enemy.current_x = enemy.target_x;
            enemy.current_y = enemy.target_y;

            if map.end == (enemy.current_x, enemy.current_y) {
                // Despawn the entity
                waves.enemy_finished();
                commands.entity(entity).despawn();
                continue;
            }

            let path = map
                .find_path(
                    map.get_tile_at(enemy.current_x, enemy.current_y),
                    map.get_tile_at(map.end.0, map.end.1),
                )
                .expect("Failed to find path for enemy!");
            let next_tile = path.first().expect("No next tile in path?");
            enemy.target_x = next_tile.x;
            enemy.target_y = next_tile.y;
            enemy.progress = 0.0;
        } else {
            enemy.progress = progress;
        }
        info!("PROGRESS: {progress:?}, DELTA: {delta}");

        transform.translation.x =
            (enemy.current_x as f32).lerp(enemy.target_x as f32, enemy.progress);

        transform.translation.y =
            (enemy.current_y as f32).lerp(enemy.target_y as f32, enemy.progress);
    }
}
