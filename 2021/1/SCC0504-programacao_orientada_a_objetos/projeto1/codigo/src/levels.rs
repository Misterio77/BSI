use crate::{
    Block, Direction, Enemy, GameScreen, Hero, Item, LivesCounter, Position, Size, Sprites, Text,
    Tile,
};
use bevy::prelude::*;

/// Estado de fase atual
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum CurrentLevel {
    GameOver,
    Win,
    Splash,
    Level1,
    Level2,
    Level3,
    Level4,
}

impl CurrentLevel {
    /// Verifica se dá para avançar de fase
    pub fn advance_condition(
        input: Res<Input<KeyCode>>,
        mut stage: ResMut<State<CurrentLevel>>,
        mut query: Query<&Item>,
    ) {
        let is_menu = (*stage.current() == CurrentLevel::Splash)
            || (*stage.current() == CurrentLevel::GameOver)
            || (*stage.current() == CurrentLevel::Win);
        if is_menu && input.just_pressed(KeyCode::Space) || !is_menu && query.iter_mut().len() == 0
        {
            let next = stage.current().next();
            stage.set(next).unwrap();
        }
    }
    /// Qual fase leva para qual
    pub fn next(&self) -> Self {
        match self {
            CurrentLevel::Win => CurrentLevel::Splash,
            CurrentLevel::GameOver => CurrentLevel::Splash,
            CurrentLevel::Splash => CurrentLevel::Level1,
            CurrentLevel::Level1 => CurrentLevel::Level2,
            CurrentLevel::Level2 => CurrentLevel::Level3,
            CurrentLevel::Level3 => CurrentLevel::Level4,
            CurrentLevel::Level4 => CurrentLevel::Win,
        }
    }
    /// Resetar a fase
    pub fn reset(
        &self,
        mut commands: Commands,
        mut entities: Query<(Entity, &CurrentLevel)>,
        sprites: Res<Sprites>,
    ) {
        for entity in entities.iter_mut().filter(|entity| entity.1 == self) {
            commands.entity(entity.0).despawn_recursive();
        }
        match self {
            CurrentLevel::Win => GameOver::start(commands, sprites),
            CurrentLevel::GameOver => GameOver::start(commands, sprites),
            CurrentLevel::Splash => Splash::start(commands, sprites),
            CurrentLevel::Level1 => Level1::start(commands, sprites),
            CurrentLevel::Level2 => Level2::start(commands, sprites),
            CurrentLevel::Level3 => Level3::start(commands, sprites),
            CurrentLevel::Level4 => Level4::start(commands, sprites),
        }
    }
}

/// Tela de game over
pub struct GameOver;
impl GameOver {
    /// Apagar elementos
    pub fn exit(mut commands: Commands, mut entities: Query<(Entity, &CurrentLevel)>) {
        for entity in entities
            .iter_mut()
            .filter(|entity| matches![entity.1, CurrentLevel::GameOver])
        {
            commands.entity(entity.0).despawn_recursive();
        }
    }
    /// Criar elementos
    pub fn start(mut commands: Commands, sprites: Res<Sprites>) {
        // Elemento pai
        let stage = commands.spawn().insert(CurrentLevel::GameOver).id();
        let children = Text::spawn_string("GAME OVER", Position::xy(4, 5), &mut commands, &sprites);
        // Adicionar todos os elementos como filhos do pai (para facilitar a remoção da fase)
        commands.entity(stage).push_children(&children);
    }
}
/// Tela de vitória
pub struct Win;
impl Win {
    /// Apagar elementos
    pub fn exit(mut commands: Commands, mut entities: Query<(Entity, &CurrentLevel)>) {
        for entity in entities
            .iter_mut()
            .filter(|entity| matches![entity.1, CurrentLevel::Win])
        {
            commands.entity(entity.0).despawn_recursive();
        }
    }
    /// Criar elementos
    pub fn start(mut commands: Commands, sprites: Res<Sprites>) {
        // Elemento pai
        let stage = commands.spawn().insert(CurrentLevel::Win).id();
        let mut children = Text::spawn_string(
            "VOCE VENCEU :D",
            Position::xy(1, 5),
            &mut commands,
            &sprites,
        );
        children.extend(Text::spawn_string(
            "BY GABRIEL E BACA",
            Position::xy(0, 2),
            &mut commands,
            &sprites,
        ));
        // Adicionar todos os elementos como filhos do pai (para facilitar a remoção da fase)
        commands.entity(stage).push_children(&children);
    }
}
/// Tela inicial
pub struct Splash;
impl Splash {
    /// Apagar elementos
    pub fn exit(mut commands: Commands, mut entities: Query<(Entity, &CurrentLevel)>) {
        for entity in entities
            .iter_mut()
            .filter(|entity| matches![entity.1, CurrentLevel::Splash])
        {
            commands.entity(entity.0).despawn_recursive();
        }
    }
    /// Criar elementos
    pub fn start(mut commands: Commands, sprites: Res<Sprites>) {
        // Elemento pai
        let stage = commands.spawn().insert(CurrentLevel::Splash).id();
        let mut children = vec![commands
            .spawn_bundle(SpriteBundle {
                material: sprites.logo.clone(),
                ..Default::default()
            })
            .insert(Size(1.0))
            .id()];
        children.extend(Text::spawn_string(
            "BEM-VINDO",
            Position::xy(5, 2),
            &mut commands,
            &sprites,
        ));
        children.extend(Text::spawn_string(
            "APERTE ESPACO",
            Position::xy(2, 1),
            &mut commands,
            &sprites,
        ));
        // Adicionar todos os elementos como filhos do pai (para facilitar a remoção da fase)
        commands.entity(stage).push_children(&children);
    }
}

