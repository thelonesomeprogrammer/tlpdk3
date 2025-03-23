use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(Modal)]
pub fn main(props: &Props) -> Html {
    return html! {
        <div class="modal">
            <div class="modal-inner">
                {props.children.clone()}
            </div>
        </div>
    };
}
