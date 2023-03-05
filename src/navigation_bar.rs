use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(NavigationBar)]
pub fn navigation_bar() -> Html {
    let current_route: Route = use_route().unwrap();

    let segmented = yew::use_state_eq(|| false);
    let set_segmented = {
        let segmented = segmented.clone();
        Callback::from(move |_| segmented.set(true))
    };

    let pages = Route::ALL
        .iter()
        .filter(|route| &current_route != *route)
        .map(|route| html! { <li><Link<Route> classes={classes!("nav-link")} to={route.clone()} > {route} </Link<Route>></li> })
        .collect::<Html>();

    let mut classes = classes!("nav-bar");
    segmented.then(|| classes.push("segmented"));

    html! {
        <div class={classes}>
            <h1>{"Web "}<span onclick={set_segmented}>{"Segment"}</span></h1>
            <ui class={classes!("nav-links")}>
                { pages }
            </ui>
        </div>
    }
}
