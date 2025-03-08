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

//#[derive(Properties, PartialEq, Clone, Deserialize)]
//pub struct FeatureCardProps {
//    pub headline: String,
//    pub description: String,
//    pub keywords: Vec<String>,
//    pub features: Vec<String>,
//    pub todos: Vec<String>,
//    pub logo: String,
//    pub status: Status,
//    pub state: State,
//}

#[function_component(FeatureCard)]
pub fn feature_card(props: &FeatureCardProps) -> Html {
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
                                    <circle cx="7" cy="7" r="7" class="fill-red-400/55" />
                                    <circle cx="7" cy="7" r="6" class="stroke-red-400/55" />
                                </svg>
                            </div>
                            <div class="flex items-center gap-x-1.5">
                                <p class="text-sm font-medium">{"dev:"}</p>
                                <svg class="w-4 h-4 shrink-0" fill="none" stroke-linecap="round">
                                    <circle cx="7" cy="7" r="7" class="fill-red-400/55" />
                                    <circle cx="7" cy="7" r="6" class="stroke-red-400/55" />
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
                                    { for props.features.split("\n").map(|feature| html! { <li>{ feature }</li> }) }
                                </ul>
                            </div>
                            <div>
                                <h3 class="text-md font-semibold text-gray-700 dark:text-gray-300 mb-1">{"Improvements"}</h3>
                                <ul class="text-sm text-gray-600 dark:text-gray-400 list-disc pl-5">
                                    { for props.improvements.split("\n").map(|todo| html! { <li>{ todo }</li> }) }
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
    //let sample_data = vec![
    //    FeatureCardProps {
    //        headline: "Rusty Bar".to_string(),
    //        description: "Rusty Bar er en statusbar til Wayland, udviklet i Rust med GTK-4. Den er designet med fokus på konfiguration og ikoner. Primært skabt til eget brug, er Rusty Bar særligt optimeret til Hyprland, da det er den window manager, jeg anvender. Jeg bruger selv Rusty Bar som min statusbar.".to_string(),
    //        logo: "https://images.unsplash.com/photo-1417325384643-aac51acc9e5d?q=75&fm=jpg&w=1080&fit=max".to_string(),
    //        keywords: vec![
    //            "Linux".to_string(),
    //            "Wayland".to_string(),
    //            "GTK-4".to_string(),
    //            "System Monitoring".to_string(),
    //            "Status Bar".to_string(),
    //        ],
    //        features: vec![
    //            "ron configuration".to_string(),
    //            "Hyprland Virtual Desktop Management".to_string(),
    //            "System Metrics (CPU, ram, temps, battery)".to_string(),
    //            "volume control with alsa integration".to_string(),
    //            "Pango markup and text formatting".to_string(),
    //        ],
    //        todos: vec![
    //            "Generic Wlroots Virtual Desktop Management".to_string(),
    //            "System tray".to_string(),
    //            "Increased stability".to_string(),
    //        ],
    //        status: Status::Stopped,
    //        state:State::Beta,
    //
    //    },
    //    FeatureCardProps {
    //        headline: "Headline".to_string(),
    //        description: "Description".to_string(),
    //        logo: "https://images.unsplash.com/photo-1417325384643-aac51acc9e5d?q=75&fm=jpg&w=1080&fit=max".to_string(),
    //        keywords: vec![
    //            "Item 1".to_string(),
    //            "Item 2".to_string(),
    //            "Item 3".to_string(),
    //            "Item 4".to_string(),
    //        ],
    //        features: vec![
    //            "Item A".to_string(),
    //            "Item B".to_string(),
    //            "Item C".to_string(),
    //            "Item D".to_string(),
    //        ],
    //        todos: vec![
    //            "Task A".to_string(),
    //            "Task B".to_string(),
    //            "Task C".to_string(),
    //            "Task D".to_string(),
    //        ],
    //        status: Status::Stopped,
    //        state:State::Lvs,
    //    },
    //    FeatureCardProps {
    //        headline: "Headline".to_string(),
    //        description: "Description".to_string(),
    //        logo: "/path/to/logo.png".to_string(),
    //        keywords: vec![
    //            "Item 1".to_string(),
    //            "Item 2".to_string(),
    //            "Item 3".to_string(),
    //            "Item 4".to_string(),
    //        ],
    //        features: vec![
    //            "Feature X".to_string(),
    //            "Feature Y".to_string(),
    //            "Feature Z".to_string(),
    //        ],
    //        todos: vec![
    //            "Todo X".to_string(),
    //            "Todo Y".to_string(),
    //            "Todo Z".to_string(),
    //        ],
    //        status: Status::Stopped,
    //        state:State::Lvs,
    //    },
    //];

    let data = use_state(|| vec![]);
    {
        let data = data.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match Request::get("/api/projects").send().await {
                Ok(response) => {
                    let bytes = response.binary().await.unwrap();
                    let mut deserializer = Deserializer::new(&bytes[..]);
                    let items: Vec<FeatureCardProps> =
                        Deserialize::deserialize(&mut deserializer).unwrap();
                    data.set(items);
                }
                Err(err) => {}
            }
        });
    }

    html! {
        <div class="space-y-4 p-6 bg-gray-100 dark:bg-gray-900 min-h-screen">
            { for data.iter().map(|data| html! { <FeatureCard ..data.clone() /> }) }
        </div>
    }
}
