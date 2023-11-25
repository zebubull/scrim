pub fn get_avg(dice: u32) -> u32 {
    match dice {
        6 => 4,
        8 => 5,
        10 => 6,
        12 => 7,
        _ => panic!("cannot get average roll of dice '{}'", dice),
    }
}

pub fn get_modifier(stat_value: u32) -> i32 {
    ((stat_value as f32 - 10.0).floor() / 2.0) as i32
}

pub fn calculate_hp(level: u32, hit_dice: u32, constitution: u32, racial_bonus: u32) -> u32 {
    let total_con_mod = get_modifier(constitution) * level as i32;
    let first_level = hit_dice;
    let other_levels = (level - 1) * get_avg(hit_dice);
    let racial_bonus = racial_bonus * level;

    (first_level + other_levels + racial_bonus).saturating_add_signed(total_con_mod)
}
