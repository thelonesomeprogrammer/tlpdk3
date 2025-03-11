use crate::comp::robot::Robot;
use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    return html! {
        <div style="display: flex; justify-content: center; align-items: center; height: 100vh;">
            <Robot angle1={-20.0} angle2={-25.0} angle3={-6.0} />
            <div><h1>{"The Lonesome Programmer"}</h1></div>
            <Robot angle1={20.0} angle2={25.0} angle3={6.0} />
        </div>
    };
}
