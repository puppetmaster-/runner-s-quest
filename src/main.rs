extern crate core;

use comfy::*;

use crate::game::ComfyGame;

mod state;
mod game;
mod tilemap;
mod utils;
mod player;
mod items;
mod assets;
mod door;
mod particles;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HIGHT: f32 = 512.0;


fn main() {
    #[cfg(feature = "color-backtrace")]
    color_backtrace::install();

    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run());
    }

    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(run());
    }
}

pub async fn run() {
    init_game_config(
        "Runner's Quest".to_string(),
        "v0.0.1",
        _comfy_default_config,
    );

    let mut engine = EngineState::new();
    let game = ComfyGame::new(&mut engine);

    run_comfy_main_async(game, engine).await;
}

pub fn _comfy_default_config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(WINDOW_WIDTH as u32, WINDOW_HIGHT as u32),
        ..config
    }
}
