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

/*

html, body, div { display: grid }
html { min-height: 100% }
body, div { place-content: center }

body {
    grid-gap: 1.75em;
    grid-template-columns: repeat(auto-fit, Min(100%, 10em));
    background: #1d031f;
    color: #fff;
    font: 200 1.75em poppins;
    text-shadow: 1px 1px 2px
}

.box {
    --list: #ffbc00, #ff0058;
    --grad: linear-gradient(45deg, var(--list));
    position: relative;
    border: solid 4px #0000;
    aspect-ratio: 7/ 10;
    border-radius: 1em;
    background: conic-gradient(rgb(0 0 0/ .75) 0 0) padding-box,
        var(--grad) border-box;

    &:nth-child(2) { --list: #03a9f4, #ff0058 }
    &:nth-child(3) { --list: #4dff03, #00d0ff }

    &::before {
        position: absolute;
        inset: 0;
        z-index: -1;
        background: var(--grad);
        filter: blur(.75em);
        content: ''
    }
}

*/
