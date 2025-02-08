use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Block Blast Solver (Rust + Yew)" }</h1>
            <button onclick={Callback::from(|_| web_sys::window().unwrap().alert_with_message("Hello from Rust!").unwrap())}>
                { "Click Me!" }
            </button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render(); // Corrected way to render the app
}

