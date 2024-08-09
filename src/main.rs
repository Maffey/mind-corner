mod app_modules;
/// Based on saved idea from a long time ago.
/// The application should be a simple (for now) CLI corner for basic mental health activities:
/// 1. Meditation Timer
/// 2. Gratitiude Journal
/// 3. Mood Tracker
/// 4. Breathing Guide
pub fn main() {
    println!("Welcome to Mind Corner! What would you like to do?");
    // TODO Implement inquire (#1)
    app_modules::select_module();
}
