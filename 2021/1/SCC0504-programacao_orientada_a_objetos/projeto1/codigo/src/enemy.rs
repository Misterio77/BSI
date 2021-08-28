use crate::{
    AnimationTimer, Block, DamageEvent, Direction, Hero, Item, Movement, MovementTimer, Position,
    Size, Sprites, Tile,
};
use bevy::prelude::*;
use rand::seq::SliceRandom;

/// Tipos de inimigo (cores)
#[derive(Clone, Copy)]
pub enum Enemy {
    Yellow,
    Magenta,
    Blue,
    Green,
}

impl Enemy {
    /// Adicionar inimigo no mapa
    pub fn spawn(
        &self,
        position: Position,
        commands: &mut Commands,
        sprites: &Res<Sprites>,
    ) -> Entity {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.chars.robot.clone(),
                sprite: TextureAtlasSprite {
                    color: self.color(),
                    ..Default::default()
                },
                global_transform: GlobalTransform::from_xyz(0.0, 0.0, 2.0),
                ..Default::default()
            })
            .insert(AnimationTimer(Timer::from_seconds(0.2, true)))
            .insert(MovementTimer(Timer::from_seconds(0.6, true)))
            .insert(Direction::Down)
            .insert(Movement(None))
            .insert(position)
            .insert(Size(1.0))
            .insert(*self)
            .id()
    }
    /// Retorna cor em rgb
    pub fn color(&self) -> Color {
        match self {
            Enemy::Yellow => Color::rgb(0.87, 0.859, 0.129),
            Enemy::Magenta => Color::rgb(0.87, 0.286, 0.710),
            Enemy::Blue => Color::rgb(0.29, 0.427, 1.0),
            Enemy::Green => Color::rgb(0.42, 1.0, 0.42),
        }
    }
    /// Verificar se o inimigo colidiu  com herói
    pub fn check_collision(
        mut heroes: Query<&Position, With<Hero>>,
        mut enemies: Query<&Position, With<Enemy>>,
        mut damage_event: EventWriter<DamageEvent>,
    ) {
        for hero in heroes.iter_mut() {
            if enemies.iter_mut().any(|enemy| hero == enemy) {
                damage_event.send(DamageEvent);
            }
        }
    }
    /// Andar automaticamente
    pub fn wander(
        mut enemies: Query<
            (&mut Movement, &mut Direction, &Position, &mut MovementTimer),
            With<Enemy>,
        >,
        solids: Query<&Position, Or<(With<Block>, With<Item>)>>,
        tiles: Query<(&Position, &Tile)>,
    ) {
        for (mut enemy_movement, mut enemy_direction, enemy_position, mut enemy_timer) in
            enemies.iter_mut()
        {
            // Caso já tenha pausado o timer (ou seja, fora do cooldown de movimento)
            if enemy_timer.paused() {
                // Possibilidades de movimento
                let mut possibilities = vec![
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                ];
                // Adicionar 3x a direção atual do inimigo, para estimular ele a continuar no mesmo
                // caminho, se possível
                possibilities.push(*enemy_direction);
                possibilities.push(*enemy_direction);
                possibilities.push(*enemy_direction);

                // Possibilidades que estão ocupadas
                let mut occupied = Vec::new();

                // Adicionar no vetor de ocupados os sólidos (blocos e items) que estiverem nos
                // destinos correspondentes
                for solid in solids.iter() {
                    occupied.extend(possibilities.clone().iter().filter(|possiblity| {
                        (*enemy_position + possiblity.as_position()) == *solid
                    }))
                }

                // Adicionar no vetor de ocupados os tiles (com flecha) que estiverem nos
                // destinos correspondentes
                let arrow_tiles = tiles.iter().filter(|tile| matches![tile.1, Tile::Arrow(_)]);
                for arrow_tile in arrow_tiles {
                    occupied.extend(
                        possibilities
                            .clone()
                            .iter()
                            .filter(|p| (*enemy_position + p.as_position()) == *arrow_tile.0),
                    )
                }

                // Adicionar no vetor de ocupados as posições fora do map
                occupied.extend(
                    possibilities
                        .clone()
                        .iter()
                        .filter(|p| !(*enemy_position + p.as_position()).inside_boundaries()),
                );

                // Tirar do vetor de possibilidades os ocupados
                let possibilities: Vec<Direction> = possibilities
                    .to_vec()
                    .into_iter()
                    .filter(|p| !occupied.contains(p))
                    .collect();

                // Escolher um aleatório desses
                let new_direction = possibilities.choose(&mut rand::thread_rng()).copied();

                // Trocar a direção do robô, caso alguma direção tenha sido escolhida
                if let Some(direction) = new_direction {
                    *enemy_direction = direction;
                }
                // Alterar movimento
                *enemy_movement = Movement(new_direction);
                // Correr cooldown de movimento
                enemy_timer.unpause();
            }
        }
    }
}
