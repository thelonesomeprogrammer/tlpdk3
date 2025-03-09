use crate::comp::robot::Robot;
use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    return html! {
        <div>
            <h2>{"hjemmesiden er stadig under construction"}</h2>
            <Robot/>
        </div>
    };
}
