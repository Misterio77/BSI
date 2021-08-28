use crate::{Direction, Position, Size, Sprites};
use bevy::prelude::*;
use std::f32::consts::PI;

/// Reprenta um fundo, onde o personagem passa por cima
#[derive(Copy, Clone, Debug)]
pub enum Tile {
    /// Flecha
    Arrow(Direction),
    /// Borda do mapa
    Border(Direction),
    /// Quina do mapa
    Corner(Direction),
    /// Fundo do nível 1
    Level1,
    /// Fundo do nível 2
    Level2,
    /// Fundo do nível 3
    Level3,
    /// Fundo do nível 4
    Level4,
}

impl Tile {
    /// Cria um tile no mapa
    pub fn spawn(
        &self,
        position: Position,
        commands: &mut Commands,
        sprites: &Res<Sprites>,
    ) -> Entity {
        let sprite = match self {
            Tile::Arrow(_) => sprites.tiles.arrow.clone(),
            Tile::Level1 => sprites.tiles.level1.clone(),
            Tile::Level2 => sprites.tiles.level2.clone(),
            Tile::Level3 => sprites.tiles.level3.clone(),
            Tile::Level4 => sprites.tiles.level4.clone(),
            Tile::Border(_) => sprites.tiles.border.clone(),
            Tile::Corner(_) => sprites.tiles.corner.clone(),
        };

        // Caso seja flecha, quina ou borda, adicionar rotação
        let rotation = if let Tile::Arrow(direction) = self {
            match direction {
                Direction::Down => Quat::from_rotation_z(-PI / 2.0),
                Direction::Up => Quat::from_rotation_z(PI / 2.0),
                Direction::Left => Quat::from_rotation_z(PI),
                Direction::Right => Quat::from_rotation_z(0.0),
            }
        } else if let Tile::Corner(direction) = self {
            match direction {
                Direction::Down => Quat::from_rotation_z(-PI / 2.0),
                Direction::Up => Quat::from_rotation_z(PI / 2.0),
                Direction::Left => Quat::from_rotation_z(PI),
                Direction::Right => Quat::from_rotation_z(0.0),
            }
        } else if let Tile::Border(direction) = self {
            match direction {
                Direction::Down => Quat::from_rotation_z(-PI / 2.0),
                Direction::Up => Quat::from_rotation_z(PI / 2.0),
                Direction::Left => Quat::from_rotation_z(PI),
                Direction::Right => Quat::from_rotation_z(0.0),
            }
        } else {
            Quat::from_rotation_z(0.0)
        };

        commands
            .spawn_bundle(SpriteBundle {
                material: sprite,
                global_transform: GlobalTransform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    rotation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(position)
            .insert(Size(1.0))
            .insert(*self)
            .id()
    }
}
