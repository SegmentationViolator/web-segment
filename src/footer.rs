use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="footer">
            <p>
                <a href="https://www.rust-lang.org/">{"Made with Crab Lang"} <img src="crab.webp"/></a>
                <sup><a href="https://github.com/">{"[source]"}</a></sup>
            </p>
        </div>
    }
}
