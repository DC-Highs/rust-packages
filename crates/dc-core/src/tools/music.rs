use crate::settings::sounds::MUSIC_KEY_NAMES;

pub fn get_music_name_from_tag(tag: &str) -> Option<&'static str> {
    let tag_lower = tag.to_lowercase();
    let music_name_1 = format!("dc_{tag_lower}_island");
    let music_name_2 = format!("_{tag_lower}");

    MUSIC_KEY_NAMES
        .iter()
        .copied()
        .find(|name| name.contains(&music_name_1) || name.ends_with(&music_name_2))
}
