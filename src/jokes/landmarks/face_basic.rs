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

    let landmark_data = use_state(|| vec![]);

    {
        let landmark_data = landmark_data.clone();
        use_effect_once(move || {
            let callback = Closure::wrap(Box::new(move |landmarks: Array| {
                let mut new_data = vec![];

                for i in 0..landmarks.length() {
                    let point = landmarks.get(i).dyn_into::<Array>().unwrap();
                    new_data.push(vec![
                        point.get(0).as_f64().unwrap_or(0.0),
                        point.get(1).as_f64().unwrap_or(0.0),
                        point.get(2).as_f64().unwrap_or(0.0),
                    ]);
                }

                landmark_data.set(new_data);
            }) as Box<dyn Fn(Array)>);
            face_landmark_basic(&callback);
            callback.forget();
            || {}
        });
    }

    let canvas_ref = use_node_ref();
    {
        let landmark_data = landmark_data.clone();
        let canvas_ref = canvas_ref.clone();

        use_effect(move || {
            if let Some(canvas) = canvas_ref.cast::<web_sys::HtmlCanvasElement>() {
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();
                context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

                context.begin_path(); // 좌표를 이용해서 그림 (x좌표, y좌표)
                context.move_to(100.0, 100.0);
                context.line_to(200.0, 100.0);
                context.line_to(150.0, 200.0);
                context.close_path();

                context.set_stroke_style(&JsValue::from_str("blue")); // 이 코드 2개 없으면 안 그려짐
                context.stroke();

                context.set_fill_style(&JsValue::from_str("rgba(0, 0, 255, 0.5)")); // 삼각형 색 채우는 코드
                context.fill();

                if !landmark_data.is_empty() {
                    console_log(landmark_data[477][0].to_string());
                    
                    for (i, landmark) in landmark_data.iter().enumerate() {
                        let x = 841.0 - ((landmark[0] * 100.0) * 6.0);
                        let y = 78.0 + ((landmark[1] * 100.0) * 6.0);
                        
                        console_log(format!("Landmark {}: x={}, y={}, z={}", i, landmark[0], landmark[1], landmark[2]));
                        
                        context.set_fill_style(&JsValue::from_str(&*circle_color_rgb));
                        context.begin_path();
                        context.arc(x, y, 2.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                        context.fill();
                        context.close_path();

                        {
                            context.begin_path();
                            context.move_to(landmark[0], landmark[1]);
                            context.line_to(landmark[0], landmark[1]);
                            context.line_to(landmark[0], landmark[1]);
                            context.close_path();

                            context.set_stroke_style(&JsValue::from_str("blue")); // 이 코드 2개 없으면 안 그려짐
                            context.stroke();

                            context.set_fill_style(&JsValue::from_str("rgba(0, 0, 255, 0.5)")); // 삼각형 색 채우는 코드
                            context.fill();
                        }
                    }
                }
            }
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