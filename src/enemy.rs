use crate::draw::{DogSprite, ParrotSprite};
use crate::physics::{Actor, IntRect};
use crate::player::Controller;
use crate::resources::Resources;
use hecs::{Entity, World};
use macroquad::prelude::*;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum EnemyKind {
    Dog,
    JumpyDog,
    SpiderParrot,
}

impl EnemyKind {
    fn jump_prob(&self) -> f32 {
        match self {
            EnemyKind::Dog => 0.45, // lots of small jumps
            EnemyKind::JumpyDog => 0.2,
            EnemyKind::SpiderParrot => 0.0,
        }
    }

    fn jump_vel(&self) -> f32 {
        match self {
            EnemyKind::Dog => -6.0,
            EnemyKind::JumpyDog => -8.0,
            EnemyKind::SpiderParrot => 0.0,
        }
    }
}

pub fn add_enemy(world: &mut World, kind: EnemyKind, x: i32, y: i32) {
    let h = match kind {
        EnemyKind::SpiderParrot => 24,
        _ => 16,
    };
    let rect = IntRect::new(x - 12, y - h, 24, h);
    let actor = Actor::new(&rect, 0.4);
    let enemy = Enemy::new(kind);
    let hittable = EnemyHittable::new(3);
    let dmg = EnemyContactDamage::new();
    if kind == EnemyKind::SpiderParrot {
        world.spawn((
            rect,
            crate::draw::ParrotSprite::new(),
            actor,
            enemy,
            hittable,
            dmg,
        ));
    } else {
        world.spawn((
            rect,
            crate::draw::DogSprite::new(),
            actor,
            enemy,
            hittable,
            dmg,
        ));
    }
}

fn with_prob(p: f32) -> bool {
    quad_rand::gen_range(0.0, 1.0) < p
}

fn rand_sign() -> f32 {
    quad_rand::gen_range(0, 2) as f32 * 2.0 - 1.0
}

fn player_x(world: &World, player_id: Entity) -> Option<f32> {
    world
        .get::<&IntRect>(player_id)
        .map(|rect| rect.centre().x)
        .ok()
}

pub struct EnemyHittable {
    pub hp: u16,
}

impl EnemyHittable {
    pub fn new(hp: u16) -> Self {
        Self { hp }
    }
}

pub struct EnemyContactDamage {}

impl EnemyContactDamage {
    pub fn new() -> Self {
        Self {}
    }
}

pub(crate) struct Enemy {
    kind: EnemyKind,
    dir: f32,
    jump_y: Option<i32>,
}

impl Enemy {
    pub fn new(kind: EnemyKind) -> Self {
        Self {
            kind,
            dir: 0.0,
            jump_y: None,
        }
    }

    pub fn update(world: &World, resources: &Resources) {
        let player_x = player_x(world, resources.player_id);
        // TODO this is super bad, look at all this duplication

        for (_, (actor, enemy, rect, spr)) in world
            .query::<(&mut Actor, &mut Enemy, &IntRect, &mut DogSprite)>()
            .iter()
        {
            if (actor.grounded || enemy.jump_y.is_some()) && with_prob(0.1) {
                if player_x.is_some() && with_prob(0.7) {
                    enemy.dir = (player_x.unwrap() - rect.centre().x).signum() * 5.0;
                } else {
                    enemy.dir = 5.0 * rand_sign();
                }
            }
            if actor.grounded {
                if with_prob(enemy.kind.jump_prob()) {
                    actor.vy = enemy.kind.jump_vel();
                    enemy.jump_y = Some(rect.y);
                } else {
                    enemy.jump_y = None;
                }
            } else {
                // stop moving horizontally if ground has fallen out from under us
                if match enemy.jump_y {
                    None => true,
                    Some(y) => y < rect.y,
                } {
                    enemy.dir = 0.0;
                }
            }
            actor.vx += enemy.dir;
            if actor.vx < 0.0 {
                spr.flipped = false
            }
            if actor.vx > 0.0 {
                spr.flipped = true
            }
            spr.n += 1;
        }

        for (_, (actor, enemy, rect, spr)) in world
            .query::<(&mut Actor, &mut Enemy, &IntRect, &mut ParrotSprite)>()
            .iter()
        {
            if (actor.grounded || enemy.jump_y.is_some()) && with_prob(0.1) {
                if player_x.is_some() && with_prob(0.7) {
                    enemy.dir = (player_x.unwrap() - rect.centre().x).signum() * 5.0;
                } else {
                    enemy.dir = 5.0 * rand_sign();
                }
            }
            if actor.grounded {
                if with_prob(enemy.kind.jump_prob()) {
                    actor.vy = enemy.kind.jump_vel();
                    enemy.jump_y = Some(rect.y);
                } else {
                    enemy.jump_y = None;
                }
            } else {
                // stop moving horizontally if ground has fallen out from under us
                if match enemy.jump_y {
                    None => true,
                    Some(y) => y < rect.y,
                } {
                    enemy.dir = 0.0;
                }
            }
            actor.vx += enemy.dir;
            if actor.vx < 0.0 {
                spr.flipped = false
            }
            if actor.vx > 0.0 {
                spr.flipped = true
            }
            spr.n += 1;
        }

        for (_, (_, rect)) in world.query::<(&EnemyContactDamage, &IntRect)>().iter() {
            if let Ok(mut q) = world.query_one::<(&mut Controller, &IntRect)>(resources.player_id) {
                if let Some((c, p_rect)) = q.get() {
                    if rect.intersects(p_rect) {
                        c.hurt();
                    }
                }
            }
        }
    }
}
