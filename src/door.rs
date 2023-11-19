use comfy::*;
use crate::state::GameState;

pub struct Door;

pub fn spawn(pos: Vec2){
    commands().spawn((
        Transform::position(pos),
        Door,
        AnimatedSpriteBuilder::new()
            .z_index(4)
            .add_animation("open", 0.1, false, AnimationSource::Spritesheet{
                name: "door".into(),
                spritesheet: Spritesheet { rows: 1, columns: 8 },
            })
            .size(vec2(16.0,32.0))
            .blend_mode(BlendMode::Alpha)
            .build(),
    ));
}

pub fn update(_state: &mut GameState, c: &mut EngineContext) {
    let _dt = c.delta;

    for (_, (_, animated_sprite, )) in
    world().query::<(&Door, &mut AnimatedSprite)>().iter()
    {
        animated_sprite.play("open");
        animated_sprite.state.timer = 0.0;
    }
}
