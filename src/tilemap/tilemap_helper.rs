use comfy::*;
use comfy::egui::TextBuffer;
use include_assets::{include_dir, NamedArchive};

use crate::state::GameState;
use crate::tilemap::Tilemap;

pub const TILEMAP_ORIGIN: Vec2 = vec2(8.0, 8.0);

const ID_WALL_1: u32 = 0;
const ID_WALL_2: u32 = 1;
const ID_LADDER: u32 = 8;
const ID_MISSING_LADDER: u32 = 17;
const ID_LINE: u32 = 9;

pub fn set_ladder_at(state: &mut GameState, pos:Vec2 ){
    state.tilemap.set_new_id_at(state.tilemap.get_layer_id("level"),8,pos);
}

pub fn is_ladder(id: Option<u32>) -> bool {
    is_ladder_or(&[id])
}

pub fn is_missing_ladder(id: Option<u32>) -> bool {
    id == Some(ID_MISSING_LADDER)
}

#[allow(dead_code)]
pub fn is_ladder_and(ids: &[Option<u32>]) -> bool {
    return !ids.iter().any(|id| *id != Some(ID_LADDER));
}

pub fn is_ladder_or(ids: &[Option<u32>]) -> bool {
    return ids.iter().any(|id| *id == Some(ID_LADDER));
}
pub fn is_line(id: Option<u32>) -> bool{
    id == Some(ID_LINE)
}

pub fn is_wall(id: Option<u32>) -> bool {
    id == Some(ID_WALL_1) || id == Some(ID_WALL_2)
}

pub fn is_not_wall(id: Option<u32>) -> bool {
    !is_wall(id)
}

pub fn is_air(id: Option<u32>) -> bool {
    !is_wall(id) && !is_ladder(id)
}

pub fn get_id(state: &GameState, position: Vec2) -> Option<u32> {
    state.tilemap.get_id_at_position(state.tilemap.get_layer_id("level"), position)
}

pub fn get_id_logic(state: &GameState, position: Vec2) -> Option<u32> {
    state.tilemap.get_id_at_position(state.tilemap.get_layer_id("logic"), position)
}

pub fn load_levels() -> Vec<Tilemap> {
    let archive = NamedArchive::load(include_dir!("assets/levels"));
    archive
        .assets()
        .filter(|(name, _)| name.ends_with(".json"))
        .sorted_by_key(|(name, _)| *name)
        .map(|(_, data)| {
            Tilemap::from_pyxeledit(Rect::from_xywh(0.0, 0.0, 128.0, 128.0), String::from_utf8_lossy(data).as_str())
        })
        .collect()
}
