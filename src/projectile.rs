use crate::enemy::EnemyHittable;
use crate::enemy::EnemyKind;
use crate::physics::collide_any;
use crate::physics::IntRect;
use crate::resources::Resources;
use crate::vfx::create_explosion;
use crate::vfx::ZapFlash;
use hecs::{CommandBuffer, World};

pub struct Projectile {
    prec_x: f32,
    prec_y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Projectile {
    pub fn new(rect: &IntRect, vx: f32, vy: f32) -> Self {
        Self {
            prec_x: rect.x as f32,
            prec_y: rect.y as f32,
            vx,
            vy,
        }
    }

    pub fn update(world: &World, resources: &mut Resources, buffer: &mut CommandBuffer) {
        for (e, (proj, rect)) in world.query::<(&mut Projectile, &mut IntRect)>().iter() {
            let ox = rect.x;
            let oy = rect.y;
            proj.prec_x += proj.vx;
            proj.prec_y += proj.vy;
            rect.x = proj.prec_x.round() as i32;
            rect.y = proj.prec_y.round() as i32;
            if collide_any(world, &resources.body_index, rect) {
                buffer.despawn(e);
                let (x, y) = find_collision_pos(world, resources, ox, oy, rect);
                let sx = if proj.vx > 0.0 { x + 7 } else { x };
                buffer.spawn((ZapFlash::new_from_centre(sx, y + 2),));
            }
            let mut live = true;
            world
                .query::<(&EnemyKind, &mut EnemyHittable, &IntRect)>()
                .iter()
                .for_each(|(e_id, (kind, en, e_rect))| {
                    if live && en.hp > 0 && rect.intersects(&e_rect) {
                        buffer.despawn(e);
                        let sx = if proj.vx > 0.0 { rect.x + 7 } else { rect.x };
                        buffer.spawn((ZapFlash::new_from_centre(sx, rect.y + 2),));
                        en.hp -= 1;
                        if en.hp <= 0 {
                            match kind {
                                EnemyKind::Dog | EnemyKind::JumpyDog => {
                                    resources.messages.add("Destroyed a hound.".to_owned())
                                }
                                EnemyKind::SpiderParrot => {
                                    resources.messages.add("Destroyed a scuttler.".to_owned())
                                }
                            }
                            buffer.despawn(e_id);
                            let (ex, ey) = e_rect.centre_int();
                            create_explosion(buffer, ex, ey);
                            resources.stats.kills += 1
                        }
                        live = false;
                    }
                });
        }
    }
}

fn find_collision_pos(
    world: &World,
    resources: &Resources,
    ox: i32,
    oy: i32,
    rect: &IntRect,
) -> (i32, i32) {
    // this function can be slow as it's only called to generate the vfx when a projectile hits a wall
    let mut r = rect.clone();
    let dx = (ox - r.x).signum();
    while r.x != ox {
        r.x += dx;
        if !collide_any(world, &resources.body_index, &r) {
            return (r.x, r.y);
        }
    }
    let dy = (oy - r.y).signum();
    while r.y != oy {
        r.y += dy;
        if !collide_any(world, &resources.body_index, &r) {
            return (r.x, r.y);
        }
    }
    (r.x, r.y)
}
