use crate::tilemap::Tilemap;

pub struct GameState {
    pub scene: Scene,
    tilemaps: Vec<Tilemap>,
    pub tilemap: Tilemap,
    pub climb_timer: f32,
    pub level: usize,
    max_level: usize,
    pub has_key: bool,
    pub has_ladder: bool,
}

impl GameState {
    pub fn new(tilemaps: Vec<Tilemap>) -> Self {
        let tilemap = tilemaps[0].clone();
        Self {
            scene: Scene::LoadMenu,
            tilemaps,
            tilemap,
            climb_timer: 0.0,
            level: 1,
            max_level: tilemaps.len() + 1,
            has_key: false,
            has_ladder: false,
        }
    }
    pub fn restart(&mut self) {
        self.has_ladder = false;
        self.has_key = false;
        self.tilemap = self.tilemaps[self.level - 1].clone();
    }

    pub fn next_level(&mut self) {
        self.has_ladder = false;
        self.has_key = false;
        if self.level == self.max_level {
            self.level += 1;
        } else {
            self.scene = Scene::LoadCredit;
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Scene {
    LoadMenu,
    Menu,
    LoadGame,
    Game,
    LoadCredit,
    Credit,
}