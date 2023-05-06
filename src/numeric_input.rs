use leptos::*;

const TITLE: &'static str = "The Numeric Input";
const DESCRIPTION: &'static str =
    "This example shows how to handle errors with an iput that only accept.";

#[component]
pub fn numeric_input(cx: Scope) -> impl IntoView {
    let (get_value, set_value) = create_signal(cx, Ok(0));
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {cx,
        <div class="card w-96 bg-base-300">
            <div class="card-body">
                <div class="card-title">{{TITLE}}</div>
                {{DESCRIPTION}}
                <div class="card-actions p-2 items-center">
                    <div class="form-control w-full">
                        <label class="label">
                        "Type a number (or not!)"
                        </label>
                        <input on:input=on_input value=get_value.get().unwrap() class="input input-bordered w-full" class:border-error=move || get_value.get().is_err()/>
                    </div>
                    <ErrorBoundary fallback=|cx, errors| view!{ cx,
                        <div class="text-error">
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                    }>
                    <p class="text-success">
                        "You entered "
                        <strong>{get_value}</strong>
                    </p>
                    </ErrorBoundary>
                </div>
            </div>
        </div>
    }
}
