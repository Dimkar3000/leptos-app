mod counter;
mod list;
mod slider;

use crate::counter::*;
use crate::list::*;
use crate::slider::*;

use leptos::*;

#[component]
fn app(cx: Scope) -> impl IntoView {
    view! {cx,

    <div class="grid grid-cols-3 p-4">
        <Counter/>
        <Slider/>
        <List/>
    </div>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App />})
}
