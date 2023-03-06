use std::{cell::RefCell, sync::mpsc::Receiver};

use futures_util::{Stream, StreamExt};
use gloo::{console::log, timers::future::IntervalStream};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{debugger::Debugger, rom_downloader::download_rom, screen::Screen};

#[component(inline_props)]
pub async fn GBView<G: Html, 'a>(cx: Scope<'a>, rom_url: Receiver<Vec<u8>>) -> View<G> {
    let buffer = create_signal::<Vec<u8>>(cx, vec![]);
    let debugger = create_signal(cx, RefCell::new(Debugger::new()));
    let frame_stream = IntervalStream::new(16);

    // Convert a stream into a future that runs forever.
    spawn_local_scoped(cx, async {
        for rom in rom_url {
            debugger.get().borrow_mut().load_rom(&rom)
        }
    });

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

    view!(cx, div {
        Screen(buffer = buffer)
    })
}
