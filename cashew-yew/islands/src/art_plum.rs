//! Port of ArtPlum.vue — plum blossom canvas animation.
//!
//! Draws branching lines from the edges of the viewport inward,
//! creating an organic, tree-like pattern.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, Element, HtmlCanvasElement, HtmlElement};

use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

const R90: f64 = PI / 2.0;
const R180: f64 = PI;
const R15: f64 = PI / 12.0;
const COLOR: &str = "#88888825";
const MIN_BRANCH: u32 = 30;
const LEN: f64 = 6.0;

fn polar2cart(x: f64, y: f64, r: f64, theta: f64) -> (f64, f64) {
    let dx = r * theta.cos();
    let dy = r * theta.sin();
    (x + dx, y + dy)
}

fn random() -> f64 {
    js_sys::Math::random()
}

fn random_middle() -> f64 {
    random() * 0.6 + 0.2
}

/// Wrapper struct to break the recursive type alias cycle.
struct Step {
    func: Box<dyn FnOnce(&CanvasRenderingContext2d, f64, f64, &mut Vec<Step>)>,
}

fn make_step(x: f64, y: f64, rad: f64, counter: u32, width: f64, height: f64) -> Step {
    Step {
        func: Box::new(move |ctx: &CanvasRenderingContext2d, w: f64, h: f64, steps: &mut Vec<Step>| {
            let length = random() * LEN;
            let count = counter + 1;
            let (nx, ny) = polar2cart(x, y, length, rad);

            ctx.begin_path();
            ctx.move_to(x, y);
            ctx.line_to(nx, ny);
            ctx.stroke();

            let rad1 = rad + random() * R15 * 1.2;
            let rad2 = rad - random() * R15 * 1.2;

            // Out of bounds check
            if nx < -100.0 || nx > w + 100.0 || ny < -100.0 || ny > h + 100.0 {
                return;
            }

            let rate = if count <= MIN_BRANCH { 0.8 } else { 0.5 };

            if random() < rate {
                steps.push(make_step(nx, ny, rad1, count, width, height));
            }
            if random() < rate {
                steps.push(make_step(nx, ny, rad2, count, width, height));
            }
        }),
    }
}

/// Initialize the plum blossom art on the given mount element.
pub fn init(mount: &Element) {
    let win = window().expect("no window");
    let document = win.document().expect("no document");

    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .unwrap()
        .dyn_into()
        .unwrap();

    let w = win.inner_width().unwrap().as_f64().unwrap();
    let h = win.inner_height().unwrap().as_f64().unwrap();
    let dpr = win.device_pixel_ratio();

    canvas.set_width((w * dpr) as u32);
    canvas.set_height((h * dpr) as u32);

    // Use HtmlElement to access .style()
    let canvas_el: &HtmlElement = canvas.unchecked_ref();
    canvas_el
        .style()
        .set_property("width", &format!("{w}px"))
        .unwrap();
    canvas_el
        .style()
        .set_property("height", &format!("{h}px"))
        .unwrap();
    canvas_el
        .style()
        .set_property("mask-image", "radial-gradient(circle, transparent, black)")
        .unwrap();

    mount.append_child(&canvas).unwrap();

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let _ = ctx.scale(dpr, dpr);
    ctx.set_line_width(1.0);
    ctx.set_stroke_style_str(COLOR);

    // Initial steps from edges
    let mut steps: Vec<Step> = vec![
        make_step(random_middle() * w, h + 5.0, -R90, 0, w, h),
        make_step(-5.0, random_middle() * h, 0.0, 0, w, h),
        make_step(w + 5.0, random_middle() * h, R180, 0, w, h),
        make_step(random_middle() * w, -5.0, R90, 0, w, h),
        make_step(random_middle() * w, h + 5.0, -R90, 0, w, h),
    ];

    if w < 500.0 {
        steps.truncate(2);
    }

    // Run the animation using requestAnimationFrame
    let steps_rc = Rc::new(RefCell::new(steps));
    let ctx_rc = Rc::new(ctx);

    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let steps_clone = steps_rc.clone();
    let ctx_clone = ctx_rc.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        let mut current_steps = steps_clone.borrow_mut();
        let prev_steps: Vec<Step> = current_steps.drain(..).collect();

        if prev_steps.is_empty() {
            return;
        }

        let mut next_steps: Vec<Step> = Vec::new();

        for step in prev_steps {
            if random() < 0.5 {
                next_steps.push(step);
            } else {
                (step.func)(&ctx_clone, w, h, &mut next_steps);
            }
        }

        *current_steps = next_steps;

        if !current_steps.is_empty() {
            web_sys::window()
                .unwrap()
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }
    }));

    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}
