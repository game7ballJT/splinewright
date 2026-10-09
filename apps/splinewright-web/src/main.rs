//! Splinewright in the browser.
//!
//! Runs the same [`splinewright_ui_egui::SplinewrightApp`] as the desktop app through eframe's web
//! runner (wgpu: WebGPU where available, WebGL2 otherwise). Build with `trunk build --release`
//! from this directory; see `docs/development.md` ("Web build").
//!
//! Differences from the desktop app:
//! - no TCP control server (browsers can't listen on sockets);
//! - File → Open uses the browser file picker; bytes arrive asynchronously through
//!   `Services::inbox`;
//! - Save / Export trigger a browser download;
//! - dropped files are read asynchronously by `web::WebShell` and delivered through the inbox.
//!
//! URL query flags: `?open=samples/<name>.splinewright` opens a bundled example (same-origin paths
//! only); `?webgl` forces the WebGL2 backend instead of WebGPU; `?lang=es` (any
//! language code) stands in for the browser's languages when the interface language is Automatic.

#[cfg(target_arch = "wasm32")]
mod locks;
#[cfg(target_arch = "wasm32")]
mod web;

/// Whether `url` is a plain same-origin path (`samples/neon-drive.splinewright`, `/apps/x/a.svg`),
/// never another site or a scheme: `?open=` is for pages that bundle their own examples, and a
/// crafted link must not make the editor fetch from anywhere else.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn is_same_origin_path(url: &str) -> bool {
    !url.is_empty() && !url.starts_with("//") && !url.contains(':') && !url.contains('\\') && !url.split('/').any(|part| part == "..")
}

#[cfg(target_arch = "wasm32")]
fn main() {
    web::start();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("splinewright-web only runs in the browser: build it with `trunk build --release` in apps/splinewright-web");
}

#[cfg(test)]
mod tests {
    #[test]
    fn open_param_accepts_only_same_origin_paths() {
        for ok in ["samples/neon-drive.splinewright", "/apps/splinewright/samples/a.svg", "a.splinewright"] {
            assert!(super::is_same_origin_path(ok), "{ok}");
        }
        for bad in [
            "",
            "https://evil.example/a.svg",
            "//evil.example/a.svg",
            "javascript:alert(1)",
            "data:image/svg+xml;base64,AA",
            "../secret.svg",
            "a/../../b.svg",
            "a\\b.svg",
        ] {
            assert!(!super::is_same_origin_path(bad), "{bad}");
        }
    }
}
