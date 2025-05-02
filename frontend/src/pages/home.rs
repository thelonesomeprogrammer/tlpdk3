use crate::comp::card_snake::{CardProps, CardSnake, SnakeProps};
use crate::comp::layered_list_card::{LayeredListCard, LayeredListCardProps};
use crate::comp::modal::Modal;
use crate::comp::robot::Robot;
use gloo::events::EventListener;
use gloo::net::http::Request;
use rmp_serde::Deserializer;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};
use yew::{function_component, html, use_effect_with, use_state, Html};

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
                            "
                            Velkommen til min personlige hjemmeside!
                            Jeg er en softwareudvikler og robotingeniør under uddannelse.
                            Jeg har erfaring inden for cloud-native systemer, embedded udvikling og robotteknologi.
                            Jeg elsker at bygge effektive og innovative løsninger, både i skyen og på bare metal.
                            På dette site kan du se nogle af mine projekter, erfaringer og de teknologier, jeg arbejder med.
                            "
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
                    <h2 class="ex-title">{"Min Tech Rejse: "}</h2>
                <CardSnake
                    cards={
                        vec![
                                CardProps {
                                    title: "Starten".to_string(),
                                    content: "Startede på Windows, med et minimum af programmeringserfaring og en nysgerrighed for teknologiens verden.".to_string(),
                                },
    CardProps {
        title: "Første Skridt".to_string(),
        content: "Begyndte at programmere med Python som en del af min gymnasieuddannelse – det tændte en gnist.".to_string(),
    },
    CardProps {
        title: "Swift & SwiftUI".to_string(),
        content: "Udforskede app-udvikling til Apple-platformen og udgav mine egne apps på App Store.".to_string(),
    },
    CardProps {
        title: "Tilbage til Python".to_string(),
        content: "Da min MacBook ikke længere blev understøttet, vendte jeg tilbage til Python og fokuserede intensivt i flere måneder.".to_string(),
    },
    CardProps {
        title: "Ubuntu Tiden".to_string(),
        content: "Skiftede midlertidigt til Ubuntu for at dykke dybere ned i lav-niveau programmering og udforske Assembly.".to_string(),
    },
    CardProps {
        title: "Frontend Nysgerrighed".to_string(),
        content: "Fik smag for frontend-verdenen gennem HTML, CSS og JavaScript – og endte hurtigt med React.".to_string(),
    },
    CardProps {
        title: "Full-Stack Drømme".to_string(),
        content: "Drømmen om fuld kontrol førte mig til Flask og Django, hvor jeg kunne bygge hele løsninger fra bunden.".to_string(),
    },
    CardProps {
        title: "Manjaro Inspiration".to_string(),
        content: "Så Manjaro på en vens laptop – det vækkede min interesse for Linux og friheden ved open-source.".to_string(),
    },
    CardProps {
        title: "C/C++ Erfaring".to_string(),
        content: "Dykkede ned i C og C++ under studiet – en udfordrende men lærerig oplevelse, hvor Python begyndte at føles begrænsende.".to_string(),
    },
    CardProps {
        title: "Arch Rejsen".to_string(),
        content: "Installerede Arch Linux ved siden af Windows. Gik fra KDE til Qtile og begyndte at forme mit eget miljø.".to_string(),
    },
    CardProps {
        title: "WM Hopperi".to_string(),
        content: "Eksperimenterede med forskellige window managers og arbejdsmiljøer – konstant søgende efter det perfekte setup.".to_string(),
    },
    CardProps {
        title: "Opdagede Rust".to_string(),
        content: "Rust fangede mig med det samme – sikkerhed, fart og moderne udvikling i ét sprog. Det blev hurtigt mit primære værktøj.".to_string(),
    },
    CardProps {
        title: "Rust i Backend".to_string(),
        content: "Begyndte at bruge Rust til backend-udvikling og opdagede dens styrker i systemprogrammering og webudvikling.".to_string(),
    },
    CardProps {
        title: "Rust i Embedded".to_string(),
        content: "Udforskede Rusts potentiale i embedded udvikling – det føltes som en perfekt pasform for min interesse.".to_string(),
    },
    CardProps {
        title: "Rust til Full-Stack".to_string(),
        content: "Begyndte at bruge Rust til både frontend og backend – det føltes som en naturlig udvikling af mine færdigheder.".to_string(),
    },
    CardProps {
        title: "ROS2".to_string(),
        content: "Dykker ned i ROS2 og robotteknologi – en spændende rejse, der kombinerer min interesse for software og hardware. og en del af min uddannelse til robotingeniør".to_string(),
    },
    CardProps {
        title: "Python i dag".to_string(),
        content: "Selvom Rust er mit foretrukne sprog, bruger jeg stadig Python til scripts, automation og hurtige prototyper.".to_string(),
    },
    CardProps {
        title: "Nuværende Setup".to_string(),
        content: "Mit nuværende udviklingsmiljø er Hyprland med Neovim, Alacritty og Vivaldi – skarpt, hurtigt og præcist, som jeg kan lide det.".to_string(),
    },
                        ]
                    }
                    nr={3} />
                </Modal>
            </div>
        </div>
    };
}
