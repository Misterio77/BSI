use crate::{Position, Size, Sprites};
use bevy::prelude::*;

/// Itens das fases
#[derive(Clone, Copy)]
pub enum Item {
    Candle,
    Cherry,
    Flashlight,
    Grape,
    Lemon,
    LightBulb,
    Strawberry,
    Sun,
}

impl Item {
    /// Criar item no mapa
    pub fn spawn(
        &self,
        position: Position,
        commands: &mut Commands,
        sprites: &Res<Sprites>,
    ) -> Entity {
        let sprite = match self {
            Item::Candle => sprites.items.candle.clone(),
            Item::Cherry => sprites.items.cherry.clone(),
            Item::Flashlight => sprites.items.flashlight.clone(),
            Item::Grape => sprites.items.grape.clone(),
            Item::Lemon => sprites.items.lemon.clone(),
            Item::LightBulb => sprites.items.lightbulb.clone(),
            Item::Strawberry => sprites.items.strawberry.clone(),
            Item::Sun => sprites.items.sun.clone(),
        };
        commands
            .spawn_bundle(SpriteBundle {
                material: sprite,
                global_transform: GlobalTransform::from_xyz(0.0, 0.0, 2.0),
                ..Default::default()
            })
            .insert(position)
            .insert(Size(1.0))
            .insert(*self)
            .id()
    }
}
