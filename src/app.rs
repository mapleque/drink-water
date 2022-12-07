use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[function_component(App)]
pub fn app() -> Html {
    let drink_counter = use_state(|| 0);

    let drink = {
        let drink_counter = drink_counter.clone();
        move |_| {
            let v = *drink_counter + 1;
            drink_counter.set(v);
        }
    };

    let drag_counter = use_state(|| 0);

    let drag = {
        let drag_counter = drag_counter.clone();
        move |_| {
            let v = *drag_counter + 1;
            drag_counter.set(v);
        }
    };

    let reset = {
        let drink_counter = drink_counter.clone();
        let drag_counter = drag_counter.clone();
        move |_| {
            drink_counter.set(0);
            drag_counter.set(0);
        }
    };

    html! {
        <main class="container">
            <h1>{"Keep health, Drink water!"}</h1>
            <div>
            <button type="button" onclick={reset}>{"Reset Today"}</button>
            </div>
            <div class="row">
                <span class="logo"><b>{ *drink_counter }</b></span>
                <span class="logo"><b>{ *drag_counter }</b></span>
            </div>
             //   <div class="row">
             //       <a href="https://tauri.app" target="_blank">
             //           <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
             //       </a>
             //       <a href="https://yew.rs" target="_blank">
             //           <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
             //       </a>
             //   </div>


            <div class="row">
                <button type="button" onclick={drink}>{"Start Drink"}</button>
                <button type="button" onclick={drag}>{"Have Drag"}</button>
            </div>
        </main>
    }
}
