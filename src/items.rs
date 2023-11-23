use comfy::*;

use crate::state::GameState;
use crate::tilemap::tilemap_helper::{get_id, get_id_logic, TILEMAP_ORIGIN};

const ID_LADDER: u32 = 24;
const ID_PULLEY: u32 = 25;
const ID_KEY: u32 = 26;

pub struct Item;
pub struct Key;
pub struct Ladder;
pub struct Pulley;
pub struct Identifier { x: i32, y: i32, }


pub fn spawn_ladders(state: &mut GameState) {
    let positions = state.tilemap.get_all_position_from_id(state.tilemap.get_layer_id("logic"), ID_LADDER);
    for p in positions {
        spawn_ladder(p + TILEMAP_ORIGIN + vec2(0.0, -8.0));
    }
}

pub fn spawn_ladder(pos: Vec2) {
    commands().spawn((
        Transform::position(pos),
        Item,
        Ladder,
        get_identifier(pos),
        get_tween(),
        AnimatedSpriteBuilder::new()
            .z_index(10)
            .add_animation("idle", 0.1, true, AnimationSource::Atlas {
                name: "sprites".into(),
                offset: ivec2(0, 0),
                step: ivec2(1, 0),
                size: isplat(16),
                frames: 1,
            })
            .size(splat(16.0))
            .blend_mode(BlendMode::Alpha)
            .build(),
    ));
}

fn get_identifier(pos: Vec2) -> Identifier {
    let x = ((pos.x - pos.x % 16.0) / 16.0) as i32;
    let y = ((pos.y - pos.y % 16.0) / 16.0) as i32;
    Identifier{x, y}
}

pub fn spawn_pulleys(state: &mut GameState) {
    let positions = state.tilemap.get_all_position_from_id(state.tilemap.get_layer_id("logic"), ID_PULLEY);
    for p in positions {
        spawn_pulley(p + TILEMAP_ORIGIN + vec2(0.0, -8.0));
    }
}

pub fn spawn_pulley(pos: Vec2) {
    commands().spawn((
        Transform::position(pos),
        Item,
        Pulley,
        get_tween(),
        AnimatedSpriteBuilder::new()
            .z_index(10)
            .add_animation("idle", 0.1, true, AnimationSource::Atlas {
                name: "sprites".into(),
                offset: ivec2(16, 0),
                step: ivec2(1, 0),
                size: isplat(16),
                frames: 1,
            })
            .size(splat(16.0))
            .blend_mode(BlendMode::Alpha)
            .build(),
    ));
}

pub fn spawn_keys(state: &mut GameState) {
    let positions = state.tilemap.get_all_position_from_id(state.tilemap.get_layer_id("logic"), ID_KEY);
    for p in positions {
        spawn_key(p + TILEMAP_ORIGIN + vec2(0.0, -8.0));
    }
}

pub fn spawn_key(pos: Vec2) {
    commands().spawn((
        Transform::position(pos),
        Item,
        Key,
        get_tween(),
        AnimatedSpriteBuilder::new()
            .z_index(10)
            .add_animation("idle", 0.1, true, AnimationSource::Atlas {
                name: "sprites".into(),
                offset: ivec2(64, 0),
                step: ivec2(1, 0),
                size: isplat(16),
                frames: 1,
            })
            .size(splat(16.0))
            .blend_mode(BlendMode::Alpha)
            .build(),
    ));
}

pub fn update(_state: &mut GameState, c: &mut EngineContext) {
    let dt = c.delta;

    for (_, (_, animated_sprite, transform, tween)) in
    world().query::<(&Item, &mut AnimatedSprite, &mut Transform, &mut Tween)>().iter()
    {
        animated_sprite.play("idle");
        tween.update(dt);
        if tween.is_finished() {
            *tween = get_tween();
        }

        transform.position += vec2(0.0, tween.value())
    }
}

pub fn get_tween() -> Tween {
    Tween::new(-0.1, 0.1, random_range(3.0, 4.0), 0.0, roundtrip)
}

pub(crate) fn pickup(state: &mut GameState, player_pos: &Vec2) {
    let id = get_id_logic(state, *player_pos);
    if id == Some(ID_KEY){
        state.has_key = true;
        for (entity, (_, transform, _)) in
        world().query::<(&Item, &mut Transform, &mut Key)>().iter()
        {
            commands().despawn(entity);
        }
    }
    if id== Some(ID_LADDER){
        let identifier = get_identifier(*player_pos);
        for (entity, (_, id, _)) in
        world().query::<(&Item, &mut Identifier, &mut Ladder)>().iter()
        {
            if id.x == identifier.x && id.y == identifier.y {
                commands().despawn(entity);
                state.pickup_ladder();
            }

        }
    }
}