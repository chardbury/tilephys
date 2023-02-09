use crate::input::KeyState;
use crate::physics::{Actor, IntRect};
use crate::projectile::make_player_projectile;
use hecs::CommandBuffer;

// eventually there will be variants whose names don't end in "...Laser"
#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum WeaponType {
    BackupLaser,
    BurstLaser,
    AutoLaser,
    DoubleLaser,
    Shotgun,
    SuperShotgun,
    ReverseShotgun,
}

pub fn weapon_name(typ: WeaponType) -> &'static str {
    match typ {
        WeaponType::BackupLaser => "backup laser",
        WeaponType::AutoLaser => "auto-laser",
        WeaponType::BurstLaser => "burst laser",
        WeaponType::DoubleLaser => "double laser",
        WeaponType::Shotgun => "shotgun",
        WeaponType::SuperShotgun => "super shotgun",
        WeaponType::ReverseShotgun => "reverse shotgun",
    }
}

pub fn weapon_name_indef(typ: WeaponType) -> &'static str {
    match typ {
        WeaponType::BackupLaser => unreachable!(),
        WeaponType::AutoLaser => "an auto-laser",
        WeaponType::BurstLaser => "a burst laser",
        WeaponType::DoubleLaser => "a double laser",
        WeaponType::Shotgun => "a shotgun",
        WeaponType::SuperShotgun => "a super shotgun",
        WeaponType::ReverseShotgun => "the reverse shotgun",
    }
}

pub fn weapon_sprite_frame(typ: WeaponType) -> usize {
    match typ {
        WeaponType::BackupLaser => 0,
        WeaponType::AutoLaser => 2,
        WeaponType::BurstLaser => 1,
        WeaponType::DoubleLaser => 7,
        WeaponType::Shotgun => 3,
        WeaponType::SuperShotgun => 4,
        WeaponType::ReverseShotgun => 5,
    }
}

pub fn weapon_v_offset(typ: WeaponType) -> f32 {
    match typ {
        WeaponType::BackupLaser => 4.0,
        WeaponType::AutoLaser => 3.0,
        WeaponType::BurstLaser => 3.0,
        WeaponType::DoubleLaser => 2.0,
        WeaponType::Shotgun => 4.0,
        WeaponType::SuperShotgun => 3.0,
        WeaponType::ReverseShotgun => 1.0,
    }
}

pub trait Weapon {
    fn get_type(&self) -> WeaponType;
    fn update(
        &mut self,
        buffer: &mut CommandBuffer,
        player: &mut Actor,
        player_rect: &IntRect,
        facing: i8,
        key_state: KeyState,
    ) -> bool;
}

struct BackupLaser {}

impl BackupLaser {
    fn new() -> Self {
        Self {}
    }
}

impl Weapon for BackupLaser {
    fn get_type(&self) -> WeaponType {
        WeaponType::BackupLaser
    }
    fn update(
        &mut self,
        buffer: &mut CommandBuffer,
        player: &mut Actor,
        player_rect: &IntRect,
        facing: i8,
        key_state: KeyState,
    ) -> bool {
        if key_state == KeyState::Pressed {
            let new_x = player_rect.x + 3 + facing as i32 * 9;
            let rect = IntRect::new(new_x, player_rect.y + 11, 8, 5);
            make_player_projectile(buffer, rect, facing as f32 * 10.0);
            player.vx -= facing as f32 * 10.0;
            return true;
        }
        false
    }
}

struct Shotgun {}

impl Shotgun {
    fn new() -> Self {
        Self {}
    }
}

impl Weapon for Shotgun {
    fn get_type(&self) -> WeaponType {
        WeaponType::Shotgun
    }
    fn update(
        &mut self,
        buffer: &mut CommandBuffer,
        player: &mut Actor,
        player_rect: &IntRect,
        facing: i8,
        key_state: KeyState,
    ) -> bool {
        if key_state == KeyState::Pressed {
            let new_x = player_rect.x + 3 + facing as i32 * 9;
            let rect = IntRect::new(new_x, player_rect.y + 11, 8, 5);
            make_player_projectile(buffer, rect, facing as f32 * 10.0);
            player.vx -= facing as f32 * 10.0;
            return true;
        }
        false
    }
}
struct SuperShotgun {}

impl SuperShotgun {
    fn new() -> Self {
        Self {}
    }
}

impl Weapon for SuperShotgun {
    fn get_type(&self) -> WeaponType {
        WeaponType::SuperShotgun
    }
    fn update(
        &mut self,
        buffer: &mut CommandBuffer,
        player: &mut Actor,
        player_rect: &IntRect,
        facing: i8,
        key_state: KeyState,
    ) -> bool {
        if key_state == KeyState::Pressed {
            let new_x = player_rect.x + 3 + facing as i32 * 9;
            let rect = IntRect::new(new_x, player_rect.y + 11, 8, 5);
            make_player_projectile(buffer, rect, facing as f32 * 10.0);
            player.vx -= facing as f32 * 10.0;
            return true;
        }
        false
    }
}