/// Nível 1
pub struct Level1;
impl Level1 {
    /// Apagar elementos
    pub fn exit(mut commands: Commands, mut entities: Query<(Entity, &CurrentLevel)>) {
        for entity in entities
            .iter_mut()
            .filter(|entity| matches![entity.1, CurrentLevel::Level1])
        {
            commands.entity(entity.0).despawn_recursive();
        }
    }
    /// Criar elementos
    pub fn start(mut commands: Commands, sprites: Res<Sprites>) {
        // Elemento pai
        let stage = commands.spawn().insert(CurrentLevel::Level1).id();

        // Elementos básicos
        let mut children = vec![
            Hero::spawn(Position::xy(4, 6), &mut commands, &sprites),
            Item::Strawberry.spawn(Position::xy(0, 0), &mut commands, &sprites),
            Item::Lemon.spawn(Position::xy(10, 10), &mut commands, &sprites),
            Item::Cherry.spawn(Position::xy(10, 0), &mut commands, &sprites),
            Item::Grape.spawn(Position::xy(0, 10), &mut commands, &sprites),
            Enemy::Green.spawn(Position::xy(9, 0), &mut commands, &sprites),
            Enemy::Blue.spawn(Position::xy(1, 0), &mut commands, &sprites),
            Enemy::Yellow.spawn(Position::xy(0, 8), &mut commands, &sprites),
            Enemy::Magenta.spawn(Position::xy(9, 10), &mut commands, &sprites),
        ];

        // Contador de vida, fundo, e borda
        children.extend(spawn_life_counter(&mut commands, &sprites).into_iter());
        children.extend(spawn_background(Tile::Level1, &mut commands, &sprites).into_iter());
        children.extend(spawn_border(&mut commands, &sprites).into_iter());

        // Blocos destrutíveis e móveis
        for (x, y) in &[
            (0, 1),
            (0, 7),
            (1, 6),
            (1, 10),
            (2, 1),
            (2, 5),
            (2, 9),
            (3, 2),
            (5, 4),
            (5, 10),
            (6, 1),
            (6, 5),
            (7, 0),
            (7, 4),
            (8, 1),
            (8, 3),
            (8, 7),
            (8, 9),
            (9, 2),
            (9, 6),
            (10, 3),
            (10, 9),
        ] {
            children.push(Block::DiamondGreen.spawn(Position::xy(*x, *y), &mut commands, &sprites));
        }

        // Blocos indestrutíveis e imóveis
        for (x, y) in &[
            (1, 1),
            (1, 3),
            (1, 5),
            (1, 7),
            (1, 9),
            (3, 1),
            (3, 3),
            (3, 5),
            (3, 7),
            (3, 9),
            (5, 1),
            (5, 3),
            (5, 5),
            (5, 7),
            (5, 9),
            (7, 1),
            (7, 3),
            (7, 5),
            (7, 7),
            (7, 9),
            (9, 1),
            (9, 3),
            (9, 5),
            (9, 7),
            (9, 9),
        ] {
            children.push(Block::SolidRed.spawn(Position::xy(*x, *y), &mut commands, &sprites));
        }
        // Adicionar todos os elementos como filhos do pai (para facilitar a remoção da fase)
        commands.entity(stage).push_children(&children);
    }
}
pub struct Level2;
impl Level2 {
    /// Apagar elementos
    pub fn exit(mut commands: Commands, mut entities: Query<(Entity, &CurrentLevel)>) {
        for entity in entities
            .iter_mut()
            .filter(|entity| matches![entity.1, CurrentLevel::Level2])
        {
            commands.entity(entity.0).despawn_recursive();
        }
    }
    /// Criar elementos
    pub fn start(mut commands: Commands, sprites: Res<Sprites>) {
        // Elemento pai
        let stage = commands.spawn().insert(CurrentLevel::Level2).id();
        // Elementos básicos
        let mut children = vec![
            Hero::spawn(Position::xy(5, 5), &mut commands, &sprites),
            Item::Strawberry.spawn(Position::xy(9, 5), &mut commands, &sprites),
            Item::Lemon.spawn(Position::xy(1, 5), &mut commands, &sprites),
            Item::Cherry.spawn(Position::xy(5, 1), &mut commands, &sprites),
            Item::Grape.spawn(Position::xy(5, 9), &mut commands, &sprites),
            Enemy::Green.spawn(Position::xy(9, 9), &mut commands, &sprites),
            Enemy::Blue.spawn(Position::xy(9, 1), &mut commands, &sprites),
            Enemy::Yellow.spawn(Position::xy(1, 9), &mut commands, &sprites),
            Enemy::Magenta.spawn(Position::xy(1, 1), &mut commands, &sprites),
        ];

        // Contador de vida, fundo, e borda
        children.extend(spawn_life_counter(&mut commands, &sprites).into_iter());
        children.extend(spawn_background(Tile::Level2, &mut commands, &sprites).into_iter());
        children.extend(spawn_border(&mut commands, &sprites).into_iter());

        // Adicionar flechas
        for (x, y, d) in &[
            (0, 3, Direction::Up),
            (0, 4, Direction::Up),
            (0, 5, Direction::Up),
            (0, 6, Direction::Up),
            (0, 7, Direction::Up),
            (2, 3, Direction::Down),
            (2, 7, Direction::Down),
            (3, 0, Direction::Left),
            (3, 2, Direction::Right),
            (3, 4, Direction::Right),
            (3, 6, Direction::Right),
            (3, 8, Direction::Right),
            (3, 10, Direction::Right),
            (4, 3, Direction::Down),
            (4, 7, Direction::Down),
            (4, 8, Direction::Down),
            (5, 8, Direction::Left),
            (6, 3, Direction::Up),
            (6, 7, Direction::Up),
            (6, 8, Direction::Left),
            (7, 0, Direction::Right),
            (7, 2, Direction::Right),
            (7, 4, Direction::Left),
            (7, 6, Direction::Right),
            (7, 8, Direction::Left),
            (7, 10, Direction::Right),
            (8, 3, Direction::Up),
            (8, 6, Direction::Up),
            (8, 7, Direction::Up),
            (9, 6, Direction::Left),
            (10, 3, Direction::Up),
            (10, 6, Direction::Left),
            (10, 7, Direction::Down),
        ] {
            children.push(Tile::Arrow(*d).spawn(Position::xy(*x, *y), &mut commands, &sprites));
        }

        // Blocos indestrutíveis e imóveis 
        for (x, y) in &[
            (1, 3),
            (1, 7),
            (3, 1),
            (3, 3),
            (3, 5),
            (3, 7),
            (3, 9),
            (5, 3),
            (5, 7),
            (7, 1),
            (7, 3),
            (7, 5),
            (7, 7),
            (7, 9),
            (9, 3),
            (9, 7),
        ] {
            children.push(Block::SolidRed.spawn(Position::xy(*x, *y), &mut commands, &sprites));
        }
        // Adicionar todos os elementos como filhos do pai (para facilitar a remoção da fase)
        commands.entity(stage).push_children(&children);
    }
}
pub struct Level3;
impl Level3 {
    /// Apagar elementos
    pub fn exit(mut commands: Commands, mut entities: Query<(Entity, &CurrentLevel)>) {
        for entity in entities
            .iter_mut()
            .filter(|entity| matches![entity.1, CurrentLevel::Level3])
        {
            commands.entity(entity.0).despawn_recursive();
        }
    }
    /// Criar elementos
    pub fn start(mut commands: Commands, sprites: Res<Sprites>) {
        // Elemento pai
        let stage = commands.spawn().insert(CurrentLevel::Level3).id();
        // Elementos básicos
        let mut children = vec![
            Hero::spawn(Position::xy(5, 5), &mut commands, &sprites),
            Item::Sun.spawn(Position::xy(0, 5), &mut commands, &sprites),
            Item::Candle.spawn(Position::xy(2, 5), &mut commands, &sprites),
            Item::LightBulb.spawn(Position::xy(8, 5), &mut commands, &sprites),
            Item::Flashlight.spawn(Position::xy(10, 5), &mut commands, &sprites),
            Enemy::Green.spawn(Position::xy(5, 0), &mut commands, &sprites),
            Enemy::Blue.spawn(Position::xy(5, 2), &mut commands, &sprites),
            Enemy::Magenta.spawn(Position::xy(5, 8), &mut commands, &sprites),
            Enemy::Yellow.spawn(Position::xy(5, 10), &mut commands, &sprites),
        ];

        // Contador de vida, fundo, e borda
        children.extend(spawn_life_counter(&mut commands, &sprites).into_iter());
        children.extend(spawn_background(Tile::Level3, &mut commands, &sprites).into_iter());
        children.extend(spawn_border(&mut commands, &sprites).into_iter());

        // Blocos indestrutiveis e móveis
        for (x, y) in &[
            (1, 1),
            (1, 1),
            (1, 2),
            (1, 3),
            (1, 4),
            (1, 5),
            (1, 6),
            (1, 7),
            (1, 8),
            (1, 9),
            (2, 1),
            (2, 9),
            (3, 1),
            (3, 3),
            (3, 4),
            (3, 5),
            (3, 6),
            (3, 7),
            (3, 9),
            (4, 1),
            (4, 3),
            (4, 7),
            (4, 9),
            (5, 1),
            (5, 3),
            (5, 7),
            (5, 9),
            (6, 1),
            (6, 3),
            (6, 7),
            (6, 9),
            (7, 1),
            (7, 3),
            (7, 4),
            (7, 5),
            (7, 6),
            (7, 7),
            (7, 9),
            (8, 1),
            (8, 9),
            (9, 1),
            (9, 2),
            (9, 3),
            (9, 4),
            (9, 5),
            (9, 6),
            (9, 7),
            (9, 8),
            (9, 9),
        ] {
            children.push(Block::DiamondRed.spawn(Position::xy(*x, *y), &mut commands, &sprites));
        }
        // Adicionar todos os elementos como filhos do pai (para facilitar a remoção da fase)
        commands.entity(stage).push_children(&children);
    }
}
pub struct Level4;
impl Level4 {
    /// Apagar elementos
    pub fn exit(mut commands: Commands, mut entities: Query<(Entity, &CurrentLevel)>) {
        for entity in entities
            .iter_mut()
            .filter(|entity| matches![entity.1, CurrentLevel::Level4])
        {
            commands.entity(entity.0).despawn_recursive();
        }
    }
    /// Criar elementos
    pub fn start(mut commands: Commands, sprites: Res<Sprites>) {
        // Elemento pai
        let stage = commands.spawn().insert(CurrentLevel::Level4).id();
        // Elementos básicos
        let mut children = vec![
            Hero::spawn(Position::xy(5, 6), &mut commands, &sprites),
            Item::Sun.spawn(Position::xy(0, 10), &mut commands, &sprites),
            Item::Candle.spawn(Position::xy(0, 0), &mut commands, &sprites),
            Item::LightBulb.spawn(Position::xy(10, 10), &mut commands, &sprites),
            Item::Flashlight.spawn(Position::xy(10, 0), &mut commands, &sprites),
            Enemy::Green.spawn(Position::xy(0, 5), &mut commands, &sprites),
            Enemy::Blue.spawn(Position::xy(10, 5), &mut commands, &sprites),
            Enemy::Magenta.spawn(Position::xy(5, 0), &mut commands, &sprites),
            Enemy::Yellow.spawn(Position::xy(5, 10), &mut commands, &sprites),
        ];

        // Contador de vida, fundo, e borda
        children.extend(spawn_life_counter(&mut commands, &sprites).into_iter());
        children.extend(spawn_background(Tile::Level4, &mut commands, &sprites).into_iter());
        children.extend(spawn_border(&mut commands, &sprites).into_iter());

        // Blocos indestrutíveis e imóveis
        for (x, y) in &[
            (1, 0),
            (1, 2),
            (7, 2),
            (10, 2),
            (0, 3),
            (3, 4),
            (2, 5),
            (8, 5),
            (2, 7),
            (10, 7),
            (5, 8),
            (0, 9),
            (8, 9),
            (3, 10),
            (7, 10),
        ] {
            children.push(Block::SolidRed.spawn(Position::xy(*x, *y), &mut commands, &sprites));
        }

        // Blocos destrutíveis e imóveis
        for (x, y) in &[
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
            (8, 8),
            (9, 9),
            (1, 9),
            (2, 8),
            (3, 7),
            (4, 6),
            (6, 4),
            (7, 3),
            (8, 2),
            (9, 1),
            (3, 1),
            (4, 2),
            (5, 3),
            (7, 5),
            (8, 6),
            (9, 7),
            (5, 1),
            (6, 2),
            (8, 4),
            (9, 5),
            (7, 1),
            (9, 3),
            (1, 3),
            (2, 4),
            (3, 5),
            (5, 7),
            (6, 8),
            (7, 9),
            (1, 5),
            (2, 6),
            (4, 8),
            (5, 9),
            (1, 7),
            (3, 9),
        ] {
            children.push(Block::SolidGreen.spawn(Position::xy(*x, *y), &mut commands, &sprites));
        }
        // Adicionar todos os elementos como filhos do pai (para facilitar a remoção da fase)
        commands.entity(stage).push_children(&children);
    }
}

