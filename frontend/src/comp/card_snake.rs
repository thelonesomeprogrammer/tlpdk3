use crate::comp::modal::Modal;
use yew::prelude::{function_component, html, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct SnakeProps {
    pub nr: usize,
    pub cards: Vec<CardProps>,
}

#[function_component(CardSnake)]
pub fn main(props: &SnakeProps) -> Html {
    let cards = props.cards.clone();
    let layers = cards.len() / props.nr;
    let nr = props.nr;
    html! {
        <div class="snake">
            {
                (0..layers).map(|layer| {
                    let mut elements = vec![];

                    elements.push(match layer {
                        0 => ElementType::EmptyRight,
                        l if l == layers - 1 && l % 2 == 1 => ElementType::EmptyRight,
                        l if l % 2 == 1 => ElementType::RightDown,
                        _ => ElementType::RightUp,
                    });

                    elements.push(ElementType::Straight);

                    for i in (0..nr).map(|i| layer * nr + if layer % 2 == 0 { i } else { 2 - i }) {
                        elements.push(ElementType::Card(cards[i].clone()));
                        elements.push(ElementType::Straight);
                    }

                    elements.push(match layer {
                        l if l == layers - 1 && l % 2 == 0 => ElementType::EmptyLeft,
                        l if l % 2 == 0 => ElementType::LeftDown,
                        _ => ElementType::LeftUp,
                    });

                    elements.iter().map(|e| html! { <Element element_type={e.clone()} /> }).collect::<Html>()
                }).collect::<Html>()
            }
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    pub title: String,
    pub content: String,
}

#[derive(Clone, PartialEq)]
enum ElementType {
    Card(CardProps),
    EmptyRight,
    EmptyLeft,
    Straight,
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
}

#[derive(Clone, PartialEq, Properties)]
struct ElementProps {
    element_type: ElementType,
}

#[function_component(Card)]
fn card(props: &CardProps) -> Html {
    html! {
        <div class="card justify-center items-center flex">
            <Modal>
                <h3>{props.title.clone()}</h3>
                <p class="text-sm text-gray-600">
                    {props.content.clone()}
                </p>
            </Modal>
        </div>
    }
}

#[function_component(Element)]
fn turn(props: &ElementProps) -> Html {
    if let ElementType::Card(card_props) = &props.element_type {
        return html! { <Card title={card_props.title.clone()} content={card_props.content.clone()} /> };
    } else {
        html! {
            <div class="snake-turn">
                {
                    match props.element_type {
                        ElementType::EmptyRight => html! { <><div class="empty" /><div class="straight" /></> },
                        ElementType::EmptyLeft => html! { <><div class="straight" /><div class="empty" /></> },
                        ElementType::Straight => html! { <div class="straight" /> },
                        ElementType::LeftDown => html! { <div class="left-down" /> },
                        ElementType::LeftUp => html! { <div class="up-left" /> },
                        ElementType::RightDown => html! { <div class="right-down" /> },
                        ElementType::RightUp => html! { <div class="up-right" /> },
                        _ => html! { <div class="empty" /> },
                    }
                }
            </div>
        }
    }
}
