use crate::index::SpatialIndex;
use crate::level::{load_level_info, LevelInfo};
use crate::messages::Messages;
use crate::render::load_flash_material;
use crate::scene::Scene;
use crate::script::ScriptEngine;
use crate::stats::LevelStats;
use crate::transition::TransitionEffectType;
use crate::weapon::{
    ammo_max, select_fireable_weapon, AmmoQuantity, AmmoType, Weapon, WeaponSelectorUI, WeaponType,
};
use enum_map::EnumMap;
use hecs::{Entity, World};
use macroquad::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::num::NonZeroU8;
use std::sync::{Arc, Mutex};

pub struct GlobalAssets {
    pub sky: Texture2D,
    pub player_sprite: Texture2D,
    pub dog_sprite: Texture2D,
    pub parrot_sprite: Texture2D,
    pub parrot_sprite2: Texture2D,
    pub pickup_sprite: Texture2D,
    pub switch_sprite: Texture2D,
    pub ui_sprite: Texture2D,
    pub weapon_sprite: Texture2D,
    pub zap_sprite: Texture2D,
    pub interstitial: Texture2D,
    pub flash_material: Material,
    pub levels: Vec<LevelInfo>,
    // should this be here?
    pub next_scene: Option<(Scene, TransitionEffectType)>,
}

pub async fn load_assets() -> GlobalAssets {
    let levels = load_level_info().await;
    GlobalAssets {
        sky: load_texture("sky.png").await.unwrap(),
        player_sprite: load_texture("princess.png").await.unwrap(),
        dog_sprite: load_texture("robodog.png").await.unwrap(),
        parrot_sprite: load_texture("spiderparrot.png").await.unwrap(),
        parrot_sprite2: load_texture("greenparrot.png").await.unwrap(),
        pickup_sprite: load_texture("pickup.png").await.unwrap(),
        switch_sprite: load_texture("switch.png").await.unwrap(),
        ui_sprite: load_texture("ui-heart.png").await.unwrap(),
        weapon_sprite: load_texture("weapons.png").await.unwrap(),
        zap_sprite: load_texture("zap.png").await.unwrap(),
        interstitial: load_texture("interstitial.png").await.unwrap(),
        flash_material: load_flash_material(),
        levels,
        next_scene: None,
    }
}

pub struct SceneResources {
    pub world_ref: Arc<Mutex<World>>,
    pub script_engine: ScriptEngine,
    pub player_id: Entity,
    pub eye_pos: Vec2,
    pub camera_pos: Vec2,
    pub death_timer: Option<NonZeroU8>,
    pub draw_order: Vec<Entity>,
    pub body_index: SpatialIndex,
    pub tileset_info: TilesetInfo,
    pub messages: Messages,
    pub selector: WeaponSelectorUI,
    pub stats: LevelStats,
    pub triggers: HashSet<String>,
    pub weapons: VecDeque<Box<dyn Weapon>>,
    pub ammo: EnumMap<AmmoType, u8>,
}

impl SceneResources {
    pub fn add_ammo(&mut self, typ: AmmoType, amt: AmmoQuantity) {
        self.ammo[typ] = (self.ammo[typ] + amt).min(ammo_max(typ));
        if let Some(n) = self
            .weapons
            .iter()
            .position(|w| w.get_type() == WeaponType::BackupLaser)
        {
            // there is a backup laser in inventory at position n
            // we should remove it if the player can now use anything else
            if self.weapons.iter().any(|w| {
                self.ammo[w.get_ammo_type()] >= w.get_ammo_use()
                    && w.get_type() != WeaponType::BackupLaser
            }) {
                self.weapons.remove(n);
                if n == 0 {
                    // backup laser was previously the selected weapon
                    select_fireable_weapon(&mut self.weapons, &mut self.ammo, &mut self.selector)
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct TilesetInfo {
    pub texture: Texture2D,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
}
