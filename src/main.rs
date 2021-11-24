mod comp;

use macroquad::prelude::*;
use serde::*;

use comp::*;
use Typ::*;

const ZOOM: f32 = 10000.0;
const FRIC_CONST: f32 = 0.9;
const CAM_LERP: f32 = 0.01;

#[macroquad::main("esk")]
async fn main() {
    let mut state = CliState::default();

    use crate::El::*;

    let player = Thing { 
        typ: PLAYER, 
        stat: Stat { 
            el: LAND, 
            el_deg: 1.0, 
            hp: 10.0, 
            mag: 1, 
            spr: 1, 
            vit: 1, 
            atk: 1, 
            spd:1, 
        }, 
        rig: Rig {
            mass: 200.0,
            max_speed: 50.0,
            ..Rig::default()
        },
        ..Thing::default()
    };

    let knife = Thing { 
        typ: KNIFE, 
        stat: Stat { 
            el: LAND, 
            el_deg: 1.0, 
            hp: 10.0, 
            mag: 1, 
            spr: 1, 
            vit: 1, 
            atk: 1, 
            spd:1, 
        }, 
        rig: Rig {
            mass: 200.0,
            max_speed: 50.0,
            ..Rig::default()
        },
        ..Thing::default()
    };

    state.player = state.world.len();

    state.world.push(player);
    state.world.push(knife);

    loop {
        clear_background(WHITE);

        //controls process

        state.cntrl = Cntrl::new();
        state.ms_clock = get_time() * 1000.0;

        state.world[state.player].rig.dir.y = 0.0;
        state.world[state.player].rig.dir.x = 0.0;

        if state.cntrl.forward {
            state.world[state.player].rig.dir.y -= 1.0;
        }
        if state.cntrl.backward {
            state.world[state.player].rig.dir.y += 1.0;
        }
        if state.cntrl.left {
            state.world[state.player].rig.dir.x -= 1.0;
        }
        if state.cntrl.right {
            state.world[state.player].rig.dir.x += 1.0;
        }

        if state.cntrl.forward || state.cntrl.backward || state.cntrl.left || state.cntrl.right {
            state.world[state.player].rig.trans_acl += state.world[state.player].stat.spd as f32 / 222.125;
        }

        state.world[state.player].rig.trans_acl -= FRIC_CONST / state.world[state.player].rig.mass;

        //apply friction

        //physics process

        for thing in &mut state.world {
            thing.rig.tick(state.ms_clock as f32);
        }

        //render process

        //move cam tgt

        state.cam_tgt = state.cam_tgt.lerp(state.world[state.player].rig.pos, CAM_LERP);

        set_camera(&Camera2D {
            zoom: macroquad::math::vec2(1.0, -screen_width() / screen_height()) / ZOOM,
            target: macroquad::math::vec2(state.cam_tgt.x, state.cam_tgt.y),
            ..Default::default()
        });
        for thing in &state.world {
            match thing.typ {
                PLAYER => draw_circle(thing.rig.pos.x, thing.rig.pos.y, 250.0, BLACK),
                KNIFE => draw_circle(thing.rig.pos.x, thing.rig.pos.y, 125.0, BROWN),
                SPITTER => draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, BLUE),
            }
        }

        //println!("trans speed  {:?}", state.world[state.player].rig.trans_speed);

        //println!("x:    {:?}", state.world[state.player].rig.pos.x);
        //println!("y:    {:?}", state.world[state.player].rig.pos.y);

        set_default_camera();

        next_frame().await
    }
}

#[derive(Debug, Default)]
struct CliState {
    pub world: Vec<Thing>,
    pub player: usize,
    pub cntrl: Cntrl,
    pub cam_tgt: Vec2,
    pub ms_clock: f64,
}

#[derive(Debug, Default)]
pub struct Cntrl {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    rotl: bool,
    rotr: bool,
    pickup: bool,
    drop: bool,
    fire1_released: bool,
    fire2_released: bool,
    fire3_released: bool,
    fire1_down: bool,
    fire2_down: bool,
    fire3_down: bool,
    mouse: Vec2,
}

impl Cntrl {
    pub fn new() -> Self {
        use KeyCode::*;
        Cntrl {
            forward: is_key_down(W),
            backward: is_key_down(S),
            left: is_key_down(A),
            right: is_key_down(D),
            rotl: is_key_down(Q),
            rotr: is_key_down(E),
            pickup: is_key_down(F),
            drop: is_key_down(G),
            fire1_released: is_mouse_button_released(MouseButton::Left),
            fire2_released: is_mouse_button_released(MouseButton::Right),
            fire3_released: is_mouse_button_released(MouseButton::Middle),
            fire1_down: is_mouse_button_down(MouseButton::Left),
            fire2_down: is_mouse_button_down(MouseButton::Right),
            fire3_down: is_mouse_button_down(MouseButton::Middle),
            mouse: mouse_position_local(),
        }
    }
}

fn vec2_to_tup(x: Vec2) -> (f32, f32) { x.into() }
