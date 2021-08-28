use crate::Text;
use bevy::prelude::*;

/// Vidas do jogador
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Lives(pub u8);
/// O padrão são 3 vidas
impl Default for Lives {
    fn default() -> Self {
        Self(3)
    }
}

impl Lives {
    pub fn update_lives(
        lives: Res<State<Lives>>,
        mut counters: Query<&mut TextureAtlasSprite, With<LivesCounter>>,
    ) {
        for mut counter in counters.iter_mut() {
            let index = Text::from_char(char::from(lives.current().0 + 48)).0;
            counter.index = index as u32;
        }
    }
}

/// Estrutura vazia representando um evento de dano
pub struct DamageEvent;

/// Tag para marcar o contador que devemos atualizar
pub struct LivesCounter;
