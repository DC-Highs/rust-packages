use crate::enums::{DragonCategory, DragonRank};

pub const MINIMUM_LEVEL: u32 = 1;
pub const MAXIMUM_LEVEL: u32 = 70;
pub const MAXIMUM_LEVEL_WITHOUT_EMPOWERMENT: u32 = 40;
pub const MINIMUM_LEVEL_TO_EMPOWER: u32 = 7;

pub const MINIMUM_STARS: u32 = 0;
pub const MAXIMUM_STARS: u32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmpowermentStage {
    pub requested_orbs: u32,
    pub usable_joker_orbs: u32,
    pub dragon_maximum_level: u32,
}

pub const EMPOWERMENT_STAGES: [EmpowermentStage; 5] = [
    EmpowermentStage {
        requested_orbs: 120,
        usable_joker_orbs: 25,
        dragon_maximum_level: 45,
    },
    EmpowermentStage {
        requested_orbs: 200,
        usable_joker_orbs: 50,
        dragon_maximum_level: 50,
    },
    EmpowermentStage {
        requested_orbs: 320,
        usable_joker_orbs: 80,
        dragon_maximum_level: 55,
    },
    EmpowermentStage {
        requested_orbs: 560,
        usable_joker_orbs: 170,
        dragon_maximum_level: 60,
    },
    EmpowermentStage {
        requested_orbs: 800,
        usable_joker_orbs: 240,
        dragon_maximum_level: 70,
    },
];

pub const ONE_STAR_POWER: f64 = 0.38223;
pub const DAMAGE_VARIANT: (f64, f64) = (0.9, 1.1);

pub const fn category_power(category: DragonCategory) -> f64 {
    match category {
        DragonCategory::Basic => 3200.0,
        DragonCategory::ProBasic => 3600.0,
        DragonCategory::PreAdvanced => 4000.0,
        DragonCategory::Advanced => 4800.0,
        DragonCategory::Elite => 5600.0,
        DragonCategory::Amazing => 5000.0,
        DragonCategory::Charming => 4500.0,
        DragonCategory::Graceful => 3000.0,
        DragonCategory::Champion => 6000.0,
        DragonCategory::Mythical => 7300.0,
        DragonCategory::Heroic => 9000.0,
    }
}

pub const fn rank_power(rank: DragonRank) -> f64 {
    match rank {
        DragonRank::BronzeIII => 0.05,
        DragonRank::BronzeII => 0.10,
        DragonRank::BronzeI => 0.15,
        DragonRank::SilverIII => 0.20,
        DragonRank::SilverII => 0.25,
        DragonRank::SilverI => 0.30,
        DragonRank::GoldIII => 0.35,
        DragonRank::GoldII => 0.40,
        DragonRank::GoldI => 0.50,
        DragonRank::PlatinumIII => 0.55,
        DragonRank::PlatinumII => 0.60,
        DragonRank::PlatinumI => 0.70,
    }
}

pub const fn rank_bonus_from_u8(rank: u8) -> f64 {
    match rank {
        1 => 0.05,
        2 => 0.10,
        3 => 0.15,
        4 => 0.20,
        5 => 0.25,
        6 => 0.30,
        7 => 0.35,
        8 => 0.40,
        9 => 0.50,
        10 => 0.55,
        11 => 0.60,
        12 => 0.70,
        _ => 0.0,
    }
}

pub const fn rank_name_key(rank: DragonRank) -> &'static str {
    match rank {
        DragonRank::BronzeIII => "tid_rank_bronze_iii",
        DragonRank::BronzeII => "tid_rank_bronze_ii",
        DragonRank::BronzeI => "tid_rank_bronze_i",
        DragonRank::SilverIII => "tid_rank_silver_iii",
        DragonRank::SilverII => "tid_rank_silver_ii",
        DragonRank::SilverI => "tid_rank_silver_i",
        DragonRank::GoldIII => "tid_rank_gold_iii",
        DragonRank::GoldII => "tid_rank_gold_ii",
        DragonRank::GoldI => "tid_rank_gold_i",
        DragonRank::PlatinumIII => "tid_rank_platinum_iii",
        DragonRank::PlatinumII => "tid_rank_platinum_ii",
        DragonRank::PlatinumI => "tid_rank_platinum_i",
    }
}
