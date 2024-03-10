use std::ops::Div;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::utils::dijkstra::{Pathfind, Vertex};

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TileType {
    Empty,
    Floor,
    Start,
    End,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tile {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub tile_type: TileType,
}

impl Vertex for Tile {
    fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Component)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Map {
    pub fn has_path(&self) -> bool {
        let (start_x, start_y) = self.start;
        let start = self.get_tile_at(start_x, start_y);

        let (end_x, end_y) = self.end;
        let end = self.get_tile_at(end_x, end_y);

        self.find_path(start, end).is_some()
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x].clone()
    }
}

impl Pathfind<Tile> for Map {
    fn get_all_verticies(&self) -> Vec<Tile> {
        self.tiles
            .iter()
            .flatten()
            .filter(|t| t.tile_type != TileType::Empty)
            .cloned()
            .collect()
    }

    /// Assumes the tile is a valid tile (within the grid)
    fn get_neighbours(&self, vertex: &Tile) -> Vec<Tile> {
        let mut neighbours = vec![];
        if vertex.x > 0 {
            neighbours.push(self.get_tile_at(vertex.x - 1, vertex.y));
        }
        if vertex.y > 0 {
            neighbours.push(self.get_tile_at(vertex.x, vertex.y - 1));
        }
        if vertex.x < WIDTH - 1 {
            neighbours.push(self.get_tile_at(vertex.x + 1, vertex.y));
        }
        if vertex.y < HEIGHT - 1 {
            neighbours.push(self.get_tile_at(vertex.x, vertex.y + 1));
        }
        neighbours
            .into_iter()
            .filter(|t| t.tile_type != TileType::Empty)
            .collect()
    }
}

#[derive(Component)]
pub struct MapVisuals {
    pub tile_mesh: Mesh2dHandle,
    pub tile_empty_material: Handle<ColorMaterial>,
    pub tile_floor_material: Handle<ColorMaterial>,
    pub start_tile_material: Handle<ColorMaterial>,
    pub end_tile_material: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct TileVisuals {
    pub x: usize,
    pub y: usize,
}

pub fn map_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Setting up map");

    let empty_material = materials.add(Color::rgb(0.2, 0.2, 0.2));
    let floor_material = materials.add(Color::rgb(0.6, 0.6, 0.6));
    let start_material = materials.add(Color::rgb(0.2, 0.6, 0.2));
    let end_material = materials.add(Color::rgb(0.6, 0.2, 0.2));

    let tile_mesh = Mesh2dHandle(meshes.add(Cuboid::new(1.0, 1.0, 0.1)));

    let start_pos @ (start_x, start_y) = (0, 0);
    let end_pos @ (end_x, end_y) = (WIDTH - 1, HEIGHT - 1);

    let mut map_tiles = Vec::with_capacity(HEIGHT);
    for y in 0..HEIGHT {
        let mut tiles = Vec::with_capacity(WIDTH);
        for x in 0..WIDTH {
            let (tile_type, mat) = if x == start_x && y == start_y {
                (TileType::Start, start_material.clone())
            } else if x == end_x && y == end_y {
                (TileType::End, end_material.clone())
            } else {
                (TileType::Empty, empty_material.clone())
            };

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: tile_mesh.clone(),
                    material: mat,
                    transform: Transform::from_xyz(x as f32, y as f32, 0.0)
                        .with_scale(Vec3::new(0.9, 0.9, 0.2)),
                    ..default()
                },
                TileVisuals { x, y },
            ));

            let tile_id = y * HEIGHT + x;
            let tile = Tile {
                tile_type,
                id: tile_id,
                x,
                y,
            };

            tiles.push(tile);
        }
        map_tiles.push(tiles);
    }

    commands.spawn(Map {
        width: WIDTH,
        height: HEIGHT,
        tiles: map_tiles,
        start: start_pos,
        end: end_pos,
    });

    commands.spawn(MapVisuals {
        tile_mesh,
        tile_empty_material: empty_material,
        tile_floor_material: floor_material,
        start_tile_material: start_material,
        end_tile_material: end_material,
    });

    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.02;
    camera.transform.translation.x = (WIDTH as f32).div(2.);
    camera.transform.translation.y = (HEIGHT as f32).div(2.);
    commands.spawn(camera);
}