/// Criar borda do jogo
fn spawn_border(commands: &mut Commands, sprites: &Res<Sprites>) -> Vec<Entity> {
    let mut entities = Vec::new();
    // Criar quinas
    for (x, y, direction) in &[
        (-1, -1, Direction::Up),
        (-1, 11, Direction::Right),
        (11, 11, Direction::Down),
        (11, -1, Direction::Left),
    ] {
        entities.push(Tile::Corner(*direction).spawn(Position::xy(*x, *y), commands, sprites));
    }
    // Criar bordas laterais
    for (x, y, direction) in &[
        // Lado esquerdo
        (-1, 0, Direction::Right),
        (-1, 1, Direction::Right),
        (-1, 2, Direction::Right),
        (-1, 3, Direction::Right),
        (-1, 4, Direction::Right),
        (-1, 5, Direction::Right),
        (-1, 6, Direction::Right),
        (-1, 7, Direction::Right),
        (-1, 8, Direction::Right),
        (-1, 9, Direction::Right),
        (-1, 10, Direction::Right),
        // Lado direito
        (11, 0, Direction::Left),
        (11, 1, Direction::Left),
        (11, 2, Direction::Left),
        (11, 3, Direction::Left),
        (11, 4, Direction::Left),
        (11, 5, Direction::Left),
        (11, 6, Direction::Left),
        (11, 7, Direction::Left),
        (11, 8, Direction::Left),
        (11, 9, Direction::Left),
        (11, 10, Direction::Left),
        // Acima
        (0, 11, Direction::Down),
        (1, 11, Direction::Down),
        (2, 11, Direction::Down),
        (3, 11, Direction::Down),
        (4, 11, Direction::Down),
        (5, 11, Direction::Down),
        (6, 11, Direction::Down),
        (7, 11, Direction::Down),
        (8, 11, Direction::Down),
        (9, 11, Direction::Down),
        (10, 11, Direction::Down),
        // Abaixo
        (0, -1, Direction::Up),
        (1, -1, Direction::Up),
        (2, -1, Direction::Up),
        (3, -1, Direction::Up),
        (4, -1, Direction::Up),
        (5, -1, Direction::Up),
        (6, -1, Direction::Up),
        (7, -1, Direction::Up),
        (8, -1, Direction::Up),
        (9, -1, Direction::Up),
        (10, -1, Direction::Up),
    ] {
        entities.push(Tile::Border(*direction).spawn(Position::xy(*x, *y), commands, sprites));
    }
    // Devolver lista de entidades criadas
    entities
}
/// Criar fundo da fase, dado tipo
fn spawn_background(kind: Tile, commands: &mut Commands, sprites: &Res<Sprites>) -> Vec<Entity> {
    let mut entities = Vec::new();
    // Criar fundos
    for x in 0..GameScreen::width() {
        for y in 0..GameScreen::height() {
            entities.push(kind.spawn(Position::xy(x as i32, y as i32), commands, sprites));
        }
    }
    // Devolver lista de entidades criadas
    entities
}
/// Criar contador de vida
fn spawn_life_counter(commands: &mut Commands, sprites: &Res<Sprites>) -> Vec<Entity> {
    // Criar título
    let mut elements = Text::spawn_string(
            "VIDAS",
            Position::xy(12, 7),
            commands,
            sprites,
    );
    // Criar número contador
    let lives = Text::from_char('0').spawn(Position::xy(12, 6), commands, sprites);
    // Adicionar a lista de criadas
    elements.push(lives);
    // Adicionar tag para manipular depois
    commands.entity(lives).insert(LivesCounter);
    // Devolver lista de entidades criadas
    elements
}
