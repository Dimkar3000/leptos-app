use leptos::*;

const TITLE: &'static str = "The List";
const DESCRIPTION: &'static str =
    "this example shows how to iterate through a list of stuff and create the layout of a list";

#[component]
fn static_list(cx: Scope) -> impl IntoView {
    let values = vec!["item 1", "item 2", "item 3"];
    view! {cx,
      <ul class="menu bg-base-100 w-full">
        <li class="menu-title">
          <span>"Static List:"</span>
        </li>
        {
          values.into_iter()
          .map(|n| view! { cx, <li><a>{n}</a></li>})
          .collect::<Vec<_>>()
        }
      </ul>
    }
}

#[component]
fn dynamic_list(cx: Scope, initial_length: usize) -> impl IntoView {
    let mut next_counter = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(cx, id + 1)))
        .collect::<Vec<_>>();

    let (read_counters, set_counters) = create_signal(cx, initial_counters);
    let add_counter = move |_| {
        let signal = create_signal(cx, next_counter + 1);

        set_counters.update(move |counters| counters.push((next_counter, signal)));

        next_counter += 1;
    };

    view! {cx,
      <div>
      <div class="flex justify-between items-center">
      "Dynamic Counter List"
      <button on:click=add_counter class="btn btn-primary btn-circle m-2">"+"</button>
      </div>
      <ul class="menu bg-base-100 w-full">
        <For
          each=read_counters
          key=|counter| counter.0
          view=move |cx, (id, (count, set_count))| {
            view! { cx,
                    <li class="flex flex-row">
                      <button class="flex-1" on:click=move |_| set_count.update(|n| *n += 1)>
                          {count}
                      </button>
                      <button
                      class="btn btn-accent rounded-none hover:bg-accent-focus text-black"
                          on:click=move |_| {
                              set_counters.update(|counters| {
                                  counters.retain(|(counter_id, _)| counter_id != &id)
                              });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
      </div>
    }
}

#[component]
pub fn list(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="card w-96 bg-base-300">
            <div class="card-body">
                <div class="card-title">{{TITLE}}</div>
                {{DESCRIPTION}}
                <StaticList />
                <DynamicList initial_length=5/>
                <div class="card-actions p-2 flex-col">
                </div>
            </div>
        </div>
    }
}
