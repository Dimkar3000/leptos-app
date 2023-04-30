use leptos::{html::Input, *};

const TITLE: &'static str = "The Slider";
const DESCRIPTION: &'static str = concat!(
    "This example show how to create child components and values with props to them.\n",
    "The slider is a child component that takes a signal to write a value to.\n",
    "The framework supports optional props.\n",
    "The progress is a percentence of the slider value and is made using a derived function from the signal and show the value halved."
);

/// The Slider is awesome
#[component]
pub fn slider(cx: Scope) -> impl IntoView {
    let (read_count, write_count) = create_signal(cx, 0);
    let double_count = move || read_count() / 4 + 25;

    view! {cx,
        <div class="card w-96 bg-base-300">
            <div class="card-body">
                <div class="card-title">{{TITLE}}</div>
                {{DESCRIPTION}}
                <div class="card-actions p-2 flex-col">
                  <ProgressBar progress=write_count initial_value=read_count.get()/>
                  <progress class="progress progress-secondary w-full" value=double_count max=100/>
                  <p class="flex-1 text-end w-full">"the value is: "{move || read_count.get()}</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProgressBar(
    cx: Scope,
    initial_value: i32,
    progress: WriteSignal<i32>,
    #[prop(default = 100)] max: i32,
    #[prop(default = -100)] min: i32,
) -> impl IntoView {
    let changeRef: NodeRef<Input> = create_node_ref(cx);
    let change = move |_: ev::Event| {
        let value = changeRef().expect("").value();
        progress.set(value.parse::<i32>().expect(""))
    };

    view! {
        cx,
        <input type="range" class="range range-primary" max=max min=min node_ref=changeRef on:input=change value=initial_value/>
    }
}
