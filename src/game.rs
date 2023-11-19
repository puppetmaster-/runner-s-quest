use comfy::*;

use crate::state::{GameState, Scene};
use crate::tilemap::Tilemap;
use crate::{door, items, player, WINDOW_HIGHT, WINDOW_WIDTH};
use crate::assets::load_sprites;


const TILEMAP_ORIGIN: Vec2 = vec2(8.0, 8.0);


pub struct ComfyGame {
    pub engine: EngineState,
    pub state: Option<GameState>,
}

impl ComfyGame {
    pub fn new(engine: EngineState) -> Self {
        Self {
            state: None,
            engine
        }
    }
}

impl GameLoop for ComfyGame {

    fn engine(&mut self) -> &mut EngineState {
        &mut self.engine
    }

    fn update(&mut self) {
        let mut c = self.engine.make_context();

        if self.state.is_none() {
            // debug mode
            //game_config_mut().dev.show_fps = true;
            //c.renderer.window().set_fullscreen(Some(Fullscreen::Borderless(None)));
            c.renderer.window().set_resizable(false);
            main_camera_mut().zoom = 512.0;
            main_camera_mut().center = vec2(512.0/2.0,-256.0/2.0);
            load_sprites(&mut c);
            let tiles_json_vec = include_bytes!("../assets/tileset.json").to_vec();
            let mut tilemap = Tilemap::from_pyxeledit(Rect::new(0.0,0.0,128.0,128.0),String::from_utf8(tiles_json_vec).unwrap().as_str());
            tilemap.color(0,DARKGREEN);
            let state = GameState::new(tilemap);
            self.state = Some(state);
        }

        if let Some(state) = self.state.as_mut() {

            run_early_update_stages(&mut c);

            setup(state, &mut c);

            handle_input(state, &mut c);

            update(state, &mut c);

            draw(state);

            run_late_update_stages(&mut c, delta());
        }
    }
}

fn setup(state: &mut GameState, _c: &mut EngineContext) {
    match state.scene{
        Scene::LoadMenu => {setup_load_menu(state)}
        Scene::LoadGame => {setup_load_game(state)}
        _ => {}
    }
}

fn setup_load_menu(state: &mut GameState) {
    state.scene = Scene::Menu;
}

fn setup_load_game(state: &mut GameState) {
    let player_spawn_pos = state.tilemap.get_all_position_from_id(state.tilemap.get_layer_id("logic"),16);
    player::spawn(player_spawn_pos[0]+TILEMAP_ORIGIN + vec2(0.0, -5.0));
    let item_ladder_pos = state.tilemap.get_all_position_from_id(state.tilemap.get_layer_id("logic"),24);
    items::spawn_ladder(item_ladder_pos[0]+TILEMAP_ORIGIN + vec2(0.0, -8.0));
    let item_pulley_pos = state.tilemap.get_all_position_from_id(state.tilemap.get_layer_id("logic"),25);
    items::spawn_pulley(item_pulley_pos[0]+TILEMAP_ORIGIN + vec2(0.0, -8.0));
    let item_key_pos = state.tilemap.get_all_position_from_id(state.tilemap.get_layer_id("logic"),26);
    items::spawn_key(item_key_pos[0]+TILEMAP_ORIGIN + vec2(0.0, -8.0));
    let door_pos = state.tilemap.get_all_position_from_id(state.tilemap.get_layer_id("logic"),11);
    door::spawn(door_pos[0]+TILEMAP_ORIGIN+ vec2(0.0, -8.0));
    state.scene = Scene::Game;
}


fn handle_input(state: &mut GameState,c: &mut EngineContext) {
    match state.scene {
        Scene::Menu => {
            if is_key_pressed(KeyCode::Escape)  {
                std::process::exit(0);
            }
            if is_key_pressed(KeyCode::Return) {
                println!("switch to game!");
                state.scene = Scene::LoadGame;
            }
        }
        Scene::Game => {
            if is_key_pressed(KeyCode::Escape)  {
                println!("switch to menu!");
                state.scene = Scene::LoadMenu;
            }
            player::handle_input(state, c);
        }
        _ => {}
    }
}


fn draw(state: & GameState) {
    match state.scene {
        Scene::Menu => { draw_menu(state)}
        Scene::Game => { draw_play(state)}
        _ => {}
    }
}

fn draw_play(state: &GameState) {
    state.tilemap.draw(texture_id("tileset"),TILEMAP_ORIGIN,state.tilemap.get_layer_id("deco2"),7, WHITE);
    state.tilemap.draw(texture_id("tileset"),TILEMAP_ORIGIN,state.tilemap.get_layer_id("deco"),6, WHITE);
    state.tilemap.draw(texture_id("tileset"),TILEMAP_ORIGIN,state.tilemap.get_layer_id("level"),3, WHITE);
    state.tilemap.draw(texture_id("tileset"),TILEMAP_ORIGIN,state.tilemap.get_layer_id("background"),2, GRAY);
    state.tilemap.draw(texture_id("tileset"),TILEMAP_ORIGIN,state.tilemap.get_layer_id("background2"),1, GRAY);
}

fn draw_menu(_state: &GameState) {
    draw_sprite(texture_id("game_logo"),vec2(WINDOW_WIDTH / 4.0,WINDOW_HIGHT / 4.0 *-1.0),WHITE,0,vec2(128.0*2.0,48.0*2.0))
}


fn update(state: &mut GameState, c: &mut EngineContext) {
    match state.scene{
        Scene::Menu => { update_menu(state, c);}
        Scene::Game => { update_play(state, c)}
        _ => {}
    }
}

fn update_menu(_state: &mut GameState,_c: &mut EngineContext) {
}
fn update_play(state: &mut GameState,c: &mut EngineContext) {
    door::update(state, c);
    items::update(state, c);
}



