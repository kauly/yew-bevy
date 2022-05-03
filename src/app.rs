use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <div class="w-full h-screen flex flex-col items-center justify-center bg-slate-500">
        <h1>{"YEW and BEVY and Tailwind"}</h1>
        <canvas id="game"></canvas>
      </div>
    }
}
