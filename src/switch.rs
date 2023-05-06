use leptos::*;

const TITLE: &'static str = "The Switch";
const DESCRIPTION: &'static str =
    "This example shows how control flow works. A switch will handle whether ButtonA or ButtonB is shown";

#[component]
pub fn switch(cx: Scope) -> impl IntoView {
    let (option, set_option) = create_signal(cx, true);

    view! {cx,
        <div class="card w-96 bg-base-300">
            <div class="card-body">
                <div class="card-title">{{TITLE}}</div>
                {{DESCRIPTION}}
                <input type="checkbox" class="toggle" prop:checked=option on:input=move |_| {set_option.update(|x| {*x = !*x;})} />
                <p>
                {move || if option() {
                   view! {cx, <button class="btn btn-primary">"Button A"</button>}
                } else {
                   view! {cx, <button class="btn btn-accent">"Button B"</button>}
                }}
                </p>
                <div class="card-actions p-2 items-center">
                    <p class="flex-1 text-end">"the value is: "{option}</p>
                </div>
            </div>
        </div>
    }
}
