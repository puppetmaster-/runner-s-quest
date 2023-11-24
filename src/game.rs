use comfy::*;
use comfy::log::{Level, log};

use crate::{door, items, player, WINDOW_HIGHT, WINDOW_WIDTH};
use crate::assets::load_sprites;
use crate::state::{GameState, Scene};
use crate::tilemap::tilemap_helper;
use crate::tilemap::tilemap_helper::TILEMAP_ORIGIN;


pub struct ComfyGame {
    pub state: Option<GameState>,
}

impl ComfyGame {
}

impl GameLoop for ComfyGame {
    fn new(_engine: &mut EngineState) -> Self {
        Self {
            state: None,
        }
    }

    fn update(&mut self, c: &mut EngineContext) {
        if self.state.is_none() {
            // debug mode
            //game_config_mut().dev.show_fps = true;
            //c.renderer.window().set_fullscreen(Some(Fullscreen::Borderless(None)));
            c.renderer.window().set_resizable(false);
            main_camera_mut().zoom = WINDOW_WIDTH / 2.0;
            main_camera_mut().center = vec2(WINDOW_WIDTH / 2.0, -WINDOW_HIGHT / 2.0);
            load_sprites(c);
            let state = GameState::new(tilemap_helper::load_levels());
            self.state = Some(state);
        }

        if let Some(state) = self.state.as_mut() {
            setup(state, c);

            handle_input(state, c);

            update(state, c);

            draw(state);
        }
    }
}

fn setup(state: &mut GameState, _c: &mut EngineContext) {
    match state.scene {
        Scene::LoadMenu => { setup_load_menu(state) }
        Scene::LoadLevel => { setup_load_level(state) }
        _ => {}
    }
}

fn setup_load_menu(state: &mut GameState) {
    println!("setup_load_menu");
    state.scene = Scene::Menu;
}

fn setup_load_level(state: &mut GameState) {
    println!("setup_load_level");
    dispawn_all();
    items::spawn_ladders(state);
    items::spawn_pulleys(state);
    items::spawn_keys(state);
    player::spawns(state);
    door::spawns(state);
    state.scene = Scene::EnterLevel;
}

fn dispawn_all() {
    for (entity, _) in
    world().query::<(&mut Transform)>().iter()
    {
        commands().despawn(entity);
    }
}


fn handle_input(state: &mut GameState, c: &mut EngineContext) {
    match state.scene {
        Scene::Menu => {
            if is_key_pressed(KeyCode::Escape) {
                std::process::exit(0);
            }
            if is_key_pressed(KeyCode::Return) {
                println!("switch to loadLevel!");
                state.scene = Scene::LoadLevel;
            }
        }
        Scene::Game => {
            if is_key_pressed(KeyCode::Escape) {
                println!("switch to loadMenu!");
                state.restart();
                dispawn_all();
                state.scene = Scene::LoadMenu;
            }
            player::handle_input(state, c);
        }
        _ => {}
    }
}


fn draw(state: &mut GameState) {
    match state.scene {
        Scene::Menu => { draw_menu(state) }
        Scene::Game => { draw_play(state) }
        Scene::EnterLevel => {
            draw_enter_transition(state);
            draw_play(state);
        }
        Scene::ExitLevel => {
            draw_exit_transition(state);
            draw_play(state);
        }
        _ => {}
    }
}

fn draw_exit_transition(state: &mut GameState) {
    println!("draw_exit_transition");
    // when finish call state.next_level()
    state.next_level();
}

fn draw_enter_transition(state: &mut GameState) {
    println!("draw_enter_transition");
    // when finish change state.scene = Scene::Game
    state.scene = Scene::Game;
}


fn draw_play(state: &GameState) {
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("deco2"), 7, WHITE);
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("deco"), 6, WHITE);
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("level"), 3, WHITE);
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("background"), 2, GRAY);
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("background2"), 1, GRAY);
}

fn draw_menu(_state: &GameState) {
    draw_sprite(texture_id("game_logo"), vec2(WINDOW_WIDTH / 2.0, WINDOW_HIGHT / 2.0 * -1.0), WHITE, 0, vec2(128.0 * 2.0, 48.0 * 2.0))
}


fn update(state: &mut GameState, c: &mut EngineContext) {
    match state.scene {
        Scene::Menu => { update_menu(state, c) }
        Scene::Game => { update_play(state, c) }
        _ => {}
    }
}

fn update_menu(_state: &mut GameState, _c: &mut EngineContext) {

}

fn update_play(state: &mut GameState, c: &mut EngineContext) {
    door::update(state, c);
    items::update(state, c);
}
