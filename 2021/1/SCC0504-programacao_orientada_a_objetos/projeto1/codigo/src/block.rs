use crate::{Movement, MovementTimer, Position, Size, Sprites};
use bevy::prelude::*;

/// Tipos de bloco
#[derive(Copy, Clone)]
pub enum Block {
    DiamondGreen,
    DiamondRed,
    SolidGreen,
    SolidRed,
}

impl Block {
    /// Adicionar bloco no mapa
    pub fn spawn(
        &self,
        position: Position,
        commands: &mut Commands,
        sprites: &Res<Sprites>,
    ) -> Entity {
        let sprite = match self {
            Block::DiamondGreen => sprites.blocks.diamond_green.clone(),
            Block::DiamondRed => sprites.blocks.diamond_red.clone(),
            Block::SolidGreen => sprites.blocks.solid_green.clone(),
            Block::SolidRed => sprites.blocks.solid_red.clone(),
        };
        commands
            .spawn_bundle(SpriteBundle {
                material: sprite,
                global_transform: GlobalTransform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            })
            .insert(MovementTimer(Timer::from_seconds(0.3, true)))
            .insert(Movement(None))
            .insert(position)
            .insert(Size(1.0))
            .insert(*self)
            .id()
    }
    /// Verifica se um bloco é móvel
    pub fn is_movable(&self) -> bool {
        matches![self, Block::DiamondGreen | Block::DiamondRed]
    }
    /// Verifica se um bloco é quebrável
    pub fn is_breakable(&self) -> bool {
        matches![self, Block::DiamondGreen | Block::SolidGreen]
    }
}
