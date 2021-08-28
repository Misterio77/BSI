use crate::{Position, Size, Sprites};
use bevy::prelude::*;

/// Essa estrutura representa uma entidade de texto, um caractere
#[derive(Clone, Copy)]
pub struct Text(pub u8);
impl Text {
    /// Criar a entidade no mapa
    pub fn spawn(
        &self,
        position: Position,
        commands: &mut Commands,
        sprites: &Res<Sprites>,
    ) -> Entity {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.text.clone(),
                global_transform: GlobalTransform::from_xyz(0.0, 0.0, 2.0),
                sprite: TextureAtlasSprite::new(self.0 as u32),
                ..Default::default()
            })
            .insert(position)
            .insert(Size(1.0))
            .insert(*self)
            .id()
    }
    /// Facilita a criação de textos maiores
    pub fn spawn_string(
        text: &str,
        position: Position,
        commands: &mut Commands,
        sprites: &Res<Sprites>,
    ) -> Vec<Entity> {
        let mut entities = Vec::new();
        for (i, c) in text.chars().into_iter().enumerate() {
            entities.push(Text::from_char(c).spawn(
                position + Position::xy(i as i32, 0),
                commands,
                sprites,
            ));
        }
        entities
    }
    /// Mapa dos caracteres para os índices na sprite sheet
    pub fn from_char(c: char) -> Text {
        let index = match c {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            'I' => 8,
            'J' => 9,
            'K' => 10,
            'L' => 11,
            'M' => 12,
            'N' => 13,
            'O' => 14,
            'P' => 15,
            'Q' => 16,
            'R' => 17,
            'S' => 18,
            'T' => 19,
            'U' => 20,
            'V' => 21,
            'W' => 22,
            'X' => 23,
            'Y' => 24,
            'Z' => 25,
            '0' => 26,
            '1' => 27,
            '2' => 28,
            '3' => 29,
            '4' => 30,
            '5' => 31,
            '6' => 32,
            '7' => 33,
            '8' => 34,
            '9' => 35,
            '-' => 36,
            ':' => 37,
            _ => 38,
        };
        Text(index)
    }
}
