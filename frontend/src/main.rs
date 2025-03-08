mod comp;
mod pages;

use pages::{home::Home, projects::Projects};
use yew::prelude::{classes, function_component, html, Html};
use yew_router::prelude::{BrowserRouter, Link, Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Projects => html! {<Projects />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
            <BrowserRouter>
                <Nav />
                <Switch<Route> render={switch} />
            </BrowserRouter>
    }
}

#[function_component(Nav)]
fn nav() -> Html {
    return html! {
        <nav class="navbar">
            <section class="navbar-section logo">
                <Link<Route> to={Route::Home}>
                    <img src="/images/new-with-text.png" alt="Logo" class="w-30 h-40" />
                </Link<Route>>
            </section>
            <section class="navbar-section desktop-menu">
                <nav class="dropmenu animated">
                    <ul class={classes!("navbar-menu")}>
                        <li class={classes!("navbar-item")}><Link<Route> to={Route::Home}>{ "Hjem" }</Link<Route>></li>
                        <li class={classes!("navbar-item")}><Link<Route> to={Route::Projects}>{ "projects" }</Link<Route>></li>
                    </ul>
                </nav>
            </section>
            <div class="mobile-menu">
                <div class="button_container" id="toggle">
                    <span class="top"></span>
                    <span class="middle"></span>
                    <span class="bottom"></span>
                </div>
            </div>
        </nav>

    };
}
