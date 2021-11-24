use std::rc::Rc;
use std::cell::RefCell;
use serde::{Deserialize, ser, Serialize};
use macroquad::prelude::*;

/*comp
    Stat
*/
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Stat {
    pub el: El,
    //multiplying or dividing factor when attacking or defending depending upon elements involved
    pub el_deg: f32,
    pub hp: f32,
    //mag and spr create a damage multiplier of 1 when identical, and should be similar in closely powered enemies
    pub mag: u32,
    pub spr: u32,
    //vit and atk create a damage multiplier of 1 when identical, and should be similar in closely powered enemies
    pub vit: u32,
    pub atk: u32,
    pub spd: u32,
}

impl Stat {
    pub fn hit(&mut self, other: &mut Stat, mag: bool, dmg_mod: f32) {
        //subtract hp of other (the entity getting hit) by total damage of self (the attacker)
        other.hp -= (({
            //calculate magic or physical multiplier (with similar strength entities should be relatively close to 1)
            if mag {
                self.mag / other.spr
            } else {
                self.atk / other.vit
            }
        }) as f32 * {
                    //if opponent weak against attack, multiply by own elemental degree
                    if other.el.is_weak(self.el) {
                        self.el_deg
                    //if opponent is strong against attack, divide by opponents elemental degree
                    } else {
                        1.0 / other.el_deg
                    }
        }) * /* multiply by special damage modifier (this should be zero and in effect do nothing under normal circumstances) */ dmg_mod;
    }
}

/*comp
    El
*/
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum El {
    LAND,
    SEA,
    SKY
}

impl Default for El {
    fn default() -> Self {
        El::LAND
    }
}

impl El {
    pub fn is_weak(self, other: Self) -> bool {
        ((self as u8 + (other as u8 - 1)) % 3) > 1
    }

    pub fn to_string(&self) -> &str {
        use El::*;
        match &self {
            LAND => "🜃",
            SKY => "🜁",
            SEA => "🜄",
        }
    }
}

/*comp
    Rig
*/
pub const Z_PUSH_REQ: f32 = 100.0;
///A struct that describes the state of an objects movement,
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Rig {
    pub mass: f32,
    //position vector
    pub pos: Vec2,
    //z plane
    pub z: u8,
    //rotation
    pub rot: f32,
    //speed
    pub max_speed: f32,
    pub rot_speed: f32,
    pub trans_speed: f32,
    pub z_push: f32,
    //acceleration
    pub rot_acl: f32,
    pub trans_acl: f32,
    pub z_acl: f32,
    ///vector that represents the current direction
    pub dir: Vec2,
    //radius of collisions
    pub rad: f32,
    //tether to another two rigs
    pub tethers: [Option<Rc<Rig>>; 2],
    //tether lerp amount
    pub tether_lerp: f32,
}

impl Rig {
    ///Exectes one physics tick for a Rig
    pub fn tick(&mut self, tick_amnt: f32) {
        println!("tick amnt {:?}", tick_amnt);
        //ensure no negative acceleration
        if self.z_acl < 0.0 {
            self.z_acl = 0.0;
        }

        if self.rot_acl < 0.0 {
            self.rot_acl = 0.0;
        }

        //accelerate
        self.rot_speed += self.rot_acl * tick_amnt;
        self.trans_speed += self.trans_acl * tick_amnt;
        self.z_push += self.z_acl * tick_amnt;

        //make sure the speed is not below zero
        if self.trans_speed < 0.0 {
            self.trans_speed = 0.0;
        }
        if self.trans_speed > self.max_speed {
            self.trans_speed = self.max_speed;
        }
        //or the trans acl
        if self.trans_acl < 0.0 {
            self.trans_acl = 0.0;
        }

        if self.z_push < 0.0 {
            self.z_push = 0.0;
        }

        if self.rot_speed < 0.0 {
            self.rot_speed = 0.0;
        }

        //initialize delta vec
        let mut d_vec = self.dir;

        d_vec = glam::vec2(d_vec.x * self.trans_speed, d_vec.y * self.trans_speed);

        //transform by delta vec
        self.transform(d_vec, 0);

        //lerp by tethers
        for tether in &mut self.tethers {
            match tether {
                Some(rig) => Rc::<Rig>::make_mut(rig).lerp(self.pos, self.tether_lerp),
                _ => return ,
            }
        }
    }

    pub fn lerp(&mut self, _other: Vec2, _amnt: f32) {
        unimplemented!()
    }
    pub fn transform(&mut self, d_vec: Vec2, d_z: u8) {
        self.pos += d_vec;
        self.z += d_z;
    }

    pub fn rotate_toward(&mut self, toward: Vec2) {
        let point_from_origin = Vec2::new(self.pos.x - toward.x, self.pos.y - toward.y);
        self.rot = point_from_origin.y.atan2(point_from_origin.x);
    }
}

/*comp
    Inf
*/
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Inf {
    pub qtty: u8,
    pub spawned: bool,
    pub vis: bool,
    pub col: bool,
    pub phys: bool,
    pub grab: bool,
}

/*comp
    Inv
*/
#[derive(Debug, Clone)]
pub struct Inv {
    pub items: Vec<Thing>,
    pub len: u8,
}

impl Default for Inv {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            len: 100,
        }
    }
}

/*comp
    Typ
*/
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Typ {
    PLAYER,
    KNIFE,
    SPITTER,
}

impl Default for Typ {
    fn default() -> Self {
        use Typ::*;
        PLAYER
    }
}

#[derive(Debug, Clone, Default)]
pub struct Thing {
    pub typ: Typ,
    pub held: Inv,
    pub equip: Inv,
    pub inf: Inf,
    pub stat: Stat,
    pub rig: Rig,
    pub col: f32,
}

#[derive(Debug, Serialize)]
pub struct It {
    typ: Typ,
    inf: Inf,
    stat: Stat,
    rig: Rig,
    col: f32,
}

impl From<Thing> for It {
    fn from(thing: Thing) -> Self {
        It {
            typ: thing.typ,
            inf: thing.inf,
            stat: thing.stat,
            rig: thing.rig,
            col: thing.col,
        }
    }
}