use yew::prelude::*;
use gloo::timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| chrono::Utc::now());

    {
        let counter = counter.clone();
        spawn_local(async move {
            TimeoutFuture::new(1_000).await;
            counter.set(chrono::Utc::now());
        });
    }

    let bgcolor = format!("background-color: {};", (*counter).format("#%H%M%S"));

    html! {
        <main style={bgcolor}>
            <time>{ (*counter).format("%H:%M:%S") }</time>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
