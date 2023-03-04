#![feature(async_closure)]
#![feature(local_key_cell_methods)]

mod screen;

use std::cell::RefCell;

use debugger::Debugger;
use screen::Screen;
mod debugger;
use futures_util::stream::StreamExt;
use gloo::timers::future::IntervalStream;
use sycamore::{futures::spawn_local_scoped, prelude::*};

fn main() {
    console_error_panic_hook::set_once();

    sycamore::render(|cx| {
        let buffer = create_signal::<Vec<u8>>(cx, vec![]);
        let debugger = create_signal(cx, RefCell::new(Debugger::new()));
        let frame_stream = IntervalStream::new(16);

        // Convert a stream into a future that runs forever.
        spawn_local_scoped(cx, async {
            frame_stream
                .for_each(|_| async {
                    let debugger = debugger.get();
                    let mut debugger = debugger.borrow_mut();
                    debugger.step();
                    buffer.set(debugger.get_frame_buffer())
                })
                .await
        });

        view! { cx,
            div {
                Screen(buffer = buffer)
            }
        }
    });
}
