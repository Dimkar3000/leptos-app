mod counter;
mod list;
mod slider;
mod form;

use crate::counter::*;
use crate::list::*;
use crate::slider::*;
use crate::form::*;

use leptos::*;

#[component]
fn app(cx: Scope) -> impl IntoView {
    view! {cx,

    <div class="grid grid-cols-3 p-4 gap-4">
        <Counter/>
        <Slider/>
        <Form/>
        <List/>
    </div>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App />})
}
