use crate::{
    AnimationTimer, CurrentLevel, Direction, Movement, MovementTimer, Position, Size, Tile,
};
use bevy::prelude::*;

const LEVEL_HEIGHT: u32 = 11;
const LEVEL_WIDTH: u32 = 11;
const MARGIN_RIGHT: i32 = 7;
const MARGIN_LEFT: i32 = 1;
const MARGIN_UP: i32 = 4;
const MARGIN_DOWN: i32 = 4;

pub struct GameScreen;

impl GameScreen {
    /// Altura da parte jogável
    pub fn height() -> u32 {
        LEVEL_HEIGHT
    }
    /// Largura da parte jogável
    pub fn width() -> u32 {
        LEVEL_WIDTH
    }
    /// Margem direita
    pub fn margin_right() -> i32 {
        MARGIN_RIGHT
    }
    /// Margem esquerda
    pub fn margin_left() -> i32 {
        MARGIN_LEFT
    }
    /// Margem superior
    pub fn margin_up() -> i32 {
        MARGIN_UP
    }
    /// Margem inferior
    pub fn margin_down() -> i32 {
        MARGIN_DOWN
    }
    /// Altura incluindo margem
    pub fn full_height() -> i32 {
        Self::height() as i32 + Self::margin_up() + Self::margin_down()
    }
    /// Largura incluindo margem
    pub fn full_width() -> i32 {
        Self::width() as i32 + Self::margin_right() + Self::margin_left()
    }
    /// Trocar frames de entidades animadas
    pub fn animations(
        atlases: Res<Assets<TextureAtlas>>,
        time: Res<Time>,
        mut query: Query<(
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &Direction,
        )>,
    ) {
        for (mut timer, mut sprite, atlas_handle, direction) in query.iter_mut() {
            timer.0.tick(time.delta());
            if timer.0.finished() {
                let atlas = atlases.get(atlas_handle).unwrap();
                let index = ((sprite.index as usize + 1) % (atlas.textures.len() / 4)) as u32;
                let offset = direction.index() * (atlas.textures.len() / 4) as u32;
                sprite.index = index + offset;
            }
        }
    }
    /// Atualizar escala de cada entidade
    pub fn scaling(
        windows: Res<Windows>,
        mut q: Query<(&mut GlobalTransform, &Size), Without<CurrentLevel>>,
    ) {
        let window = windows.get_primary().unwrap();
        for (mut transform, size) in q.iter_mut() {
            transform.scale = Vec3::new(
                size.0 * (window.width() as i32 / GameScreen::full_width()) as f32 / 16.0,
                size.0 * (window.height() as i32 / GameScreen::full_height()) as f32 / 16.0,
                1.0,
            );
        }
    }
    /// Atualizar posição de cada entidade
    pub fn positions(
        windows: Res<Windows>,
        mut q: Query<(&Position, &mut GlobalTransform), Without<CurrentLevel>>,
    ) {
        fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
            let tile_size = bound_window / bound_game;
            pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
        }
        let window = windows.get_primary().unwrap();
        for (pos, mut transform) in q.iter_mut() {
            let pos = pos.as_absolute();
            transform.translation = Vec3::new(
                convert(
                    pos.0 as f32,
                    window.width() as f32,
                    GameScreen::full_width() as f32,
                ),
                convert(
                    pos.1 as f32,
                    window.height() as f32,
                    GameScreen::full_height() as f32,
                ),
                transform.translation.z,
            );
        }
    }
    /// Fazer movimentos de entidades que estão com um pendente
    pub fn movements(
        time: Res<Time>,
        mut entities: Query<(&mut Movement, &mut Position, &mut MovementTimer)>,
        mut tiles: Query<(&Position, &Tile), Without<Movement>>,
    ) {
        for (mut movement, mut position, mut timer) in entities.iter_mut() {
            let delta = time.delta();
            timer.tick(delta);

            let tile = tiles
                .iter_mut()
                .find(|tile| *tile.0 == *position && matches![tile.1, Tile::Arrow(_)]);
            if let Some(tile) = tile {
                if let Tile::Arrow(direction) = tile.1 {
                    if let Some(movement_direction) = **movement {
                        if movement_direction.is_opposite(direction) {
                            *movement = Movement(None);
                        }
                    } else {
                        *movement = Movement(Some(*direction));
                        timer.unpause();
                        timer.tick(delta * 7);
                    }
                }
            }
            if timer.just_finished() {
                match movement.0 {
                    Some(Direction::Left) => position.x -= 1,
                    Some(Direction::Right) => position.x += 1,
                    Some(Direction::Down) => position.y -= 1,
                    Some(Direction::Up) => position.y += 1,
                    None => {}
                }
                movement.0 = None;
                timer.pause();
                timer.reset();
            }
        }
    }
}
