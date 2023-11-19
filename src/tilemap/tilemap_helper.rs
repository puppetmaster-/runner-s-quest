use comfy::*;

use crate::state::GameState;
use crate::tilemap::Tilemap;

pub fn is_ladder(id: Option<u32>) -> bool {
    is_ladder_or(&[id])
}

#[allow(dead_code)]
pub fn is_ladder_and(ids: &[Option<u32>]) -> bool {
    return !ids.iter().any(|id| *id != Some(8));
}

pub fn is_ladder_or(ids: &[Option<u32>]) -> bool {
    return ids.iter().any(|id| *id == Some(8));
}

pub fn is_wall(id: Option<u32>) -> bool {
    id == Some(0) || id == Some(1)
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

pub fn load_levels() -> Vec<Tilemap> {
    let tiles_json_vec = include_bytes!("../../assets/tileset.json").to_vec();
    let mut tilemaps = Vec::new();
    let mut tilemap = Tilemap::from_pyxeledit(Rect::new(0.0, 0.0, 128.0, 128.0), String::from_utf8(tiles_json_vec).unwrap().as_str());
    tilemaps.push(tilemap.clone());
    tilemaps.push(tilemap.clone());
    tilemaps
}