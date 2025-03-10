use yew::prelude::*;

#[function_component(Robot)]
pub fn robot() -> Html {
    let angle1: f32 = 0.0f32.to_radians();
    let angle2: f32 = 90.0f32.to_radians();
    let angle3: f32 = 90.0f32.to_radians();

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
    let joint = format!("transform: translate({}px, {}px)", pos2[0], pos2[1],);
    let link2 = format!(
        "transform: translate({}px, {}px) rotate({}rad);",
        pos2[0],
        pos2[1],
        -angle2 - angle1
    );
    let link3 = format!(
        "transform: translate({}px, {}px) rotate({}rad);",
        pos3[0],
        pos3[1],
        -angle3 - angle2 - angle1
    );

    html! {
        <div style="display: flex; justify-content: center; align-items: center;">
            <div class="robot">
                <div class="base"></div>
                <div class="link" style={link1}></div>
                <div class="joint" style={joint}></div>
                <div class="link" style={link2}></div>
                <div class="grip" style={link3}><div></div></div>
            </div>
        </div>
    }
}
