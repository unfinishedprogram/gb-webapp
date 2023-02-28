use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        let a = create_signal(cx, 0);
        let b = create_signal(cx, 0);
        let result = create_memo(cx, || *a.get() * *b.get());

        view! { cx,
            button(
                on:click= |_| a.set(*a.get() + 1)
            ) {(a.get())}

            span { "*" }

            button(
                on:click= |_| b.set(*b.get() + 1)
            ) {(b.get())}

            span { "=" }
            span { (result.get()) }
        }
    });
}
