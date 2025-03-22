use gloo::net::http::Request;
use rmp_serde::Deserializer;
use serde::Deserialize;
use yew::prelude::*;

#[derive(PartialEq, Clone, Deserialize)]
pub enum Status {
    Strated,
    Stalled,
    Stopped,
    Done,
}

#[derive(PartialEq, Clone, Deserialize)]
pub enum State {
    NotReady,
    Lvs,
    Beta,
    Ready,
}

#[derive(Properties, PartialEq, Clone, Deserialize)]
pub struct FeatureCardProps {
    pub headine: String,
    pub description: String,
    pub features: String,
    pub improvements: String,
    pub logo: String,
    pub devstatus: Status,
    pub state: State,
}

#[function_component(FeatureCard)]
pub fn feature_card(props: &FeatureCardProps) -> Html {
    let state = match props.state {
        State::NotReady => "fill-red-400/55 stroke-red-400/55",
        State::Lvs => "fill-yellow-400/55 stroke-yellow-400/55",
        State::Beta => "fill-blue-400/55 stroke-blue-400/55",
        State::Ready => "fill-green-400/55 stroke-green-400/55",
    };
    let devstatus = match props.devstatus {
        Status::Strated => ("fill-blue-400/55", "stroke-blue-400/55"),
        Status::Stalled => ("fill-yellow-400/55", "stroke-yellow-400/55"),
        Status::Stopped => ("fill-red-400/55", "stroke-red-400/55"),
        Status::Done => ("fill-green-400/55", "stroke-green-400/55"),
    };

    html! {
        <div class="rounded-2xl shadow-md p-4 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-200">
            <div class="flex items-start gap-4">

                <div class="flex-shrink-0"><img src={props.logo.clone()} alt="Logo" class="w-30 h-40 rounded-md" /></div>

                <div class="block" style="width:100%" >
                    <div class="flex justify-between" >
                        <h2 class="text-xl font-bold text-gray-800 dark:text-gray-100">{ &props.headine }</h2>
                        <div class="flex items-center gap-x-2">
                            <div class="flex items-center gap-x-1.5">
                                <p class="text-sm font-medium">{"state:"}</p>
                                <svg class="w-4 h-4 shrink-0" fill="none" stroke-linecap="round">
                                    <circle cx="7" cy="7" r="7" class={state} />
                                    <circle cx="7" cy="7" r="6" class={state} />
                                </svg>
                            </div>
                            <div class="flex items-center gap-x-1.5">
                                <p class="text-sm font-medium">{"dev:"}</p>
                                <svg class="w-4 h-4 shrink-0" fill="none" stroke-linecap="round">
                                    <circle cx="7" cy="7" r="7" class={devstatus.0} />
                                    <circle cx="7" cy="7" r="6" class={devstatus.1} />
                                </svg>
                            </div>
                        </div>
                    </div>
                    <div class="grid grid-cols-3 gap-4">
                        <p class="text-sm text-gray-600 dark:text-gray-400 mb-2">{ &props.description }</p>
                        <div class="grid grid-cols-3 col-span-2 gap-4">
                            <div>
                                <h3 class="text-md font-semibold text-gray-700 dark:text-gray-300 mb-1">{"Key-words"}</h3>
                                <ul class="text-sm text-gray-600 dark:text-gray-400 list-disc pl-5">
                                </ul>
                            </div>
                            <div>
                                <h3 class="text-md font-semibold text-gray-700 dark:text-gray-300 mb-1">{"Features"}</h3>
                                <ul class="text-sm text-gray-600 dark:text-gray-400 list-disc pl-5">
                                    {
                                        for props.features.split("\n").filter_map(|feature|
                                            if feature.is_empty(){None}else{ Some(html! { <li>{ feature }</li> })})
                                    }
                                </ul>
                            </div>
                            <div>
                                <h3 class="text-md font-semibold text-gray-700 dark:text-gray-300 mb-1">{"Improvements"}</h3>
                                <ul class="text-sm text-gray-600 dark:text-gray-400 list-disc pl-5">
                                    {
                                        for props.improvements.split("\n").filter_map(|todo|
                                            if todo.is_empty(){None}else{Some(html! { <li>{ todo }</li> })})
                                    }
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(Projects)]
pub fn main() -> Html {
    let data = use_state(|| Vec::new());
    {
        let data = data.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("/api/projects").send().await {
                    Ok(response) => {
                        let bytes = response.binary().await.unwrap();
                        let mut deserializer = Deserializer::new(&bytes[..]);
                        let items: Vec<FeatureCardProps> =
                            Deserialize::deserialize(&mut deserializer).unwrap();
                        data.set(items);
                    }
                    Err(_err) => {}
                }
            });
        });
    }

    html! {
        <div class="space-y-4 p-6 bg-gray-100 dark:bg-gray-900 min-h-screen">
            <h1 class="text-3xl font-bold text-gray-800 dark:text-gray-100">{"Projects"}</h1>
            <h1 class="text-3xl font-bold text-gray-800 dark:text-gray-100">{"Projects"}</h1>
            { for data.iter().map(|data| html! { <FeatureCard ..data.clone() /> }) }
        </div>
    }
}
