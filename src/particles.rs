use comfy::*;
use crate::state::GameState;

const PARTICLE_RIGHT: u32 = 54;
const PARTICLE_LEFT: u32 = 55;
const PARTICLE_UP: u32 = 62;
const PARTICLE_DOWN: u32 = 63;

pub fn spawn(state: &mut GameState) {
        // right
        let positions = state.tilemap.get_all_position_from_id(state.tilemap.get_layer_id("logic"), &PARTICLE_RIGHT);
        for p in positions{
            commands().spawn((
                ParticleSystem::with_spawn_rate(100, 4.0, move|| {
                    Particle {
                        texture: texture_id("particle"),
                        position: random_circle(8.0),
                        size: splat(4.0),
                        direction: vec2(1.0,0.0),
                        velocity: 4.0,
                        velocity_end: 4.0,
                        lifetime_max: 5.0,
                        color_curve: quad_in_out,
                        color_start: YELLOW,
                        ..Default::default()
                    }
                }),
                Transform::position(p + vec2(6.0,-8.0)),
            ));
        }
    // left
    let positions = state.tilemap.get_all_position_from_id(state.tilemap.get_layer_id("logic"), &PARTICLE_LEFT);
    for p in positions{
        commands().spawn((
            ParticleSystem::with_spawn_rate(100, 4.0, move|| {
                Particle {
                    texture: texture_id("particle"),
                    position: random_circle(8.0),
                    size: splat(4.0),
                    direction: vec2(-1.0,0.0),
                    velocity: 4.0,
                    velocity_end: 4.0,
                    lifetime_max: 5.0,
                    color_curve: quad_in_out,
                    color_start: YELLOW,
                    ..Default::default()
                }
            }),
            Transform::position(p + vec2(12.0,-8.0)),
        ));
    }
}