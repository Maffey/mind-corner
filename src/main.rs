use crate::app_modules::AppAction;

mod app_modules;
mod data;
mod project_consts;
mod utilities;

pub fn main() {
    env_logger::init();
    println!("Welcome to Mind Corner!");
    loop {
         if app_modules::select_module() == AppAction::Exit {
             break
         }
    }
}
