pub mod settings;
pub mod orbs;

use std::{fs, io, path::PathBuf};

use serde::{Serialize, Deserialize};

use crate::inventory::Item;

pub const DEFAULTSPAWN: &str = "MarshSpawn.Main";
pub const MOKI_SPAWNS: &[&str] = &[
    "MarshSpawn.Main",
    "HowlsDen.Teleporter",
    "GladesTown.Teleporter",
    "InnerWellspring.Teleporter",
    "MidnightBurrows.Teleporter",
];
pub const GORLEK_SPAWNS: &[&str] = &[
    "MarshSpawn.Main",
    "HowlsDen.Teleporter",
    "EastHollow.Teleporter",
    "GladesTown.Teleporter",
    "InnerWellspring.Teleporter",
    "MidnightBurrows.Teleporter",
    "WoodsEntry.Teleporter",
    "WoodsMain.Teleporter",
    "LowerReach.Teleporter",
    "UpperDepths.Teleporter",
    "EastPools.Teleporter",
    "LowerWastes.WestTP",
    "LowerWastes.EastTP",
    "UpperWastes.NorthTP",
    "WillowsEnd.InnerTP",
];

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Pathset {
    Moki,
    Gorlek,
    Glitch,
    Unsafe,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Skill {
    Bash,
    WallJump,
    DoubleJump,
    Launch,
    Glide,
    WaterBreath,
    Grenade,
    Grapple,
    Flash,
    Spear,
    Regenerate,
    Bow,
    Hammer,
    Sword,
    Burrow,
    Dash,
    WaterDash,
    Shuriken,
    Seir,
    Blaze,
    Sentry,
    Flap,
    AncestralLight,
}
impl Skill {
    pub fn from_id(id: u8) -> Option<Skill> {
        match id {
            0 => Some(Skill::Bash),
            3 => Some(Skill::WallJump),
            5 => Some(Skill::DoubleJump),
            8 => Some(Skill::Launch),
            14 => Some(Skill::Glide),
            23 => Some(Skill::WaterBreath),
            51 => Some(Skill::Grenade),
            57 => Some(Skill::Grapple),
            62 => Some(Skill::Flash),
            74 => Some(Skill::Spear),
            77 => Some(Skill::Regenerate),
            97 => Some(Skill::Bow),
            98 => Some(Skill::Hammer),
            100 => Some(Skill::Sword),
            101 => Some(Skill::Burrow),
            102 => Some(Skill::Dash),
            104 => Some(Skill::WaterDash),
            106 => Some(Skill::Shuriken),
            108 => Some(Skill::Seir),
            115 => Some(Skill::Blaze),
            116 => Some(Skill::Sentry),
            118 => Some(Skill::Flap),
            120 | 121 => Some(Skill::AncestralLight),
            _ => None,
        }
    }
    pub fn to_id(&self) -> u16 {
        match self {
            Skill::Bash => 0,
            Skill::WallJump => 3,
            Skill::DoubleJump => 5,
            Skill::Launch => 8,
            Skill::Glide => 14,
            Skill::WaterBreath => 23,
            Skill::Grenade => 51,
            Skill::Grapple => 57,
            Skill::Flash => 62,
            Skill::Spear => 74,
            Skill::Regenerate => 77,
            Skill::Bow => 97,
            Skill::Hammer => 98,
            Skill::Sword => 100,
            Skill::Burrow => 101,
            Skill::Dash => 102,
            Skill::WaterDash => 104,
            Skill::Shuriken => 106,
            Skill::Seir => 108,
            Skill::Blaze => 115,
            Skill::Sentry => 116,
            Skill::Flap => 118,
            Skill::AncestralLight => 120,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Resource {
    SpiritLight,
    Health,
    Energy,
    Ore,
    Keystone,
    ShardSlot,
}
impl Resource {
    pub fn from_id(id: u8) -> Option<Resource> {
        match id {
            0 => Some(Resource::Health),
            1 => Some(Resource::Energy),
            2 => Some(Resource::Ore),
            3 => Some(Resource::Keystone),
            4 => Some(Resource::ShardSlot),
            _ => None,
        }
    }
    pub fn to_id(&self) -> u16 {
        match self {
            Resource::SpiritLight => 255,
            Resource::Health => 0,
            Resource::Energy => 1,
            Resource::Ore => 2,
            Resource::Keystone => 3,
            Resource::ShardSlot => 4,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Shard {
    Overcharge,
    TripleJump,
    Wingclip,
    Bounty,
    Swap,
    Magnet,
    Splinter,
    Reckless,
    Quickshot,
    Resilience,
    SpiritLightHarvest,
    Vitality,
    LifeHarvest,
    EnergyHarvest,
    Energy,
    LifePact,
    LastStand,
    Sense,
    UltraBash,
    UltraGrapple,
    Overflow,
    Thorn,
    Catalyst,
    Turmoil,
    Sticky,
    Finesse,
    SpiritSurge,
    Lifeforce,
    Deflector,
    Fracture,
    Arcing,
}
impl Shard {
    pub fn from_id(id: u8) -> Option<Shard> {
        match id {
            1 => Some(Shard::Overcharge),
            2 => Some(Shard::TripleJump),
            3 => Some(Shard::Wingclip),
            4 => Some(Shard::Bounty),
            5 => Some(Shard::Swap),
            8 => Some(Shard::Magnet),
            9 => Some(Shard::Splinter),
            13 => Some(Shard::Reckless),
            14 => Some(Shard::Quickshot),
            18 => Some(Shard::Resilience),
            19 => Some(Shard::SpiritLightHarvest),
            22 => Some(Shard::Vitality),
            23 => Some(Shard::LifeHarvest),
            25 => Some(Shard::EnergyHarvest),
            26 => Some(Shard::Energy),
            27 => Some(Shard::LifePact),
            28 => Some(Shard::LastStand),
            30 => Some(Shard::Sense),
            32 => Some(Shard::UltraBash),
            33 => Some(Shard::UltraGrapple),
            34 => Some(Shard::Overflow),
            35 => Some(Shard::Thorn),
            36 => Some(Shard::Catalyst),
            38 => Some(Shard::Turmoil),
            39 => Some(Shard::Sticky),
            40 => Some(Shard::Finesse),
            41 => Some(Shard::SpiritSurge),
            43 => Some(Shard::Lifeforce),
            44 => Some(Shard::Deflector),
            46 => Some(Shard::Fracture),
            47 => Some(Shard::Arcing),
            _ => None,
        }
    }
    pub fn to_id(&self) -> u16 {
        match self {
            Shard::Overcharge => 1,
            Shard::TripleJump => 2,
            Shard::Wingclip => 3,
            Shard::Bounty => 4,
            Shard::Swap => 5,
            Shard::Magnet => 8,
            Shard::Splinter => 9,
            Shard::Reckless => 13,
            Shard::Quickshot => 14,
            Shard::Resilience => 18,
            Shard::SpiritLightHarvest => 19,
            Shard::Vitality => 22,
            Shard::LifeHarvest => 23,
            Shard::EnergyHarvest => 25,
            Shard::Energy => 26,
            Shard::LifePact => 27,
            Shard::LastStand => 28,
            Shard::Sense => 30,
            Shard::UltraBash => 32,
            Shard::UltraGrapple => 33,
            Shard::Overflow => 34,
            Shard::Thorn => 35,
            Shard::Catalyst => 36,
            Shard::Turmoil => 38,
            Shard::Sticky => 39,
            Shard::Finesse => 40,
            Shard::SpiritSurge => 41,
            Shard::Lifeforce => 43,
            Shard::Deflector => 44,
            Shard::Fracture => 46,
            Shard::Arcing => 47,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Teleporter {
    Marsh,
    Den,
    Hollow,
    Glades,
    Wellspring,
    Burrows,
    WestWoods,
    EastWoods,
    Reach,
    Depths,
    EastLuma,
    WestLuma,
    WestWastes,
    EastWastes,
    OuterRuins,
    InnerRuins,
    Willow,
    Shriek,
}
impl Teleporter {
    pub fn from_id(id: u8) -> Option<Teleporter> {
        match id {
            0 => Some(Teleporter::Burrows),
            1 => Some(Teleporter::Den),
            2 => Some(Teleporter::EastLuma),
            3 => Some(Teleporter::Wellspring),
            4 => Some(Teleporter::Reach),
            5 => Some(Teleporter::Hollow),
            6 => Some(Teleporter::Depths),
            7 => Some(Teleporter::WestWoods),
            8 => Some(Teleporter::EastWoods),
            9 => Some(Teleporter::WestWastes),
            10 => Some(Teleporter::EastWastes),
            11 => Some(Teleporter::OuterRuins),
            12 => Some(Teleporter::Willow),
            13 => Some(Teleporter::WestLuma),
            14 => Some(Teleporter::InnerRuins),
            15 => Some(Teleporter::Shriek),
            16 => Some(Teleporter::Marsh),
            17 => Some(Teleporter::Glades),
            _ => None,
        }
    }
    pub fn to_id(&self) -> u16 {
        match self {
            Teleporter::Burrows => 0,
            Teleporter::Den => 1,
            Teleporter::EastLuma => 2,
            Teleporter::Wellspring => 3,
            Teleporter::Reach => 4,
            Teleporter::Hollow => 5,
            Teleporter::Depths => 6,
            Teleporter::WestWoods => 7,
            Teleporter::EastWoods => 8,
            Teleporter::WestWastes => 9,
            Teleporter::EastWastes => 10,
            Teleporter::OuterRuins => 11,
            Teleporter::Willow => 12,
            Teleporter::WestLuma => 13,
            Teleporter::InnerRuins => 14,
            Teleporter::Shriek => 15,
            Teleporter::Marsh => 16,
            Teleporter::Glades => 17,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BonusItem {
    HealthRegen,
    EnergyRegen,
    ExtraDoubleJump,
    ExtraAirDash,
    Relic,
}
impl BonusItem {
    pub fn from_id(id: u8) -> Option<BonusItem> {
        match id {
            20 => Some(BonusItem::Relic),
            30 => Some(BonusItem::HealthRegen),
            31 => Some(BonusItem::EnergyRegen),
            35 => Some(BonusItem::ExtraDoubleJump),
            36 => Some(BonusItem::ExtraAirDash),
            _ => None,
        }
    }
    pub fn to_id(&self) -> u16 {
        match self {
            BonusItem::Relic => 20,
            BonusItem::HealthRegen => 30,
            BonusItem::EnergyRegen => 31,
            BonusItem::ExtraDoubleJump => 35,
            BonusItem::ExtraAirDash => 36,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BonusUpgrade {
    RapidHammer,
    RapidSword,
    BlazeEfficiency,
    SpearEfficiency,
    ShurikenEfficiency,
    SentryEfficiency,
    BowEfficiency,
    RegenerationEfficiency,
    FlashEfficiency,
    GrenadeEfficiency,
}
impl BonusUpgrade {
    pub fn from_id(id: u8) -> Option<BonusUpgrade> {
        match id {
            0 => Some(BonusUpgrade::RapidHammer),
            1 => Some(BonusUpgrade::RapidSword),
            2 => Some(BonusUpgrade::BlazeEfficiency),
            3 => Some(BonusUpgrade::SpearEfficiency),
            4 => Some(BonusUpgrade::ShurikenEfficiency),
            5 => Some(BonusUpgrade::SentryEfficiency),
            6 => Some(BonusUpgrade::BowEfficiency),
            7 => Some(BonusUpgrade::RegenerationEfficiency),
            8 => Some(BonusUpgrade::FlashEfficiency),
            9 => Some(BonusUpgrade::GrenadeEfficiency),
            _ => None,
        }
    }
    pub fn to_id(&self) -> u16 {
        match self {
            BonusUpgrade::RapidHammer => 0,
            BonusUpgrade::RapidSword => 1,
            BonusUpgrade::BlazeEfficiency => 2,
            BonusUpgrade::SpearEfficiency => 3,
            BonusUpgrade::ShurikenEfficiency => 4,
            BonusUpgrade::SentryEfficiency => 5,
            BonusUpgrade::BowEfficiency => 6,
            BonusUpgrade::RegenerationEfficiency => 7,
            BonusUpgrade::FlashEfficiency => 8,
            BonusUpgrade::GrenadeEfficiency => 9,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Hint {
    Marsh,
    Hollow,
    Glades,
    Wellspring,
    Woods,
    Reach,
    Depths,
    Pools,
    Wastes,
    Ruins,
    Willow,
    Burrows,
    Void,
}
impl Hint {
    pub fn from_id(id: u8) -> Option<Hint> {
        match id {
            0 => Some(Hint::Marsh),
            1 => Some(Hint::Hollow),
            2 => Some(Hint::Glades),
            3 => Some(Hint::Wellspring),
            4 => Some(Hint::Pools),
            5 => Some(Hint::Burrows),
            6 => Some(Hint::Reach),
            7 => Some(Hint::Woods),
            8 => Some(Hint::Depths),
            9 => Some(Hint::Wastes),
            10 => Some(Hint::Ruins),
            11 => Some(Hint::Willow),
            12 => Some(Hint::Void),
            _ => None,
        }
    }
    pub fn to_id(&self) -> u16 {
        match self {
            Hint::Marsh => 0,
            Hint::Hollow => 1,
            Hint::Glades => 2,
            Hint::Wellspring => 3,
            Hint::Pools => 4,
            Hint::Burrows => 5,
            Hint::Reach => 6,
            Hint::Woods => 7,
            Hint::Depths => 8,
            Hint::Wastes => 9,
            Hint::Ruins => 10,
            Hint::Willow => 11,
            Hint::Void => 12,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Enemy {
    Mantis,
    Slug,
    WeakSlug,
    BombSlug,
    CorruptSlug,
    SneezeSlug,
    ShieldSlug,
    Lizard,
    Bat,
    Hornbug,
    Skeeto,
    SmallSkeeto,
    Bee,
    Nest,
    Crab,
    SpinCrab,
    Tentacle,
    Balloon,
    Miner,
    MaceMiner,
    ShieldMiner,
    CrystalMiner,
    ShieldCrystalMiner,
    Sandworm,
    Spiderling,
    EnergyRefill,
}
impl Enemy {
    pub fn health(self) -> f32 {
        match self {
            Enemy::BombSlug | Enemy::CorruptSlug | Enemy::Balloon => 1.0,
            Enemy::SmallSkeeto => 8.0,
            Enemy::WeakSlug | Enemy::Spiderling => 12.0,
            Enemy::Slug => 13.0,
            Enemy::Skeeto | Enemy::Sandworm | Enemy::Tentacle => 20.0,
            Enemy::ShieldSlug | Enemy::Lizard | Enemy::Bee => 24.0,
            Enemy::Nest => 25.0,
            Enemy::Mantis | Enemy::SneezeSlug | Enemy::Bat | Enemy::Crab | Enemy::SpinCrab => 32.0,
            Enemy::Hornbug | Enemy::Miner => 40.0,
            Enemy::MaceMiner | Enemy::ShieldMiner => 60.0,
            Enemy::CrystalMiner | Enemy::ShieldCrystalMiner => 80.0,
            _ => 0.0,
        }
    }
    pub fn shielded(self) -> bool {
        matches!(self, Enemy::Hornbug | Enemy::ShieldSlug | Enemy::ShieldMiner | Enemy::ShieldCrystalMiner)
    }
    pub fn armored(self) -> bool {
        matches!(self, Enemy::Tentacle)
    }
    pub fn aerial(self) -> bool {
        matches!(self, Enemy::Bat | Enemy::Skeeto | Enemy::SmallSkeeto | Enemy::Bee | Enemy::Nest | Enemy::Tentacle)
    }
    pub fn ranged(self) -> bool {
        matches!(self, Enemy::BombSlug | Enemy::CorruptSlug | Enemy::Balloon | Enemy::Bat)
    }
    pub fn dangerous(self) -> bool {
        matches!(self, Enemy::SneezeSlug | Enemy::Hornbug | Enemy::Crab | Enemy::SpinCrab | Enemy::Miner | Enemy::MaceMiner | Enemy::ShieldMiner | Enemy::CrystalMiner | Enemy::ShieldCrystalMiner)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RefillType {
    Full,
    Checkpoint,
    Health(f32),
    Energy(f32),
}
#[derive(Debug, PartialEq)]
pub enum NodeType {
    Anchor,
    Pickup,
    State,
    Quest,
}

pub fn read_file(file: &PathBuf, default_folder: &str) -> Result<String, io::Error> {
    let mut in_folder = PathBuf::new();
    in_folder.push(default_folder);
    in_folder.push(file);
    fs::read_to_string(in_folder).or_else(|_| {
        fs::read_to_string(file)
    })
}

pub fn energy_cost(skill: Skill) -> f32 {
    match skill {
        Skill::Bow => 0.25,
        Skill::Shuriken => 0.5,
        Skill::Grenade | Skill::Flash | Skill::Regenerate | Skill::Blaze | Skill::Sentry => 1.0,
        Skill::Spear => 2.0,
        _ => 0.0,
    }
}
pub fn damage(skill: Skill, unsafe_paths: bool) -> f32 {
    match skill {
        Skill::Bow | Skill::Sword => 4.0,
        Skill::Shuriken => 7.0,
        Skill::Hammer | Skill::Sentry => 12.0,
        Skill::Grenade => if unsafe_paths { 16.0 } else { 12.0 },
        Skill::Flash => 14.0,
        Skill::Blaze => 13.8,
        Skill::Spear => 20.0,
        _ => 0.0,
    }
}

pub fn parse_pickup<'a>(pickup: &'a str) -> Result<(Item, u16), String> {
    let pickup = pickup.trim();
    let mut parts = pickup.split('|');
    let pickup_type = parts.next().unwrap_or("tried to parse empty pickup");
    match pickup_type {
        "0" => {
            let spirit_light = parts.next().ok_or_else(|| format!("missing spirit light amount in pickup {}", pickup))?;
            if spirit_light.starts_with('-') {
                Ok((Item::Custom(pickup.to_string()), 1))
            } else {
                let spirit_light: u16 = spirit_light.parse().map_err(|_| format!("invalid resource type in pickup {}", pickup))?;
                Ok((Item::Resource(Resource::SpiritLight, 1), spirit_light))
            }
        }
        "1" => {
            let resource_type = parts.next().ok_or_else(|| format!("missing resource type in pickup {}", pickup))?;
            let resource_type: u8 = resource_type.parse().map_err(|_| format!("invalid resource type in pickup {}", pickup))?;
            let resource = Resource::from_id(resource_type).ok_or_else(|| format!("invalid resource type in pickup {}", pickup))?;
            Ok((Item::Resource(resource, 1), 1))
        },
        "2" => {
            let skill_type = parts.next().ok_or_else(|| format!("missing skill type in pickup {}", pickup))?;
            if skill_type.starts_with('-') {
                Ok((Item::Custom(pickup.to_string()), 1))
            } else {
                let skill_type: u8 = skill_type.parse().map_err(|_| format!("invalid skill type in pickup {}", pickup))?;
                let skill = Skill::from_id(skill_type).ok_or_else(|| format!("invalid skill type in pickup {}", pickup))?;
                Ok((Item::Skill(skill), 1))
            }
        }
        "3" => {
            let shard_type = parts.next().ok_or_else(|| format!("missing shard type in pickup {}", pickup))?;
            if shard_type.starts_with('-') {
                Ok((Item::Custom(pickup.to_string()), 1))
            } else {
                let shard_type: u8 = shard_type.parse().map_err(|_| format!("invalid shard type in pickup {}", pickup))?;
                let shard = Shard::from_id(shard_type).ok_or_else(|| format!("invalid shard type in pickup {}", pickup))?;
                Ok((Item::Shard(shard), 1))
            }
        }
        "5" => {
            let teleporter_type = parts.next().ok_or_else(|| format!("missing teleporter type in pickup {}", pickup))?;
            if teleporter_type.starts_with('-') {
                Ok((Item::Custom(pickup.to_string()), 1))
            } else {
                let teleporter_type: u8 = teleporter_type.parse().map_err(|_| format!("invalid teleporter type in pickup {}", pickup))?;
                let teleporter = Teleporter::from_id(teleporter_type).ok_or_else(|| format!("invalid teleporter type in pickup {}", pickup))?;
                Ok((Item::Teleporter(teleporter), 1))
            }
        }
        "8" => {
            // sanitize, don't know if this is user input through headers and later it would crash on malformed pickups
            let uber_group = parts.next().ok_or_else(|| format!("missing uber group in pickup {}", pickup))?;
            uber_group.parse::<u16>().map_err(|_| format!("invalid uber group in pickup {}", pickup))?;
            let uber_id = parts.next().ok_or_else(|| format!("missing uber id in pickup {}", pickup))?;
            uber_id.parse::<u16>().map_err(|_| format!("invalid uber id in pickup {}", pickup))?;

            let uber_type = parts.next().ok_or_else(|| format!("missing uber state type in pickup {}", pickup))?;
            let value = parts.next().ok_or_else(|| format!("missing uber value in pickup {}", pickup))?;

            let strip_sign = |value: &'a str| -> &'a str { if value.starts_with(&['+', '-'][..]) { &value[1..] } else { value } };

            match uber_type {
                "bool" | "teleporter" => { value.parse::<bool>().map_err(|_| format!("invalid uber value in pickup {}", pickup))?; },
                "byte" => { strip_sign(value).parse::<u8>().map_err(|_| format!("invalid uber value in pickup {}", pickup))?; },
                "int" => { strip_sign(value).parse::<i32>().map_err(|_| format!("invalid uber value in pickup {}", pickup))?; },
                "float" => { strip_sign(value).parse::<f32>().map_err(|_| format!("invalid uber value in pickup {}", pickup))?; },
                _ => return Err(format!("invalid uber state type in pickup {}", pickup)),
            }

            let command = format!("{}|{}|{}|{}", uber_group, uber_id, uber_type, value);
            Ok((Item::UberState(command), 1))
        }
        "9" => {
            let world_event_type = parts.next().ok_or_else(|| format!("missing world event type in pickup {}", pickup))?;
            if world_event_type.starts_with('-') {
                Ok((Item::Custom(pickup.to_string()), 1))
            } else {
                let world_event_type: u8 = world_event_type.parse().map_err(|_| format!("invalid world event type in pickup {}", pickup))?;
                if world_event_type != 0 { return Err(format!("invalid world event type in pickup {}", pickup)); }
                Ok((Item::Water, 1))
            }
        }
        "10" => {
            let bonus_type = parts.next().ok_or_else(|| format!("missing bonus item type in pickup {}", pickup))?;
            let bonus_type: u8 = bonus_type.parse().map_err(|_| format!("invalid bonus item type in pickup {}", pickup))?;
            let bonus = BonusItem::from_id(bonus_type).ok_or_else(|| format!("invalid bonus item type in pickup {}", pickup))?;
            Ok((Item::BonusItem(bonus), 1))
        }
        "11" => {
            let bonus_type = parts.next().ok_or_else(|| format!("missing bonus item type in pickup {}", pickup))?;
            let bonus_type: u8 = bonus_type.parse().map_err(|_| format!("invalid bonus item type in pickup {}", pickup))?;
            let bonus = BonusUpgrade::from_id(bonus_type).ok_or_else(|| format!("invalid bonus item type in pickup {}", pickup))?;
            Ok((Item::BonusUpgrade(bonus), 1))
        }
        "12" => {
            let hint_type = parts.next().ok_or_else(|| format!("missing hint type in pickup {}", pickup))?;
            let hint_type: u8 = hint_type.parse().map_err(|_| format!("invalid hint type in pickup {}", pickup))?;
            let hint = Hint::from_id(hint_type).ok_or_else(|| format!("invalid hint type in pickup {}", pickup))?;
            Ok((Item::Hint(hint), 1))
        }
        "4" | "6" => {
            Ok((Item::Custom(pickup.to_string()), 1))
        }
        _ => Err(format!("invalid pickup type in pickup {}", pickup)),
    }
}

pub fn trace_parse_error(areas: &PathBuf, position: usize) -> String {
    let input = read_file(areas, "logic").unwrap();
    let mut input = &input[position..];
    if input.starts_with('\n') {
        input = &input[1..];
    }
    if let Some(index) = input.find(&['\n', '\r'][..]) {
        return input[..index].to_string();
    }
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pickup_parsing() {
        assert_eq!(parse_pickup("0|5000"), Ok((Item::Resource(Resource::SpiritLight, 1), 5000)));
        assert_eq!(parse_pickup("0|-5000"), Ok((Item::Custom(String::from("0|-5000")), 1)));
        assert_eq!(parse_pickup("1|2"), Ok((Item::Resource(Resource::Ore, 1), 1)));
        assert!(parse_pickup("1|-2").is_err());
        assert!(parse_pickup("1|5").is_err());
        assert_eq!(parse_pickup("2|8"), Ok((Item::Skill(Skill::Launch), 1)));
        assert_eq!(parse_pickup("2|120"), Ok((Item::Skill(Skill::AncestralLight), 1)));
        assert_eq!(parse_pickup("2|121"), Ok((Item::Skill(Skill::AncestralLight), 1)));
        assert!(parse_pickup("2|25").is_err());
        assert_eq!(parse_pickup("2|-9"), Ok((Item::Custom(String::from("2|-9")), 1)));
        assert_eq!(parse_pickup("3|28"), Ok((Item::Shard(Shard::LastStand), 1)));
        assert_eq!(parse_pickup("5|16"), Ok((Item::Teleporter(Teleporter::Marsh), 1)));
        assert_eq!(parse_pickup("9|0"), Ok((Item::Water, 1)));
        assert_eq!(parse_pickup("9|-0"), Ok((Item::Custom(String::from("9|-0")), 1)));
        assert_eq!(parse_pickup("11|0"), Ok((Item::BonusUpgrade(BonusUpgrade::RapidHammer), 1)));
        assert_eq!(parse_pickup("10|31"), Ok((Item::BonusItem(BonusItem::EnergyRegen), 1)));
        assert!(parse_pickup("12|13").is_err());
        assert!(parse_pickup("8|5|3|6").is_err());
        assert!(parse_pickup("8||||").is_err());
        assert!(parse_pickup("8|5|3|in|3").is_err());
        assert!(parse_pickup("8|5|3|bool|3").is_err());
        assert!(parse_pickup("8|5|3|float|hm").is_err());
        assert_eq!(parse_pickup("8|5|3|int|6"), Ok((Item::UberState(String::from("5|3|int|6")), 1)));
        assert_eq!(parse_pickup("4|0"), Ok((Item::Custom(String::from("4|0")), 1)));
        assert!(parse_pickup("12").is_err());
        assert!(parse_pickup("").is_err());
        assert!(parse_pickup("0|").is_err());
        assert!(parse_pickup("0||400").is_err());
        assert!(parse_pickup("7|3").is_err());
        assert!(parse_pickup("-0|65").is_err());
    }
}