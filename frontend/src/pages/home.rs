use crate::comp::modal::Modal;
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
            <div class="content pt-4 ps-4 pe-4">
                <Modal>
                    <p>
                        {
                            "Velkommen til min personlige hjemmeside!
                            Jeg er en softwareudvikler og robotingeniør under uddannelse.
                            Jeg har erfaring inden for cloud-native systemer, embedded udvikling og robotteknologi.
                            Jeg elsker at bygge effektive og innovative løsninger, både i skyen og på bare metal.
                            På dette site kan du se nogle af mine projekter, erfaringer og de teknologier, jeg arbejder med."
                        }
                    </p>
                </Modal>
                <div class="content modal-line-container flex justify-center mt-8 mb-8">
                    <Modal>
                        <h2 class="ex-title">{"Mine Erfaringer"}</h2>
                        <div class="flex justify-evenly">
                            <Modal>
                                <h3>{"Linux"}</h3>
                                <ul class="layered-list">
                                    <li>
                                        {"Arch Linux - Desktop"}
                                        <ul>
                                            <li>{"3+ år"}</li>
                                        </ul>
                                    </li>
                                    <li>
                                        {"Arch Linux - Server"}
                                        <ul>
                                            <li>{"1+ år"}</li>
                                        </ul>
                                    </li>
                                    <li>
                                        {"NixOS - Server"}
                                        <ul>
                                            <li>{"1+ år"}</li>
                                        </ul>
                                    </li>
                                </ul>
                            </Modal>
                            <Modal>
                                <h3>{"programmering"}</h3>
                                <ul class="layered-list">
                                    <li>
                                        {"Python"}
                                        <ul>
                                            <li>{"4+ år"}</li>
                                        </ul>
                                    </li>
                                    <li>
                                        {"Rust"}
                                        <ul>
                                            <li>{"3+ år"}</li>
                                        </ul>
                                    </li>
                                    <li>
                                        {"C/C++"}
                                        <ul>
                                            <li>{"2+ år"}</li>
                                        </ul>
                                    </li>
                                </ul>
                            </Modal>
                            <Modal>
                                <h3>{"frameworks"}</h3>
                                <ul class="layered-list">
                                    <li>
                                        {"Arduino"}
                                        <ul>
                                            <li>{"2+ år"}</li>
                                        </ul>
                                    </li>
                                    <li>
                                        {"Yew.rs"}
                                        <ul>
                                            <li>{"2+ år"}</li>
                                        </ul>
                                    </li>
                                    <li>
                                        {"ROS2"}
                                        <ul>
                                            <li>{"1+ år"}</li>
                                        </ul>
                                    </li>
                                </ul>
                            </Modal>
                            <Modal>
                                <h3>{"Arbejde"}</h3>
                                <ul class="layered-list">
                                    <li>
                                        {"Raklame omdeler"}
                                        <ul>
                                            <li>{"1 sommer"}</li>
                                        </ul>
                                    </li>
                                    <li>
                                        {"fajedreng"}
                                        <ul>
                                            <li>{"2+ år"}</li>
                                        </ul>
                                    </li>
                                    <li>
                                        {"Servicemedarbejder"}
                                        <ul>
                                            <li>{"2+ år"}</li>
                                        </ul>
                                    </li>
                                </ul>
                            </Modal>
                        </div>
                    </Modal>
                </div>
            </div>
        </div>
    };
}
