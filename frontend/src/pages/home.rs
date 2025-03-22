use crate::comp::robot::Robot;
use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    return html! {
        <div>
            <div class="title">
                <div class="flex justify-center gap-4 title-inner">
                    <Robot angle1={-20.0} angle2={-25.0} angle3={-6.0} />
                    <h1 class="leading-none">
                        {"The"}<br/>
                        <span class="lonesome"> {"Lonesome"} </span><br/>
                        {"Programmer"}
                    </h1>
                    <Robot angle1={20.0} angle2={25.0} angle3={6.0} />
                </div>
            </div>
            /*<div class="content" style="margin-top: 2rem; height: 100vh;">
                <p>
                    {
                        "Welcome to my personal website! I'm a software engineer with a passion for web development,
                            and robotics. I'm currently working on a few projects, including a vesual pos system
                            and a robot arm. Feel free to check out my projects"
                    }
                </p>
            </div>*/
        </div>
    };
}
