
use crate::tilemap::Tilemap;

pub struct GameState {
    pub scene: Scene,
    pub tilemap: Tilemap,
    pub climb_timer: f32,
    pub last_animation: &'static str,
}

// This state can be initialized from the engine's context.
impl GameState {
    pub fn new(tilemap: Tilemap) -> Self {
        Self {
            scene: Scene::LoadMenu,
            tilemap,
            climb_timer: 0.0,
            last_animation: "",
        }
    }
}

#[derive(PartialEq,Debug)]
pub enum Scene{
    LoadMenu,
    Menu,
    LoadGame,
    Game,
}