use comfy::*;

use crate::{assets, door, items, particles, player, WINDOW_HIGHT, WINDOW_WIDTH};
use crate::assets::{load_music, load_sound, load_sprites};
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
            // game_config_mut().dev.show_fps = false;
            // game_config_mut().dev.show_debug = false;
            // game_config_mut().dev.show_lighting_config = false;
            //c.renderer.window().set_fullscreen(Some(Fullscreen::Borderless(None)));
            c.renderer.window().set_resizable(false);
            main_camera_mut().zoom = WINDOW_WIDTH / 2.0;
            main_camera_mut().center = vec2(WINDOW_WIDTH / 2.0, -WINDOW_HIGHT / 2.0);
            load_sprites(c);
            load_sound();
            load_music();
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

// *** SETUP ***

fn setup(state: &mut GameState, _c: &mut EngineContext) {
    match state.scene {
        Scene::LoadMenu => { setup_load_menu(state) }
        Scene::LoadLevel => { setup_load_level(state) }
        Scene::LoadCredit => { setup_load_credit(state)}
        _ => {}
    }
}

fn setup_load_menu(state: &mut GameState) {
    println!("setup_load_menu");
    state.scene = Scene::Menu;
    play_music("game_music");
    play_sound("welcome");
    assets::spawn_logo(vec2(WINDOW_WIDTH / 2.0, WINDOW_HIGHT / 2.0 * -1.0));
}

fn setup_load_credit(state: &mut GameState) {
    println!("setup_load_credit");
    play_sound("thank_you");
    dispawn_all();
    state.scene = Scene::Credit;

}

fn setup_load_level(state: &mut GameState) {
    println!("setup_load_level");
    if state.level == 1{
        play_sound("hello");
    }
    dispawn_all();
    items::spawn_ladders(state);
    items::spawn_pulleys(state);
    items::spawn_keys(state);
    particles::spawn(state);
    player::spawns(state);
    door::spawns(state);
    state.scene = Scene::EnterLevel;
}

// *** DRAW ***

fn draw(state: &mut GameState) {
    match state.scene {
        Scene::Menu => { draw_menu(state) }
        Scene::Game => { draw_play(state) }
        Scene::Credit => { draw_credit(state) }
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
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("deco2"), 100, WHITE);
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("deco"), 6, WHITE);
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("level"), 3, WHITE);
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("background"), 2, GRAY);
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("background2"), 1, DARKBROWN);
}

fn draw_menu(_state: &GameState) {
    //draw_sprite(texture_id("game_logo"), vec2(WINDOW_WIDTH / 2.0, WINDOW_HIGHT / 2.0 * -1.0), WHITE, 0, vec2(128.0 * 2.0, 48.0 * 2.0))
}

fn draw_credit(_state: &GameState) {
    //clear_background(Color::rgb8(255, 255, 255));
    //clear_background(Color::rgb8(109, 194, 202));
    /*
    draw_sprite(
        texture_id("bg_sky"), vec2(WINDOW_WIDTH / 2.0, WINDOW_HIGHT/ 2.0 * -1.0),
        WHITE, 0, vec2(WINDOW_WIDTH , WINDOW_HIGHT));
     */
    draw_sprite(
        texture_id("thank_you"), vec2(WINDOW_WIDTH / 2.0, WINDOW_HIGHT / 2.0 * -1.0),
        WHITE, 1, vec2(66.0 * 2.0, 40.0 * 2.0));
    draw_sprite(
        texture_id("created"), vec2(WINDOW_WIDTH / 2.0 +150.0, WINDOW_HIGHT / 2.0 * -1.0 - 80.0),
        WHITE, 1, vec2(66.0 * 2.0, 40.0 * 2.0))
}

// *** UPDATE ***

fn update(state: &mut GameState, c: &mut EngineContext) {
    match state.scene {
        Scene::Menu => { update_menu(state, c) }
        Scene::Game => { update_play(state, c) }
        _ => {}
    }
}

fn update_menu(state: &mut GameState, c: &mut EngineContext) {
    assets::update(state,c);
}

fn update_play(state: &mut GameState, c: &mut EngineContext) {
    door::update(state, c);
    items::update(state, c);
}

fn dispawn_all() {
    for (entity, _) in
    world().query::<&mut Transform>().iter()
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
            if is_key_pressed(KeyCode::Return) || is_key_pressed(KeyCode::Space) {
                println!("switch to loadLevel!");
                state.scene = Scene::LoadLevel;
            }
        }
        Scene::Credit => {
            if is_key_pressed(KeyCode::Escape) {
                println!("switch to loadMenu!");
                state.restart();
                dispawn_all();
                state.scene = Scene::LoadMenu;
            }
        }
        Scene::Game => {
            if is_key_pressed(KeyCode::R) {
                println!("switch to loadLevel!");
                state.restart();
                state.scene = Scene::LoadLevel;
            }
            player::handle_input(state, c);
        }
        _ => {}
    }
}