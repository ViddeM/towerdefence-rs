use std::borrow::Borrow;

use bevy::prelude::*;

#[derive(Component)]
pub struct Waves {
    active_wave: Option<Wave>,
    previous_waves: Vec<Wave>,
    time_since_last_wave_spawned: f32,
}

pub struct Wave {
    pub num_enemies: usize,
    pub spawned_enemies: usize,
}

impl Waves {
    pub fn next_wave(&mut self) {
        if self.active_wave.is_some() {
            warn!("A wave is already active!");
            return;
        }

        let num_enemies = self.previous_waves.len() + 1;
        self.active_wave = Some(Wave {
            num_enemies: num_enemies,
            spawned_enemies: 0,
        });

        let wave_num = num_enemies;
        info!("Started wave {wave_num}");
    }

    pub fn end_wave(&mut self) {
        let Some(wave) = self.active_wave.borrow() else {
            warn!("Tried to end wave when none is active?");
            return;
        };

        if wave.num_enemies > wave.spawned_enemies {
            warn!(
                "Tried to end wave but not all enemies have been spawned ({}/{} spawned)",
                wave.spawned_enemies, wave.num_enemies
            );
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

pub fn waves_setup(mut commands: Commands) {
    commands.spawn(Waves {
        active_wave: None,
        previous_waves: vec![],
        time_since_last_wave_spawned: 0.,
    });
}

pub fn wave_spawner() {}
