use crate::{
    AnimationTimer, Block, CurrentLevel, DamageEvent, Direction, Item, Lives, Movement,
    MovementTimer, Position, Size, Sprites, Tile,
};
use bevy::prelude::*;

/// Representa o herói
pub struct Hero;

impl Hero {
    /// Criar herói no mapa
    pub fn spawn(position: Position, commands: &mut Commands, sprites: &Res<Sprites>) -> Entity {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.chars.hero.clone(),
                global_transform: GlobalTransform::from_xyz(0.0, 0.0, 2.0),
                ..Default::default()
            })
            .insert(MovementTimer(Timer::from_seconds(0.3, true)))
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(Movement(None))
            .insert(Direction::Down)
            .insert(position)
            .insert(Size(1.0))
            .insert(Hero)
            .id()
    }
    /// Tratar da colisão com items
    pub fn collide_items(
        mut commands: Commands,
        mut hero: Query<&Position, With<Hero>>,
        mut items: Query<(Entity, &Position), With<Item>>,
    ) {
        for hero_pos in hero.iter_mut() {
            for (item, item_pos) in items.iter_mut() {
                if hero_pos == item_pos {
                    commands.entity(item).despawn()
                }
            }
        }
    }
    /// Tratar de dano recebido
    pub fn check_damage(
        mut reader: EventReader<DamageEvent>,
        mut lives: ResMut<State<Lives>>,
        mut level: ResMut<State<CurrentLevel>>,
        commands: Commands,
        entities: Query<(Entity, &CurrentLevel)>,
        sprites: Res<Sprites>,
    ) {
        if reader.iter().next().is_some() {
            let current_lives = lives.current().0;

            if current_lives == 0 {
                level.set(CurrentLevel::GameOver).unwrap();
                lives.set(Lives(3)).unwrap();
            } else {
                level.current().reset(commands, entities, sprites);
                lives.set(Lives(current_lives - 1)).unwrap();
            }
        }
    }
    /// Controles do herói
    pub fn controls(
        mut commands: Commands,
        input: Res<Input<KeyCode>>,
        // Query com o herói apenas
        mut hero: Query<(&Position, &mut Direction, &mut Movement, &mut MovementTimer), With<Hero>>,
        mut tiles: Query<(&Position, &Tile), Without<Hero>>,
        // Duas queries (que podem ter intersecções)
        // Ambos tem explicitamente Without<Hero>, para evitar conflito com outra query
        mut query: QuerySet<(
            // Essa query é para os blocos que o jogador pode empurrar
            // Então, ela pegará apenas blocos que tem Movement
            Query<(&Position, &mut Movement, &Block, Entity, &mut MovementTimer), Without<Hero>>,
            // Essa query é para verificar onde o jogador não pode ir
            // Então, ela pegará tudo que não é Tile
            Query<(&Position, (Without<Tile>, Without<Hero>))>,
        )>,
    ) {
        for (position, mut direction, mut movement, mut timer) in hero.iter_mut() {
            // Atirar
            if input.pressed(KeyCode::Space) {
                let target = *position + direction.as_position();
                let block = query.q0_mut().iter_mut().find(|block| *block.0 == target);
                if let Some(block) = block {
                    if block.2.is_breakable() {
                        commands.entity(block.3).despawn();
                    }
                }
            }

            // Movimento (apenas é possível mexer quando não está no meio do movimento)
            let new_movement = if input.pressed(KeyCode::Left)
                || input.pressed(KeyCode::A)
                || input.pressed(KeyCode::H)
            {
                *direction = Direction::Left;
                Movement(Some(Direction::Left))
            } else if input.pressed(KeyCode::Right)
                || input.pressed(KeyCode::D)
                || input.pressed(KeyCode::L)
            {
                *direction = Direction::Right;
                Movement(Some(Direction::Right))
            } else if input.pressed(KeyCode::Down)
                || input.pressed(KeyCode::S)
                || input.pressed(KeyCode::J)
            {
                *direction = Direction::Down;
                Movement(Some(Direction::Down))
            } else if input.pressed(KeyCode::Up)
                || input.pressed(KeyCode::W)
                || input.pressed(KeyCode::K)
            {
                *direction = Direction::Up;
                Movement(Some(Direction::Up))
            } else {
                Movement(None)
            };

            // Mudança de posição
            let offset = match new_movement.0 {
                Some(Direction::Left) => Position::xy(-1, 0),
                Some(Direction::Right) => Position::xy(1, 0),
                Some(Direction::Down) => Position::xy(0, -1),
                Some(Direction::Up) => Position::xy(0, 1),
                None => Position::xy(0, 0),
            };

            let new_pos = *position + offset;

            // Verificar se está em cima de uma flecha
            let is_over_arrow = tiles
                .iter_mut()
                .any(|tile| matches![tile.1, Tile::Arrow(_)] && tile.0 == position);

            // Só permitir o usuário interagir caso não tenha movimento pendente ou esteja numa
            // flecha
            if (timer.paused() || is_over_arrow)
                && new_movement.0.is_some()
                && new_pos.inside_boundaries()
            {
                // O movimento pode possivelmente mover um bloco.
                // Então vamos buscar agora (pois a busca do bloco irá tomar a referencia mutável)
                // se esse destino do bloco seria válido
                let new_block_pos = new_pos + offset;
                let free_pos = !query
                    .q1_mut()
                    .iter_mut()
                    .any(|other| *other.0 == new_block_pos);

                // Buscar um bloco na posição que o herói vai
                let block = query.q0_mut().iter_mut().find(|block| *block.0 == new_pos);
                // Caso haja
                if let Some(mut block) = block {
                    // Verificar se a posição que ele vai está dentro das bordas, está livre
                    // e se o bloco é móvel
                    if new_block_pos.inside_boundaries() && free_pos && block.2.is_movable() {
                        // Mover o bloco
                        *block.1 = new_movement;
                        block.4.unpause();
                        // Mover o herói
                        *movement = new_movement;
                        timer.unpause();
                    }
                } else {
                    // Mover o herói, apenas
                    *movement = new_movement;
                    timer.unpause();
                }
            }
        }
    }
}
