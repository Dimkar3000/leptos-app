use leptos::*;

const TITLE: &'static str = "The Counter";
const DESCRIPTION: &'static str =
    "This example demonstrates how to create a signal and event listeners that update its state";

#[component]
pub fn counter(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! {cx,
        <div class="card w-96 bg-base-300">
            <div class="card-body">
                <div class="card-title">{{TITLE}}</div>
                {{DESCRIPTION}}
                <div class="card-actions p-2 items-center">
                    <button class="btn btn-circle flex-shrink-0" on:click=move |_|{set_count.update(|n| *n += 1);}>"+1" </button>
                    <button class="btn btn-circle flex-shrink-0" on:click=move |_|{set_count.update(|n| *n -= 1);}>"-1" </button>
                    <p class="flex-1 text-end">"the value is: "{move || count.get()}</p>
                </div>
            </div>
        </div>
    }
}
