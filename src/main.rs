mod app;
mod jokes;
mod base_ground;

use std::sync::{Mutex, Arc};
use app::App;
use jokes::{
    changing_box_color, 
    landmarks::{hand_basic, face_basic}
};
use base_ground::{
    event,
    frontend,
    game
};

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<hand_basic::App>::new().render();
}
