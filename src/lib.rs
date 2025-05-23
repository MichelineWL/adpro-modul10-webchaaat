#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

// Your main component
struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{ "Hello from Yew!" }</div>
        }
    }
}

#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<Model>();
}
