use comfy::*;
use crate::state::GameState;
pub struct Item;

pub fn spawn_ladder(pos: Vec2){
    commands().spawn((
        Transform::position(pos),
        Item,
        get_tween(),
        AnimatedSpriteBuilder::new()
            .z_index(10)
            .add_animation("idle", 0.1, true, AnimationSource::Atlas{
                name: "sprites".into(),
                offset: ivec2(0,0),
                step: ivec2(1, 0),
                size: isplat(16),
                frames: 1,
            })
            .size(splat(16.0))
            .blend_mode(BlendMode::Alpha)
            .build(),
    ));
}

pub fn spawn_pulley(pos: Vec2){
    commands().spawn((
        Transform::position(pos),
        Item,
        get_tween(),
        AnimatedSpriteBuilder::new()
            .z_index(10)
            .add_animation("idle", 0.1, true, AnimationSource::Atlas{
                name: "sprites".into(),
                offset: ivec2(16,0),
                step: ivec2(1, 0),
                size: isplat(16),
                frames: 1,
            })
            .size(splat(16.0))
            .blend_mode(BlendMode::Alpha)
            .build(),
    ));
}

pub fn spawn_key(pos: Vec2){
    commands().spawn((
        Transform::position(pos),
        Item,
        get_tween(),
        AnimatedSpriteBuilder::new()
            .z_index(10)
            .add_animation("idle", 0.1, true, AnimationSource::Atlas{
                name: "sprites".into(),
                offset: ivec2(64,0),
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

        transform.position += vec2(0.0,tween.value())
    }
}

pub fn get_tween()-> Tween{
    Tween::new(-0.1, 0.1, random_range(3.0,4.0), 0.0, roundtrip)
}