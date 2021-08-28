use bevy::prelude::*;
use projeto1::levels::*;
use projeto1::{DamageEvent, Enemy, GameScreen, Hero, Lives, Sprites};

fn main() {
    App::build()
        // Adicionar janela
        .insert_resource(WindowDescriptor {
            title: "Skooter!".into(),
            width: 800.0,
            height: 800.0,
            ..Default::default()
        })
        // Cor de fundo
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .add_plugins(DefaultPlugins)
        // Carregar sprites e ativar renderização
        .add_startup_system(setup.system())
        // Adicionar hooks de escalar, posicionar, e mover elementos
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(GameScreen::scaling.system())
                .with_system(GameScreen::positions.system())
                .with_system(GameScreen::movements.system()),
        )
        // Adicionar animações
        .add_system(GameScreen::animations.system())
        // Adicionar controles do herói
        .add_system(Hero::controls.system())
        // Adicionar colisões do herói com item
        .add_system(Hero::collide_items.system())
        // Adicionar colisões de inimigo com herói
        .add_system(Enemy::check_collision.system())
        // Começar na splash
        .add_state(CurrentLevel::Splash)
        // Adicionar vidas
        .add_state(Lives::default())
        // Adicionar hook de atualizar vidas na tela
        .add_system(Lives::update_lives.system())
        // Adicionar movimento automático nos inimigos
        .add_system(Enemy::wander.system())
        // Adicionar evento de dano no heróiu
        .add_event::<DamageEvent>()
        // Adicionar listener do evento de dano no herói
        .add_system(Hero::check_damage.system())
        // Vitória
        .add_system_set(SystemSet::on_enter(CurrentLevel::Win).with_system(Win::start.system()))
        .add_system_set(SystemSet::on_exit(CurrentLevel::Win).with_system(Win::exit.system()))
        .add_system_set(
            SystemSet::on_update(CurrentLevel::Win)
                .with_system(CurrentLevel::advance_condition.system()),
        )
        // Game Over
        .add_system_set(
            SystemSet::on_enter(CurrentLevel::GameOver).with_system(GameOver::start.system()),
        )
        .add_system_set(
            SystemSet::on_exit(CurrentLevel::GameOver).with_system(GameOver::exit.system()),
        )
        .add_system_set(
            SystemSet::on_update(CurrentLevel::GameOver)
                .with_system(CurrentLevel::advance_condition.system()),
        )
        // Splash
        .add_system_set(
            SystemSet::on_enter(CurrentLevel::Splash).with_system(Splash::start.system()),
        )
        .add_system_set(SystemSet::on_exit(CurrentLevel::Splash).with_system(Splash::exit.system()))
        .add_system_set(
            SystemSet::on_update(CurrentLevel::Splash)
                .with_system(CurrentLevel::advance_condition.system()),
        )
        // Nivel 1
        .add_system_set(
            SystemSet::on_enter(CurrentLevel::Level1).with_system(Level1::start.system()),
        )
        .add_system_set(SystemSet::on_exit(CurrentLevel::Level1).with_system(Level1::exit.system()))
        .add_system_set(
            SystemSet::on_update(CurrentLevel::Level1)
                .with_system(CurrentLevel::advance_condition.system()),
        )
        // Nivel 2
        .add_system_set(
            SystemSet::on_enter(CurrentLevel::Level2).with_system(Level2::start.system()),
        )
        .add_system_set(SystemSet::on_exit(CurrentLevel::Level2).with_system(Level2::exit.system()))
        .add_system_set(
            SystemSet::on_update(CurrentLevel::Level2)
                .with_system(CurrentLevel::advance_condition.system()),
        )
        // Nivel 3
        .add_system_set(
            SystemSet::on_enter(CurrentLevel::Level3).with_system(Level3::start.system()),
        )
        .add_system_set(SystemSet::on_exit(CurrentLevel::Level3).with_system(Level3::exit.system()))
        .add_system_set(
            SystemSet::on_update(CurrentLevel::Level3)
                .with_system(CurrentLevel::advance_condition.system()),
        )
        // Nivel 4
        .add_system_set(
            SystemSet::on_enter(CurrentLevel::Level4).with_system(Level4::start.system()),
        )
        .add_system_set(SystemSet::on_exit(CurrentLevel::Level4).with_system(Level4::exit.system()))
        .add_system_set(
            SystemSet::on_update(CurrentLevel::Level4)
                .with_system(CurrentLevel::advance_condition.system()),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    atlases: ResMut<Assets<TextureAtlas>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Sprites::load(assets, atlases, materials));
}

