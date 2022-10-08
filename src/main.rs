use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        let state = create_signal(cx, 0i32);
        let increment = |_| state.set(*state.get() + 1);
        let decrement = |_| state.set(*state.get() - 1);

        view! { cx,
            div(style="display: grid; place-items: center; height: 100vh") {
                div(style="display: grid; place-items: center") {
                    p (style="color: blue; font-size: 20px"){"" (state.get())}
                    div(style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px") {
                        button (on:click=increment) { "+" }
                        button (on:click=decrement) { "-" }
                    }
                }
            }
        }
    });
}
