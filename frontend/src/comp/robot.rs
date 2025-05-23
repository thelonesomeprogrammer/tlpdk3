use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct RobotProps {
    pub angle1: f32,
    pub angle2: f32,
    pub angle3: f32,
}

#[function_component(Robot)]
pub fn robot(props: &RobotProps) -> Html {
    let angle1 = props.angle1.to_radians();
    let angle2 = props.angle2.to_radians();
    let angle3 = props.angle3.to_radians();
    let pos1 = [0.0, 0.0];
    let pos2 = [angle1.sin() * 80.0, angle1.cos() * 80.0];
    let pos3 = [
        angle1.sin() * 80.0 + (angle1 + angle2).sin() * 80.0,
        angle1.cos() * 80.0 + (angle1 + angle2).cos() * 80.0,
    ];

    let link1 = format!(
        "transform: translate({}px, {}px) rotate({}rad); ",
        pos1[0], pos1[1], -angle1
    );
    let joint1 = format!(
        "transform: translate({}px, {}px)",
        pos2[0],
        pos2[1] + 3.0 + 2.5 * angle1.sin(),
    );
    let link2 = format!(
        "transform: translate({}px, {}px) rotate({}rad);",
        pos2[0],
        pos2[1],
        -angle2 - angle1
    );
    let joint2 = format!(
        "transform: translate({}px, {}px)",
        pos3[0],
        pos3[1] + 3.0 + 2.5 * angle1.sin(),
    );
    let link3 = format!(
        "transform: translate({}px, {}px) rotate({}rad);z-index: -1",
        pos3[0],
        pos3[1] + 2.5 + 2.5 * angle1.sin(),
        -angle3 - angle2 - angle1
    );

    html! {
        <div class="robot-container">
            <div class="robot">
                <div class="base"></div>
                <div class="link" style={link1}></div>
                <div class="joint" style={joint1}></div>
                <div class="link" style={link2}></div>
                <div class="joint" style={joint2}></div>
                <div class="grip" style={link3}></div>
            </div>
        </div>
    }
}
