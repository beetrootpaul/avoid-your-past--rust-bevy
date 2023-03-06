pub use gamepad_controls::GameGamepadControlsPlugin;
pub use keyboard_controls::GameKeyboardControlsPlugin;
#[cfg(target_arch = "wasm32")]
pub use html_controls::GameHtmlControlsPlugin;

mod gamepad_controls;
mod keyboard_controls;
#[cfg(target_arch = "wasm32")]
mod html_controls;
