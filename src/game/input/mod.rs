pub use gamepad_controls::GameGamepadControlsPlugin;
#[cfg(target_arch = "wasm32")]
pub use html_controls::GameHtmlControlsPlugin;
pub use keyboard_controls::GameKeyboardControlsPlugin;

mod gamepad_controls;
#[cfg(target_arch = "wasm32")]
mod html_controls;
mod keyboard_controls;
