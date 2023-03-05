use std::marker;

use yew::prelude::*;


struct Home;

struct Links;

pub struct Page<P> {
    marker: marker::PhantomData<P>
}

struct Projects;

impl Page<()> {
    #[inline]
    pub fn home() -> Html {
        Page::<Home>::view()
    }

    #[inline]
    pub fn links() -> Html {
        Page::<Links>::view()
    }

    #[inline]
    pub fn project() -> Html {
        Page::<Projects>::view()
    }
}

impl Page<Home> {
    fn view() -> Html {
        html! {
            <>
                <h3>{"I am,"}</h3>
                <br/>
                <h2>{"Saad Kondvilkar"}</h2>
                <p>{"A.K.A. Segmentation Violator"}</p>
            </>
        }
    }
}

impl Page<Links> {
    fn view() -> Html {
        html!("todo")
    }
}

impl Page<Projects> {
    fn view() -> Html {
        html!("todo")
    }
}