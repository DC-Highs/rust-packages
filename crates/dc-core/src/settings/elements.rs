use crate::enums::DragonElement;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElementInfo {
    pub acronym: &'static str,
    pub name_key: &'static str,
    pub strengths: &'static [DragonElement],
    pub weaknesses: &'static [DragonElement],
}

pub const fn element_info(element: DragonElement) -> ElementInfo {
    match element {
        DragonElement::Terra => ElementInfo {
            acronym: "e",
            name_key: "TerraKey_DragonUtils",
            strengths: &[DragonElement::Electric, DragonElement::Flame],
            weaknesses: &[DragonElement::Metal, DragonElement::War],
        },
        DragonElement::Flame => ElementInfo {
            acronym: "f",
            name_key: "FlameKey_DragonUtils",
            strengths: &[DragonElement::Nature, DragonElement::Ice],
            weaknesses: &[DragonElement::Sea, DragonElement::Terra],
        },
        DragonElement::Sea => ElementInfo {
            acronym: "w",
            name_key: "SeaKey_DragonUtils",
            strengths: &[DragonElement::Flame, DragonElement::War],
            weaknesses: &[DragonElement::Nature, DragonElement::Electric],
        },
        DragonElement::Nature => ElementInfo {
            acronym: "p",
            name_key: "NatureKey_DragonUtils",
            strengths: &[DragonElement::Sea, DragonElement::Light],
            weaknesses: &[DragonElement::Flame, DragonElement::Ice],
        },
        DragonElement::Electric => ElementInfo {
            acronym: "el",
            name_key: "ElectricKey_DragonUtils",
            strengths: &[DragonElement::Sea, DragonElement::Metal],
            weaknesses: &[DragonElement::Terra, DragonElement::Light],
        },
        DragonElement::Ice => ElementInfo {
            acronym: "i",
            name_key: "IceKey_DragonUtils",
            strengths: &[DragonElement::Nature, DragonElement::War],
            weaknesses: &[DragonElement::Flame, DragonElement::Metal],
        },
        DragonElement::Metal => ElementInfo {
            acronym: "m",
            name_key: "MetalKey_DragonUtils",
            strengths: &[DragonElement::Terra, DragonElement::Ice],
            weaknesses: &[DragonElement::Electric, DragonElement::Dark],
        },
        DragonElement::Dark => ElementInfo {
            acronym: "d",
            name_key: "DarkKey_DragonUtils",
            strengths: &[DragonElement::Metal, DragonElement::Light],
            weaknesses: &[DragonElement::War],
        },
        DragonElement::Light => ElementInfo {
            acronym: "li",
            name_key: "LightKey_DragonUtils",
            strengths: &[DragonElement::Electric, DragonElement::Dark],
            weaknesses: &[DragonElement::Nature],
        },
        DragonElement::War => ElementInfo {
            acronym: "wr",
            name_key: "WarKey_DragonUtils",
            strengths: &[DragonElement::Terra, DragonElement::Dark],
            weaknesses: &[DragonElement::Sea, DragonElement::Ice],
        },
        DragonElement::Pure => ElementInfo {
            acronym: "pu",
            name_key: "PureKey_DragonUtils",
            strengths: &[DragonElement::Wind],
            weaknesses: &[DragonElement::Primal],
        },
        DragonElement::Legend => ElementInfo {
            acronym: "l",
            name_key: "LegendKey_DragonUtils",
            strengths: &[DragonElement::Primal],
            weaknesses: &[DragonElement::Pure],
        },
        DragonElement::Primal => ElementInfo {
            acronym: "pr",
            name_key: "tid_nw_hud_element_primal",
            strengths: &[DragonElement::Pure],
            weaknesses: &[DragonElement::Time],
        },
        DragonElement::Wind => ElementInfo {
            acronym: "wd",
            name_key: "tid_nw_hud_element_wind",
            strengths: &[DragonElement::Time],
            weaknesses: &[DragonElement::Legend],
        },
        DragonElement::Time => ElementInfo {
            acronym: "ti",
            name_key: "tid_nw_hud_element_time",
            strengths: &[DragonElement::Legend],
            weaknesses: &[DragonElement::Wind],
        },
        DragonElement::Happy => ElementInfo {
            acronym: "hp",
            name_key: "tid_nw_hud_element_happiness",
            strengths: &[DragonElement::Chaos, DragonElement::Magic],
            weaknesses: &[],
        },
        DragonElement::Chaos => ElementInfo {
            acronym: "ch",
            name_key: "tid_nw_hud_element_chaos",
            strengths: &[DragonElement::Magic, DragonElement::Soul],
            weaknesses: &[],
        },
        DragonElement::Magic => ElementInfo {
            acronym: "mg",
            name_key: "tid_nw_hud_element_magic",
            strengths: &[DragonElement::Soul, DragonElement::Beauty],
            weaknesses: &[],
        },
        DragonElement::Soul => ElementInfo {
            acronym: "so",
            name_key: "tid_nw_hud_element_soul",
            strengths: &[DragonElement::Dream, DragonElement::Beauty],
            weaknesses: &[],
        },
        DragonElement::Beauty => ElementInfo {
            acronym: "bt",
            name_key: "tid_nw_hud_element_beauty",
            strengths: &[DragonElement::Dream, DragonElement::Happy],
            weaknesses: &[],
        },
        DragonElement::Dream => ElementInfo {
            acronym: "dr",
            name_key: "tid_nw_hud_element_dream",
            strengths: &[DragonElement::Happy, DragonElement::Chaos],
            weaknesses: &[],
        },
        DragonElement::Physical => ElementInfo {
            acronym: "ph",
            name_key: "Physical",
            strengths: &[],
            weaknesses: &[DragonElement::Legend],
        },
    }
}
