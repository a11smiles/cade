//! CADE desktop shell.
//!
//! The UI (app/ui, React + TypeScript) calls these commands over Tauri IPC.
//! The GPU canvas is a native wgpu surface owned by `cade-render`, placed
//! under the transparent webview (FOUND/CANVAS backlog); until that lands
//! the UI draws a placeholder canvas.

use cade_feedback::{Feedback, IssueDraft};
use cade_policy::{Action, Decision, Policy};
use std::sync::Mutex;

mod command_error;
pub use command_error::CommandError;

#[derive(Default)]
pub struct AppState {
    policy: Mutex<Policy>,
}

// Expand the shared declarations as actual Tauri handlers and register exactly
// that set. The generator consumes the same input without depending on Tauri.
macro_rules! commands {
    ($(
        $(#[$meta:meta])*
        $mode:ident fn $name:ident(
            [$($injected:ident: $injected_ty:ty),*] $(, $arg:ident: $arg_ty:ty)*
        ) -> Result<$result:ty, $error:ty> $body:block
    )*) => {
        $(commands!(@handler $mode $(#[$meta])* fn $name(
            $($injected: $injected_ty,)* $($arg: $arg_ty),*
        ) -> Result<$result, $error> $body);)*

        /// Register the declared commands and state for both real and mock runtimes.
        fn with_commands<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
            builder.manage(AppState::default()).invoke_handler(tauri::generate_handler![$($name),*])
        }
    };
    (@handler sync $($function:tt)*) => {
        #[tauri::command]
        $($function)*
    };
    (@handler async $(#[$meta:meta])* fn $name:ident($($args:tt)*) -> $result:ty $body:block) => {
        $(#[$meta])*
        #[tauri::command]
        async fn $name($($args)*) -> $result $body
    };
}

include!("commands.rs");

pub fn run() {
    with_commands(tauri::Builder::default())
        .run(tauri::generate_context!())
        .expect("error while running CADE");
}

#[cfg(test)]
mod tests;
