use js_sys::Array;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, prelude::*};
use yew_hooks::use_effect_once;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[wasm_bindgen(module = "/static_js/console_log.js")]
extern "C" {
    fn console_log(prompt: String);
}

#[wasm_bindgen(module = "/static_js/hand_landmark_basic.js")]
extern "C" {
    fn hand_landmark_basic(callback: &Closure<dyn Fn(Array, Array)>);
}

/*#[wasm_bindgen(module = "/static_js/face_landmark_basic.js")]
extern "C" {
    fn face_landmark_basic(callback: &Closure<dyn Fn(Array, Array)>);
}*/

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
    }
}