struct ReverseShotgun {}

impl ReverseShotgun {
    fn new() -> Self {
        Self {}
    }
}

impl Weapon for ReverseShotgun {
    fn get_type(&self) -> WeaponType {
        WeaponType::ReverseShotgun
    }
    fn update(
        &mut self,
        buffer: &mut CommandBuffer,
        player: &mut Actor,
        player_rect: &IntRect,
        facing: i8,
        key_state: KeyState,
    ) -> bool {
        if key_state == KeyState::Pressed {
            let new_x = player_rect.x + 3 - facing as i32 * 9;
            let rect = IntRect::new(new_x, player_rect.y + 11, 8, 5);
            make_player_projectile(buffer, rect, facing as f32 * -10.0);
            player.vx += facing as f32 * 10.0;
            return true;
        }
        false
    }
}

struct AutoLaser {
    delay: u8,
}

impl AutoLaser {
    fn new() -> Self {
        Self { delay: 0 }
    }
}

impl Weapon for AutoLaser {
    fn get_type(&self) -> WeaponType {
        WeaponType::AutoLaser
    }
    fn update(
        &mut self,
        buffer: &mut CommandBuffer,
        player: &mut Actor,
        player_rect: &IntRect,
        facing: i8,
        key_state: KeyState,
    ) -> bool {
        if self.delay > 0 {
            self.delay -= 1
        }
        if key_state != KeyState::NotPressed && self.delay == 0 {
            let new_x = player_rect.x + 3 + facing as i32 * 9;
            let rect = IntRect::new(new_x, player_rect.y + 11, 8, 5);
            make_player_projectile(buffer, rect, facing as f32 * 10.0);
            player.vx -= facing as f32 * 10.0;
            self.delay = 3;
            return true;
        }
        false
    }
}

struct BurstLaser {
    delay: u8,
    shots: u8,
}

impl BurstLaser {
    fn new() -> Self {
        Self { delay: 0, shots: 0 }
    }
}

impl Weapon for BurstLaser {
    fn get_type(&self) -> WeaponType {
        WeaponType::BurstLaser
    }
    fn update(
        &mut self,
        buffer: &mut CommandBuffer,
        player: &mut Actor,
        player_rect: &IntRect,
        facing: i8,
        key_state: KeyState,
    ) -> bool {
        if self.delay > 0 {
            self.delay -= 1
        }
        if key_state != KeyState::NotPressed && self.delay == 0 && self.shots < 3 {
            let new_x = player_rect.x + 3 + facing as i32 * 9;
            let rect = IntRect::new(new_x, player_rect.y + 11, 8, 5);
            make_player_projectile(buffer, rect, facing as f32 * 10.0);
            player.vx -= facing as f32 * 10.0;
            self.delay = 2;
            self.shots += 1;
            return true;
        }
        if key_state == KeyState::NotPressed {
            self.shots = 0;
        }
        false
    }
}

struct DoubleLaser {}

impl DoubleLaser {
    fn new() -> Self {
        Self {}
    }
}

impl Weapon for DoubleLaser {
    fn get_type(&self) -> WeaponType {
        WeaponType::DoubleLaser
    }
    fn update(
        &mut self,
        buffer: &mut CommandBuffer,
        player: &mut Actor,
        player_rect: &IntRect,
        facing: i8,
        key_state: KeyState,
    ) -> bool {
        if key_state == KeyState::Pressed {
            let new_x = player_rect.x + 3 + facing as i32 * 9;
            let rect = IntRect::new(new_x, player_rect.y + 8, 8, 5);
            make_player_projectile(buffer, rect, facing as f32 * 10.0);
            let rect = IntRect::new(new_x, player_rect.y + 14, 8, 5);
            make_player_projectile(buffer, rect, facing as f32 * 10.0);
            player.vx -= facing as f32 * 10.0;
            return true;
        }
        false
    }
}

pub fn new_weapon(typ: WeaponType) -> Box<dyn Weapon> {
    match typ {
        WeaponType::BackupLaser => Box::new(BackupLaser::new()),
        WeaponType::AutoLaser => Box::new(AutoLaser::new()),
        WeaponType::BurstLaser => Box::new(BurstLaser::new()),
        WeaponType::DoubleLaser => Box::new(DoubleLaser::new()),
        WeaponType::Shotgun => Box::new(Shotgun::new()),
        WeaponType::SuperShotgun => Box::new(SuperShotgun::new()),
        WeaponType::ReverseShotgun => Box::new(ReverseShotgun::new()),
    }
}

pub struct WeaponSelectorUI {
    pub timer: u16,
    pub offset: f32,
    pub hidden: bool,
}

impl WeaponSelectorUI {
    pub fn new() -> Self {
        Self {
            timer: 0,
            offset: 0.0,
            hidden: false,
        }
    }

    pub fn change(&mut self, delta: f32) {
        self.timer = 45;
        self.offset += delta;
    }

    pub fn update(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
        self.offset *= 0.8;
    }
}
