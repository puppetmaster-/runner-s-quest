use comfy::*;

use crate::state::GameState;
use crate::tilemap::Tilemap;

pub const TILEMAP_ORIGIN: Vec2 = vec2(8.0, 8.0);

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

pub fn get_id_logic(state: &GameState, position: Vec2) -> Option<u32> {
    state.tilemap.get_id_at_position(state.tilemap.get_layer_id("logic"), position)
}

pub fn load_levels() -> Vec<Tilemap> {
    let mut tilemaps = Vec::new();
    let tiles_json_vec = include_bytes!("../../assets/levels/level_1.json").to_vec();
    let tiles_json_vec2 = include_bytes!("../../assets/levels/level_2.json").to_vec();
    let tiles_json_vec3 = include_bytes!("../../assets/levels/level_3.json").to_vec();
    let tiles_json_vec4 = include_bytes!("../../assets/levels/level_4.json").to_vec();
    let tilemap = Tilemap::from_pyxeledit(Rect::from_xywh(0.0, 0.0, 128.0, 128.0), String::from_utf8(tiles_json_vec).unwrap().as_str());
    let tilemap2 = Tilemap::from_pyxeledit(Rect::from_xywh(0.0, 0.0, 128.0, 128.0), String::from_utf8(tiles_json_vec2).unwrap().as_str());
    let tilemap3 = Tilemap::from_pyxeledit(Rect::from_xywh(0.0, 0.0, 128.0, 128.0), String::from_utf8(tiles_json_vec3).unwrap().as_str());
    let tilemap4 = Tilemap::from_pyxeledit(Rect::from_xywh(0.0, 0.0, 128.0, 128.0), String::from_utf8(tiles_json_vec4).unwrap().as_str());
    tilemaps.push(tilemap);
    tilemaps.push(tilemap2);
    tilemaps.push(tilemap3);
    tilemaps.push(tilemap4);
    tilemaps
}
