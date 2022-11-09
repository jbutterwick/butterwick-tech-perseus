use chrono::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn Hello<G: Html>(cx: Scope) -> View<G> {
    let name = create_signal(cx, String::new());

    let displayed_name = || {
        if name.get().is_empty() {
            "World".to_string()
        } else {
            name.get().as_ref().clone()
        }
    };

    let day = create_signal(cx, String::new());

    let displayed_date = || {
        if day.get().is_empty() {
            Utc::now().format("%Y-%m-%d").to_string()
        } else {
            day.get().as_ref().clone()
        }
    };

    view! { cx,
        div {
            h1 {"Hello "(displayed_name())"!"}
            input(placeholder="What is your name?", bind:value=name)
            p {(displayed_date())}
            input(type="date", bind:value=day)
        }
    }
}
