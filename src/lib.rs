
use wasm_bindgen::prelude::*;

use js_sys;
use web_sys::console;
use wasm_bindgen::JsCast;

mod julia;



const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let doc = web_sys::window().unwrap().document().unwrap();
    let canvas = doc
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    doc.body().unwrap().append_child(&canvas)?;

    canvas.set_width(WIDTH);
    canvas.set_height(HEIGHT);
    canvas.style().set_property("border", "solid")?;

    let ctx = julia::get_context(&canvas)?;


    {
        // let ctx = ctx.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x: f64 = -1.5 + ((event.offset_x() as f64 / WIDTH as f64) * 3.0);
            let y: f64 = 1.5 - ((event.offset_y() as f64 / HEIGHT as f64) * 3.0);
            console::log(&mut js_sys::Array::from(&JsValue::from_str(&format!("x: {}, y: {}", x, y))).into());
            julia::draw(&ctx, WIDTH, HEIGHT, x, y).expect("bad draw");
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    Ok(())
}
