use sycamore::prelude::*;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[derive(Prop)]
pub struct ScreenProps<'a> {
    buffer: &'a ReadSignal<Vec<u8>>,
}

// Displays the given frame-buffer
#[component]
pub fn Screen<'a, G: Html>(cx: Scope<'a>, props: ScreenProps<'a>) -> View<G> {
    let canvas_ref = create_node_ref(cx);

    let canvas_ctx = create_signal::<Option<CanvasRenderingContext2d>>(cx, None);

    on_mount(cx, || {
        let node = canvas_ref.get::<DomNode>();
        let node = node.to_web_sys();
        let canvas: HtmlCanvasElement = node.dyn_into().map_err(|_| ()).expect("Not a canvas");
        let ctx = canvas
            .get_context("2d")
            .expect("Can't get context")
            .expect("Can't get context");

        let ctx: CanvasRenderingContext2d = ctx.dyn_into().expect("Not a rendering context");
        canvas_ctx.set(Some(ctx));
    });

    create_effect(cx, || {
        let buffer = props.buffer.get();
        if buffer.len() > 0 {
            let img_data = ImageData::new_with_u8_clamped_array(Clamped(&props.buffer.get()), 160)
                .expect("Wrong buffer size");

            if let Some(ctx) = canvas_ctx.get().as_ref() {
                ctx.put_image_data(&img_data, 0.0, 0.0)
                    .expect("Failed to put image data");
            }
        }
    });

    view! { cx,
        canvas(ref=canvas_ref, id="screen")
    }
}
