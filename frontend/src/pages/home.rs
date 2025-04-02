use crate::comp::layered_list_card::{LayeredListCard, LayeredListCardProps};
use crate::comp::modal::Modal;
use crate::comp::robot::Robot;
use gloo::events::EventListener;
use gloo::net::http::Request;
use rmp_serde::Deserializer;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};
use yew::{function_component, html, use_effect_with, use_state, Html, Properties};

#[function_component(Home)]
pub fn home() -> Html {
    use_effect_with((), move |_| {
        let window = window().expect("window not available");
        let text_element = window
            .document()
            .expect("document not available")
            .query_selector(".magictext")
            .expect("failed to query selector")
            .expect("query selector returned None")
            .dyn_into::<HtmlElement>()
            .expect("element is not an HtmlElement");

        // Scroll event listener
        let listener = EventListener::new(
            &web_sys::window().expect("window not available"),
            "scroll",
            move |_event| {
                let scroll_y = window.scroll_y().unwrap_or(0.0);
                let transform_value = format!("translateY({}px)", -scroll_y * 0.1);
                text_element
                    .style()
                    .set_property("transform", &transform_value)
                    .expect("failed to set style");
            },
        );

        // Cleanup: automatically drops the listener when effect runs again or component unmounts
        move || drop(listener)
    });

    let layered_list_items = use_state(|| Vec::new());
    {
        let data = layered_list_items.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("/api/experiences").send().await {
                    Ok(response) => {
                        let bytes = response.binary().await.unwrap();
                        let mut deserializer = Deserializer::new(&bytes[..]);
                        let items: Vec<LayeredListCardProps> =
                            Deserialize::deserialize(&mut deserializer).unwrap();
                        data.set(items);
                    }
                    Err(_err) => {}
                }
            });
        });
    }

    return html! {
        <div>
            <div class="title">
                <div class="flex justify-center gap-4 title-inner">
                    <Robot angle1={-20.0} angle2={-25.0} angle3={-6.0} />
                    <h1 class="leading-none magictext">
                        {"The"}<br/>
                        <span class="lonesome"> {"Lonesome"} </span><br/>
                        {"Programmer"}
                    </h1>
                    <Robot angle1={20.0} angle2={25.0} angle3={6.0} />
                </div>
            </div>
            <div class="content pt-4 ps-4 pe-4">
                <Modal>
                    <p>
                        {
                            "Velkommen til min personlige hjemmeside!
                            Jeg er en softwareudvikler og robotingeniør under uddannelse.
                            Jeg har erfaring inden for cloud-native systemer, embedded udvikling og robotteknologi.
                            Jeg elsker at bygge effektive og innovative løsninger, både i skyen og på bare metal.
                            På dette site kan du se nogle af mine projekter, erfaringer og de teknologier, jeg arbejder med."
                        }
                    </p>
                </Modal>
                <div class="content modal-line-container flex justify-center mt-8 pe-8">
                    <Modal>
                        <h2 class="ex-title">{"Mine Erfaringer"}</h2>
                        <div class="flex justify-evenly">
                            {
                                for layered_list_items.iter().map(|experience| {
                                    html! {
                                        <LayeredListCard title={experience.title.to_string()} items={experience.items.clone()} />
                                    }
                                })
                            }
                        </div>
                    </Modal>
                </div>
            </div>
            <div class="content pt-4 ps-4 pe-4 ">
                <Modal>
                    <div class="relative modal-line-container grid snake">
                        <SnakeTurn turn={SnakeTurnType::EmptyRight} /> <SnakeTurn turn={SnakeTurnType::Straight} /><Card /> <SnakeTurn turn={SnakeTurnType::Straight} /> <Card /> <SnakeTurn turn={SnakeTurnType::Straight} /> <Card /> <SnakeTurn turn={SnakeTurnType::Straight} /> <SnakeTurn turn={SnakeTurnType::LeftDown} />
                        <SnakeTurn turn={SnakeTurnType::RightDown} /> <SnakeTurn turn={SnakeTurnType::Straight} /> <Card /> <SnakeTurn turn={SnakeTurnType::Straight} /> <Card /> <SnakeTurn turn={SnakeTurnType::Straight} /> <Card /> <SnakeTurn turn={SnakeTurnType::Straight} /> <SnakeTurn turn={SnakeTurnType::UpLeft} />
                        <SnakeTurn turn={SnakeTurnType::UpRight} /> <SnakeTurn turn={SnakeTurnType::Straight} /> <Card /> <SnakeTurn turn={SnakeTurnType::Straight} /> <Card /> <SnakeTurn turn={SnakeTurnType::Straight} /> <Card /> <SnakeTurn turn={SnakeTurnType::Straight} /> <SnakeTurn turn={SnakeTurnType::LeftDown} />
                        <SnakeTurn turn={SnakeTurnType::EmptyRight} /> <SnakeTurn turn={SnakeTurnType::Straight} /><Card /> <SnakeTurn turn={SnakeTurnType::Straight} /> <Card /> <SnakeTurn turn={SnakeTurnType::Straight} /> <Card /> <SnakeTurn turn={SnakeTurnType::Straight} /> <SnakeTurn turn={SnakeTurnType::UpLeft} />
                    </div>
                </Modal>
            </div>
        </div>
    };
}

#[function_component(Card)]
fn card() -> Html {
    html! {
        <div class="card justify-center items-center flex">
            <Modal>
                <h3>{"some text"}</h3>
                <p class="text-sm text-gray-600">
                    {"some longer description some longer description even more description and so on ..."}
                </p>
            </Modal>
        </div>
    }
}

#[derive(Clone, PartialEq)]
enum SnakeTurnType {
    EmptyRight,
    EmptyLeft,
    Straight,
    LeftDown,
    UpLeft,
    RightDown,
    UpRight,
}

#[derive(Clone, PartialEq, Properties)]
struct SnakeTurnProps {
    turn: SnakeTurnType,
}

#[function_component(SnakeTurn)]
fn turn(props: &SnakeTurnProps) -> Html {
    html! {
        <div class="snake-turn">
            {
                match props.turn {
                    SnakeTurnType::EmptyRight => html! { <><div class="empty" /><div class="straight" /></> },
                    SnakeTurnType::EmptyLeft => html! { <><div class="straight" /><div class="empty" /></> },
                    SnakeTurnType::Straight => html! { <div class="straight" /> },
                    SnakeTurnType::LeftDown => html! { <div class="left-down" /> },
                    SnakeTurnType::UpLeft => html! { <div class="up-left" /> },
                    SnakeTurnType::RightDown => html! { <div class="right-down" /> },
                    SnakeTurnType::UpRight => html! { <div class="up-right" /> },
                }
            }
        </div>
    }
}
