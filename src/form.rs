use crate::html::Input;
use leptos::ev::SubmitEvent;
use leptos::*;

const TITLE: &'static str = "The Form";
const DESCRIPTION: &'static str = concat!(
  "This example show how to handle a form\n",  
  "There are 2 ways to handle the form. You can have an input to be fully managed by the state of the signal. This aproach with update to dom for every time a user types in the field.\n",
  "The second way to handle this case in the manual one. Here we update the value on the signal by clicking the submit button.");

#[component]
pub fn form(cx: Scope) -> impl IntoView {
    let (read_name, set_name) = create_signal(cx, "managed name".to_string());
    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element()
            .expect("<input> to exist")
            .value();
        set_name(value);
    };
    view! {cx,
        <div class="card w-96 bg-base-300">
            <form class="card-body" on:submit=on_submit>
                <div class="card-title">{{TITLE}}</div>
                {{DESCRIPTION}}
                <input type="text" placeholder="Type here"  class="input w-full max-w-xs" on:input=move |e| {set_name(event_target_value(&e))} value=read_name/>
                <input node_ref=input_element type="text" placeholder="Type here" class="input w-full max-w-xs"/>
                <div class="card-actions p-2 items-center">
                <p>"The name is: " <strong>{read_name}</strong></p>
                <input type="submit" value="Submit" class="btn btn-primary"/>
                </div>
            </form>
        </div>
    }
}
