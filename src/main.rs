mod counter;
mod list;
mod slider;
mod form;
mod switch;

use crate::counter::*;
use crate::list::*;
use crate::slider::*;
use crate::form::*;
use crate::switch::*;

use leptos::*;

#[component]
fn app(cx: Scope) -> impl IntoView {
    view! {cx,

    <div class="grid grid-cols-3 p-4 gap-4">
        <Counter/>
        <Slider/>
        <Switch/>
        <Form/>
        <List/>
    </div>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App />})
}
