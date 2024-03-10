use bevy::prelude::*;

use crate::map::{Map, MapVisuals, TileType, TileVisuals};

pub fn tile_build_system(
    mut map_query: Query<&mut Map>,
    mut tiles_query: Query<(&TileVisuals, &mut Handle<ColorMaterial>)>,
    map_visuals_query: Query<&MapVisuals>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let mut map = map_query.single_mut();
    let map_visuals = map_visuals_query.single();
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_screen_pos) = windows.single().cursor_position() else {
        return;
    };
    let Some(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_screen_pos)
    else {
        return;
    };

    let tile_x = cursor_world_pos.x.round() as i64;
    let tile_y = cursor_world_pos.y.round() as i64;

    if tile_x < 0 || tile_y < 0 {
        return;
    }

    let tile_x = tile_x as usize;
    let tile_y = tile_y as usize;

    if tile_x >= map.width || tile_y >= map.height {
        return;
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        let tile = map.tiles.get_mut(tile_y).unwrap().get_mut(tile_x).unwrap();
        if tile.tile_type != TileType::Empty {
            return;
        }

        info!("\n\tScreen pos {cursor_screen_pos:?}\n\t world_pos: {cursor_world_pos:?}\n\t ({tile_x}, {tile_y}");

        tiles_query
            .iter_mut()
            .filter(|(tile, _)| tile.x == tile_x && tile.y == tile_y)
            .for_each(|(_, mut mat)| {
                *mat = map_visuals.tile_floor_material.clone_weak();
            });
        tile.tile_type = TileType::Floor;
    }
}
