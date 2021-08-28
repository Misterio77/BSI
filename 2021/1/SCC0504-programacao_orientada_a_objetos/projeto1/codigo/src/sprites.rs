use bevy::prelude::*;

/// Criador de sprites

/// Sprites do jogo
pub struct Sprites {
    pub logo: Handle<ColorMaterial>,
    pub text: Handle<TextureAtlas>,
    pub chars: SpritesChars,
    pub tiles: SpritesTiles,
    pub blocks: SpritesBlocks,
    pub items: SpritesItems,
}

/// Sprites de personagens
pub struct SpritesChars {
    pub hero: Handle<TextureAtlas>,
    pub robot: Handle<TextureAtlas>,
}
/// Sprites de tiles (fundos, etc)
pub struct SpritesTiles {
    pub arrow: Handle<ColorMaterial>,
    pub level1: Handle<ColorMaterial>,
    pub level2: Handle<ColorMaterial>,
    pub level3: Handle<ColorMaterial>,
    pub level4: Handle<ColorMaterial>,
    pub border: Handle<ColorMaterial>,
    pub corner: Handle<ColorMaterial>,
}
/// Sprites de blocos
pub struct SpritesBlocks {
    pub diamond_red: Handle<ColorMaterial>,
    pub diamond_green: Handle<ColorMaterial>,
    pub solid_red: Handle<ColorMaterial>,
    pub solid_green: Handle<ColorMaterial>,
}
/// Sprites de itens
pub struct SpritesItems {
    pub candle: Handle<ColorMaterial>,
    pub cherry: Handle<ColorMaterial>,
    pub flashlight: Handle<ColorMaterial>,
    pub grape: Handle<ColorMaterial>,
    pub lemon: Handle<ColorMaterial>,
    pub lightbulb: Handle<ColorMaterial>,
    pub strawberry: Handle<ColorMaterial>,
    pub sun: Handle<ColorMaterial>,
}

impl Sprites {
    /// Carregar sprites do sistema de arquivos, criar materiais e atlas
    pub fn load(
        assets: Res<AssetServer>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        // Definir algumas cores comuns
        // Fundos e bordas
        let blue = Color::rgb(0.129, 0.141, 1.0);
        // Blocos destrutíveis
        let green = Color::rgb(0.129, 0.859, 0.129);
        // Blocos indestrutíveis
        let red = Color::rgb(1.0, 0.141, 0.129);
        // Título do jogo
        let yellow = Color::rgb(0.9, 0.9, 0.2);

        Self {
            // Criar atlas da fonte do jogo
            text: atlases.add(TextureAtlas::from_grid(
                assets.load("text.png"),
                Vec2::new(8.0, 8.0),
                39,
                1,
            )),
            // Criar sprite do logo
            logo: materials.add(ColorMaterial {
                color: yellow,
                texture: Some(assets.load("logo.png")),
            }),
            // Criar sprites dos personagens
            chars: SpritesChars {
                // Atlas (animado) do herói
                hero: atlases.add(TextureAtlas::from_grid(
                    assets.load("chars/hero.png"),
                    Vec2::new(26.0, 26.0),
                    3,
                    4,
                )),
                // Atlas (animado) dos inimigos
                robot: atlases.add(TextureAtlas::from_grid(
                    assets.load("chars/robot.png"),
                    Vec2::new(16.0, 16.0),
                    3,
                    4,
                )),
            },
            // Crias sprites dos fundos
            tiles: SpritesTiles {
                // Flecha
                arrow: materials.add(ColorMaterial {
                    color: blue,
                    texture: Some(assets.load("tiles/arrow.png")),
                }),
                // Fundo do nível 1
                level1: materials.add(ColorMaterial {
                    color: blue,
                    texture: Some(assets.load("tiles/1.png")),
                }),
                // Fundo do nível 2
                level2: materials.add(ColorMaterial {
                    color: blue,
                    texture: Some(assets.load("tiles/2.png")),
                }),
                // Fundo do nível 3
                level3: materials.add(ColorMaterial {
                    color: blue,
                    texture: Some(assets.load("tiles/3.png")),
                }),
                // Fundo do nível 4
                level4: materials.add(ColorMaterial {
                    color: blue,
                    texture: Some(assets.load("tiles/4.png")),
                }),
                // Borda do mapa
                border: materials.add(ColorMaterial {
                    color: blue,
                    texture: Some(assets.load("tiles/border.png")),
                }),
                // Quinas do mapa
                corner: materials.add(ColorMaterial {
                    color: blue,
                    texture: Some(assets.load("tiles/corner.png")),
                }),
            },
            // Criar sprites dos blocos
            blocks: SpritesBlocks {
                // Bloco indestrutível móvel
                diamond_red: materials.add(ColorMaterial {
                    color: red,
                    texture: Some(assets.load("blocks/diamond.png")),
                }),
                // Bloco destrutível móvel
                diamond_green: materials.add(ColorMaterial {
                    color: green,
                    texture: Some(assets.load("blocks/diamond.png")),
                }),
                // Bloco indestrutível imóvel
                solid_red: materials.add(ColorMaterial {
                    color: red,
                    texture: Some(assets.load("blocks/solid.png")),
                }),
                // Bloco destrutível imóvel
                solid_green: materials.add(ColorMaterial {
                    color: green,
                    texture: Some(assets.load("blocks/solid.png")),
                }),
            },
            // Criar sprites dos itens
            items: SpritesItems {
                candle: materials.add(assets.load("items/candle.png").into()),
                cherry: materials.add(assets.load("items/cherry.png").into()),
                flashlight: materials.add(assets.load("items/flashlight.png").into()),
                grape: materials.add(assets.load("items/grape.png").into()),
                lemon: materials.add(assets.load("items/lemon.png").into()),
                lightbulb: materials.add(assets.load("items/lightbulb.png").into()),
                strawberry: materials.add(assets.load("items/strawberry.png").into()),
                sun: materials.add(assets.load("items/sun.png").into()),
            },
        }
    }
}
