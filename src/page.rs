// web segment - a personal website used to host some text files and my portfolio
// Copyright (C) 2023  Saad Kondvilkar

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::marker;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{paragraph::Paragraph, Route};

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
                <h4>{"I am,"}</h4><br/>

                <h3>{"Saad Kondvilkar"}</h3>
                <h5>{"A.K.A. Segmentation Violator"}</h5><br/>

                <Paragraph>
                    <p>
                        {"I am a hobbyist programmer, my interests in the field of programming are systems programming, graphics programming and web development. "}
                        {"I like making software that I find interesting, and learn-by-doing"}
                    </p>
                </Paragraph><br/>

                <div style="margin: 1em;">
                    <h4>{"Programming Languages Known To Me:"}</h4>
                    <ul>
                        <li>{"Rust"}</li>
                        <li>{"Python"}</li>
                    </ul><br/>
                </div>
                    
                <Paragraph>
                    <p>
                        {"I have worked on projects like Discord bots, a programming language, an emulator and a text editor. I have listed some on my projects "}
                        <Link<Route> to={Route::Projects}>{"here"}</Link<Route>>
                        {" and you can find links to my profiles on various platforms and my E-mail "}
                        <Link<Route> to={Route::Links}>{"here"}</Link<Route>>
                        </p>
                </Paragraph>
            </>
        }
    }
}

impl Page<Links> {
    fn view() -> Html {
        html! {
            <ul>
                <li>
                    <h4 style="display: inline;">{"Github"}</h4>
                    <sup><a href="https://github.com/SegmentationViolator">{"[link]"}</a></sup>
                </li>
                <li>
                    <h4 style="display: inline;">{"E-mail: "}</h4><h4 style="display: inline;">{"segmentationviolator@proton.me"}</h4>
                </li>
            </ul>
        }
    }
}

impl Page<Projects> {
    fn view() -> Html {
        html!("todo")
    }
}