mod art_plum;

use wasm_bindgen::prelude::*;
use web_sys::window;

/// Entry point: find mount points in the DOM and hydrate WASM islands.
#[wasm_bindgen(start)]
pub fn main() {
    let window = window().expect("no window");
    let document = window.document().expect("no document");

    // Mount art background if present
    if let Some(art_mount) = document.get_element_by_id("art-mount") {
        if let Some(art_type) = art_mount.get_attribute("data-art") {
            let art = if art_type == "random" {
                let rand = js_sys::Math::random();
                if rand < 0.33 {
                    "plum"
                } else if rand < 0.67 {
                    "dots"
                } else {
                    "dust"
                }
            } else {
                &art_type
            };

            match art {
                "plum" => art_plum::init(&art_mount),
                "dots" => {
                    // TODO: Implement ArtDots
                    web_sys::console::log_1(&"ArtDots not yet implemented".into());
                }
                "dust" => {
                    // TODO: Implement ArtDust
                    web_sys::console::log_1(&"ArtDust not yet implemented".into());
                }
                _ => {}
            }
        }
    }
}
