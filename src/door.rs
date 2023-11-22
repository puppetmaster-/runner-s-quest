use comfy::*;

use crate::state::GameState;

use crate::tilemap::tilemap_helper::{get_id_logic, TILEMAP_ORIGIN};

const ID_DOOR: u32 = 11;

pub struct Door;
pub fn spawns(state: &mut GameState) {
    let door_pos = state.tilemap.get_all_position_from_id(state.tilemap.get_layer_id("logic"), ID_DOOR);
    spawn(door_pos[0] + TILEMAP_ORIGIN + vec2(0.0, -8.0));
}

pub fn spawn(pos: Vec2) {
    commands().spawn((
        Transform::position(pos),
        Door,
        AnimatedSpriteBuilder::new()
            .z_index(4)
            .add_animation("close", 0.2, false, AnimationSource::Spritesheet {
                name: "door".into(),
                spritesheet: Spritesheet { rows: 1, columns: 8 },
            })
            .add_animation("open", 0.2, false, AnimationSource::Spritesheet {
                name: "door".into(),
                spritesheet: Spritesheet { rows: 1, columns: 8 },
            })
            .size(vec2(16.0, 32.0))
            .blend_mode(BlendMode::Alpha)
            .build(),
    ));
}

pub fn update(state: &mut GameState, c: &mut EngineContext) {
    let _dt = c.delta;

    for (_, (_, animated_sprite, )) in
    world().query::<(&Door, &mut AnimatedSprite)>().iter()
    {
        if !state.has_key {
            animated_sprite.play("close");
            animated_sprite.state.timer = 0.0;
        } else {
            animated_sprite.play("open");
        }
    }
}

pub(crate) fn exit(state: &mut GameState, player_pos: &Vec2) {
    let id = get_id_logic(state, *player_pos);
    if id == Some(ID_DOOR) {
        state.exit_level();
    }
}