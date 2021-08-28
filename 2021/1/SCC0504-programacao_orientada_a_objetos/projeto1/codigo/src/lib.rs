#![allow(clippy::type_complexity)]
/// Módulos do programa
pub mod levels;
pub use levels::{CurrentLevel, Level1, Level2, Level3, Level4, Splash};
pub mod sprites;
pub use sprites::Sprites;
pub mod gamescreen;
pub use gamescreen::GameScreen;
pub mod hero;
pub use hero::Hero;
pub mod enemy;
pub use enemy::Enemy;
pub mod block;
pub use block::Block;
pub mod item;
pub use item::Item;
pub mod tile;
pub use tile::Tile;
pub mod lives;
pub use lives::{DamageEvent, Lives, LivesCounter};
pub mod text;
pub use text::Text;

use bevy::prelude::Timer;
use std::ops::{Deref, DerefMut};

/// Tamanho das sprites
#[derive(Debug, Copy, Clone)]
pub struct Size(pub f32);

/// Possíveis direções na grid
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

impl Direction {
    /// Devolve o índice daquela direção na sheet de animação
    pub fn index(&self) -> u32 {
        match self {
            Direction::Left => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Up => 3,
        }
    }
    /// Verifica se duas direções são opostas
    pub fn is_opposite(&self, other: &Self) -> bool {
        matches![
            (self, other),
            (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
                | (Direction::Down, Direction::Up)
                | (Direction::Up, Direction::Down)
        ]
    }
    /// Transforma em uma coordenada (Position)
    pub fn as_position(&self) -> Position {
        match self {
            Direction::Down => Position::xy(0, -1),
            Direction::Up => Position::xy(0, 1),
            Direction::Left => Position::xy(-1, 0),
            Direction::Right => Position::xy(1, 0),
        }
    }
}

/// Construtor padrão
impl Default for Direction {
    fn default() -> Direction {
        Direction::Down
    }
}

/// Posição na grid
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// Construir a partir de x e y
    pub fn xy(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    /// Construir partindo do meio da grid
    pub fn middle() -> Self {
        Self {
            x: GameScreen::width() as i32 / 2,
            y: GameScreen::height() as i32 / 2,
        }
    }
    /// Transformar em coordenadas absolutas da grid geral
    pub fn as_absolute(&self) -> (i32, i32) {
        (
            self.x + GameScreen::margin_left(),
            self.y + GameScreen::margin_down(),
        )
    }
    /// Verificar se está dentro dos limites da grid jogável
    pub fn inside_boundaries(&self) -> bool {
        self.y >= 0
            && self.y < GameScreen::height() as i32
            && self.x >= 0
            && self.x < GameScreen::width() as i32
    }
}

/// Soma duas posições
impl std::ops::Add for Position {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Movimento de um personagem ou bloco
#[derive(Debug, Copy, Clone)]
pub struct Movement(pub Option<Direction>);

/// O padrão é não estar em movimento
impl Default for Movement {
    fn default() -> Self {
        Self(None)
    }
}
/// É desreferenciável em Direction
impl std::ops::Deref for Movement {
    type Target = Option<Direction>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Movement {
    fn deref_mut(&mut self) -> &mut Option<Direction> {
        &mut self.0
    }
}

/// Wrapper para timer de animação
pub struct AnimationTimer(Timer);
impl Deref for AnimationTimer {
    type Target = Timer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AnimationTimer {
    fn deref_mut(&mut self) -> &mut Timer {
        &mut self.0
    }
}
/// Wrapper para timer de movimento
pub struct MovementTimer(pub Timer);
impl Deref for MovementTimer {
    type Target = Timer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for MovementTimer {
    fn deref_mut(&mut self) -> &mut Timer {
        &mut self.0
    }
}
/// Wrapper para timer de movimento aleatório dos inimigos
pub struct WanderTimer(pub Timer);
impl Deref for WanderTimer {
    type Target = Timer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for WanderTimer {
    fn deref_mut(&mut self) -> &mut Timer {
        &mut self.0
    }
}
