use crate::utils::progress_bar::ProgressBar;

pub type Exp = i64;
pub type Level = i64;


pub fn xp_to_level(xp: Exp, level_divider: i32) -> Level {
    if level_divider <= 0 { return 0 }
    let result = f64::floor((1.0 + f64::sqrt(1.0 + 8.0 * xp as f64 / level_divider as f64)) / 2.0);
    result as Level
}

pub fn level_to_xp(level: Level, level_divider: i32) -> Exp { 
    if level_divider <= 0 { return 0 }
    let result = f64::floor((level as f64 * (level - 1) as f64 / 2.0) * level_divider as f64);
    result as Exp
}

pub fn make_progress_bar(exp: Exp, exp_for_next_level: Exp, char_count: usize) -> String {
    ProgressBar::builder()
        .scale(exp_for_next_level as f64)
        .progress(exp as f64)
        .char_count(char_count)
        .background("🟩")
        .fill("⬛")
        .render()
}
