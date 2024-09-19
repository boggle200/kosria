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
    let circle_color_rgb = use_state(|| "rgb(255, 255, 255)");
    let box_color = use_state_eq(|| "rgb(124, 155, 102)");

    let landmark_data = use_state(|| vec![]);
    let hand_labels = use_state(|| vec![]);

    {
        let landmark_data = landmark_data.clone();
        let hand_labels = hand_labels.clone();
        use_effect_once(move || {
            let callback = Closure::wrap(Box::new(move |landmarks: Array, handedness: Array| {
                let mut new_data = vec![];
                let mut labels = vec![];

                for i in 0..landmarks.length() {
                    let hand = landmarks.get(i).dyn_into::<Array>().unwrap();
                    let label = handedness.get(i).as_string().unwrap_or("unknown".to_string());
                    labels.push(label);

                    let mut hand_data = vec![vec![0.0, 0.0, 0.0]; 21];
                    for j in 0..hand.length() {
                        let point = hand.get(j).dyn_into::<Array>().unwrap();
                        hand_data[j as usize] = vec![
                            point.get(0).as_f64().unwrap_or(0.0),
                            point.get(1).as_f64().unwrap_or(0.0),
                            point.get(2).as_f64().unwrap_or(0.0),
                        ];
                    }
                    new_data.push(hand_data);
                }

                hand_labels.set(labels);
                landmark_data.set(new_data);
            }) as Box<dyn Fn(Array, Array)>);
            hand_landmark_basic(&callback);
            callback.forget();
            || {}
        });
    }

    let canvas_ref = use_node_ref();
    {
        let landmark_data = landmark_data.clone();
        let hand_labels = hand_labels.clone();
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

                let cx = 570.0;
                let cy = 300.0;

                let main_box_width = 400.0;
                let main_box_height = 170.0;
                let box_radius = 15.0;

                let processed_cx = cx - (main_box_width / 2.0);
                let processed_cy = cy - (main_box_height / 2.0);

                let spot_1 = [processed_cx, processed_cy];
                let spot_2 = [processed_cx + main_box_width, processed_cy];
                let spot_3 = [processed_cx + main_box_width, processed_cy + main_box_height];
                let spot_4 = [processed_cx, processed_cy + main_box_height];

                {
                    context.set_fill_style(&JsValue::from_str(&*&box_color));
                    context.begin_path();
                    context.fill_rect(processed_cx, processed_cy, main_box_width, main_box_height);
                    context.arc(spot_1[0], spot_1[1], 30.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                    context.arc(spot_2[0], spot_2[1], 30.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                    context.arc(spot_3[0], spot_3[1], 30.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                    context.arc(spot_4[0], spot_4[1], 30.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                    context.fill();
                }

                if !landmark_data.is_empty() {
                    for (hand_idx, hand) in landmark_data.iter().enumerate() {
                        for i in 0..21 {
                            let landmark = &hand[i];
                            let x = 841.0 - ((landmark[0] * 100.0) * 6.0);
                            let y = 78.0 + ((landmark[1] * 100.0) * 6.0);
                            context.set_fill_style(&JsValue::from_str(&*circle_color_rgb));
                            context.begin_path();
                            context.arc(x, y, 5.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                            context.fill();

                            //console_log(hand[0][0].to_string()); // [left or right][index num] 형식. index num 값 정확히 뽑아냄
                        }
                        // Print hand label
                        let label = &hand_labels[hand_idx];
                        console_log(label.to_string());/*
                        context.set_fill_style(&JsValue::from_str("black"));
                        context.set_font("20px Arial");
                        context.fill_text(label, 10.0, 30.0 + hand_idx as f64 * 30.0).unwrap();*/
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