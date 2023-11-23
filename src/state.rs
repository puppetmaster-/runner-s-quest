use crate::tilemap::Tilemap;

pub struct GameState {
    pub scene: Scene,
    tilemaps: Vec<Tilemap>,
    pub tilemap: Tilemap,
    pub climb_timer: f32,
    pub level: usize,
    max_level: usize,
    pub has_key: bool,
    pub ladder: usize,
}

impl GameState {
    pub fn new(tilemaps: Vec<Tilemap>) -> Self {
        let level = 1;
        let tilemap = tilemaps[level - 1].clone();
        let max = tilemaps.len() + 1;
        Self {
            scene: Scene::LoadMenu,
            tilemaps,
            tilemap,
            climb_timer: 0.0,
            level,
            max_level: max,
            has_key: false,
            ladder: 0
        }
    }
    pub fn restart(&mut self) {
        println!("restart");
        self.ladder = 0;
        self.has_key = false;
        self.climb_timer = 0.0;
        self.tilemap = self.tilemaps[self.level - 1].clone();
    }

    pub fn next_level(&mut self) {
        println!("next_level");
        self.ladder = 0;
        self.has_key = false;
        self.climb_timer = 0.0;
        if self.level < self.max_level {
            println!("go loading next level");
            self.level += 1;
            self.tilemap = self.tilemaps[self.level - 1].clone();
            self.scene = Scene::LoadLevel;
        } else {
            println!("go loading credit");
            self.scene = Scene::LoadCredit;
        }
    }
    pub fn exit_level(&mut self) {
        println!("exit_level");
        self.scene = Scene::ExitLevel;
    }

    pub fn give_ladder(&mut self) -> bool {
        println!("give_ladder");
        if self.ladder > 0 {
            self.ladder -= 1;
            return true;
        }else{
            return false;
        }
    }

    pub fn pickup_ladder(&mut self){
        println!("pickup_ladder");
        self.ladder += 1;
    }
}

#[derive(PartialEq, Debug)]
pub enum Scene {
    LoadMenu,
    Menu,
    LoadLevel,
    Game,
    EnterLevel,
    ExitLevel,
    LoadCredit,
    Credit,
}