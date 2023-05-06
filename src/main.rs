mod counter;
mod form;
mod list;
mod numeric_input;
mod slider;
mod switch;

use crate::counter::*;
use crate::form::*;
use crate::list::*;
use crate::numeric_input::*;
use crate::slider::*;
use crate::switch::*;

use leptos::*;

#[component]
fn app(cx: Scope) -> impl IntoView {
    view! {cx,

    <div class="grid grid-cols-3 p-4 gap-4">
        <Counter/>
        <Slider/>
        <Switch/>
        <NumericInput/>
        <Form/>
        <List/>
    </div>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App />})
}
