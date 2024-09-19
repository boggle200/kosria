use js_sys::Array;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
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

#[wasm_bindgen(module = "/static_js/face_landmark_basic.js")]
extern "C" {
    fn face_landmark_basic(callback: &Closure<dyn Fn(Array)>);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let circle_color_rgb = use_state(|| "rgb(255, 255, 255)");

    let canvas_ref = use_node_ref();
    let rect_x = use_state(|| 50.0);
    let rect_y = use_state(|| 50.0);
    let rect_vx = use_state(|| 2.0);
    let rect_vy = use_state(|| 2.0);
    let box_color = use_state(|| "rgba(255, 0, 0, 0.5)".to_string());

    {
        let canvas_ref = canvas_ref.clone();
        let rect_x = rect_x.clone();
        let rect_y = rect_y.clone();
        let rect_vx = rect_vx.clone();
        let rect_vy = rect_vy.clone();
        let box_color = box_color.clone();

        use_effect(move || {
            let window = web_sys::window().expect("no global `window` exists");
            let closure = Closure::wrap(Box::new(move || {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    let context = canvas
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap();
                    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

                    // Update rectangle position
                    let mut x = *rect_x;
                    let mut y = *rect_y;
                    let vx = *rect_vx;
                    let vy = *rect_vy;

                    x += vx;
                    y += vy;

                    if x <= 0.0 || x >= canvas.width() as f64 - 50.0 {
                        rect_vx.set(-vx);
                    }

                    if y <= 0.0 || y >= canvas.height() as f64 - 50.0 {
                        rect_vy.set(-vy);
                    }

                    rect_x.set(x);
                    rect_y.set(y);

                    // Draw rectangle
                    context.set_fill_style(&JsValue::from_str(&box_color));
                    context.fill_rect(x, y, 50.0, 50.0);
                }

                // Request next animation frame
            }) as Box<dyn FnMut()>);

            window.request_animation_frame(closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();

            || {}
        });
    }

    {
        let box_color = box_color.clone();
        let mut check_num = 0;

        use_effect_once(move || {
            let window = web_sys::window().expect("no global `window` exists");
            let keydown_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                if event.key() == "Shift" && check_num == 0 { // "키보드 자판에 적혀있는 글자 그대로 입력해야 키 인식이 됨"
                    box_color.set("rgba(0, 255, 0, 0.5)".to_string());
                    check_num = 1;
                } else if event.key() == "Shift" && check_num == 1 {
                    box_color.set("rgba(255, 255, 0, 0.5".to_string());
                    check_num = 0;
                }
            }) as Box<dyn FnMut(_)>);

            window
                .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
                .expect("failed to add keydown listener");

            keydown_closure.forget();

            || {}
        });
    }

    html! {
        <main class="container">
            <div id="row">
                <canvas id="canvas" ref={canvas_ref} width="1140" height="700" style="border:1px solid #777777;">
                    <video class="input_video" style="display:none;"></video>
                    <canvas class="output_canvas" width="1280" height="720"></canvas>
                </canvas>
            </div>
        </main>
    }
}