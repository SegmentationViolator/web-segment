use std::fmt;

use yew::prelude::*;
use yew_router::prelude::*;

mod footer;
mod navigation_bar;
mod page;

use navigation_bar::NavigationBar;
use footer::Footer;

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/links")]
    Links,
}

impl Route {
    pub const ALL: [Self; 3] = [
        Self::Home,
        Self::Links,
        Self::Projects,
    ];
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[function_component(App)]
fn app() -> Html {
    yew::html! {
        <div id="App">
            <BrowserRouter>
                <NavigationBar />
                <br/>
                <div class="body">
                    <Switch<Route> render={switch} />
                </div>
                <Footer />
            </BrowserRouter>
        </div>
    }
}

fn switch(route: Route) -> Html {
    let document = web_sys::window().unwrap().document().unwrap();
    let app = document.get_element_by_id("App").unwrap();
    match app.class_name().as_str() {
        "fade-again" => app.set_class_name("fade"),
        _  => app.set_class_name("fade-again"),
    }

    match route {
        Route::Home => page::Page::home(),
        Route::Links => page::Page::links(),
        Route::Projects => page::Page::project(),
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
