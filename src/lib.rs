
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use js_sys;
use std::ops::Add;
use web_sys::console;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, ImageData};


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


    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;


    {
        // let ctx = ctx.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x: f64 = -1.5 + ((event.offset_x() as f64 / WIDTH as f64) * 3.0);
            let y: f64 = 1.5 - ((event.offset_y() as f64 / HEIGHT as f64) * 3.0);
            console::log(&mut js_sys::Array::from(&JsValue::from_str(&format!("x: {}, y: {}", x, y))).into());
            draw(&ctx, WIDTH, HEIGHT, x, y).expect("bad draw");
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    Ok(())
}
#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
) -> Result<(), JsValue> {
    // The real workhorse of this algorithm, generating pixel data
    let c = Complex { real, imaginary };
    let mut data = get_julia_set(width, height, c);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn get_julia_set(width: u32, height: u32, c: Complex) -> Vec<u8> {
    let mut data = Vec::new();

    let param_i = 1.5;
    let param_r = 1.5;
    let scale = 0.005;

    for x in 0..width {
        for y in 0..height {
            let z = Complex {
                real: y as f64 * scale - param_r,
                imaginary: x as f64 * scale - param_i,
            };
            let iter_index = get_iter_index(z, c);
            data.push((iter_index / 4) as u8);
            data.push((iter_index / 2) as u8);
            data.push(iter_index as u8);
            data.push(255);
        }
    }

    data
}

fn get_iter_index(z: Complex, c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;
    while iter_index < 200 {
        if z.norm() > 2.0 {
            break;
        }
        z = z.square() + c;
        iter_index += 1;
    }
    iter_index
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn square(self) -> Complex {
        let real = (self.real * self.real) - (self.imaginary * self.imaginary);
        let imaginary = 2.0 * self.real * self.imaginary;
        Complex { real, imaginary }
    }

    fn norm(&self) -> f64 {
        (self.real * self.real) + (self.imaginary * self.imaginary)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}
