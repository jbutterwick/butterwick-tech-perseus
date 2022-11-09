use sycamore::prelude::*;

#[component]
pub fn Counter<G: Html>(cx: Scope) -> View<G> {
    let state = create_signal(cx, 0i32);
    let increment = |_| state.set(*state.get() + 1);
    let decrement = |_| state.set(*state.get() - 1);
    let reset = |_| state.set(0);
    view! { cx,
        div {
            h2 { "Value: " (state.get()) }
            button(on:click=increment) { "+" }
            button(on:click=decrement) { "-" }
            button(on:click=reset) { "Reset" }
        }
    }
}
