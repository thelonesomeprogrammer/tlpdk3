use crate::comp::modal::Modal;
use serde::Deserialize;
use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Clone, Deserialize)]
pub struct LayeredListItems {
    pub name: String,
    pub items: Vec<String>,
}

#[derive(Properties, PartialEq, Clone, Deserialize)]
pub struct LayeredListCardProps {
    pub title: String,
    pub items: Vec<LayeredListItems>,
}

#[function_component(LayeredListCard)]
pub fn main(props: &LayeredListCardProps) -> Html {
    html! {
        <Modal>
            <h3>{props.title.clone()}</h3>
            <ul class="layered-list">
                {
                    for props.items.iter().map(|item| {
                        html! {
                            <li>
                                {item.name.clone()}
                                <ul>
                                    { for item.items.iter().map(|item| html! { <li>{item}</li> }) }
                                </ul>
                            </li>
                        }
                    })
                }
            </ul>
        </Modal>
    }
}
