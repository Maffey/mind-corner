mod app_modules;
mod data_analysis;
mod project_consts;

/// Based on saved idea from a long time ago.
/// The application should be a simple (for now) CLI corner for basic mental health activities:
/// 1. Meditation Timer
/// 2. Gratitiude Journal
/// 3. Mood Tracker
/// 4. Breathing Guide
pub fn main() {
    env_logger::init();
    println!("Welcome to Mind Corner!");
    app_modules::select_module();
}
