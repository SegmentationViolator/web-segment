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