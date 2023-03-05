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

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children
}

#[function_component(Paragraph)]
pub fn footer(properties: &Props) -> Html {
    html! {
        <div class="paragraph">
            <pre>{"    "}</pre>
            { for properties.children.iter() }
        </div>
    }
}
