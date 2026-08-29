use core::time::Duration;
use emacs::{Env, Result, Value, defun};
use gtk::{gdk, glib};
use std::sync::OnceLock;

//use emacs::use_symbols;
emacs::plugin_is_GPL_compatible!();

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

static GTK_INITIALIZED: OnceLock<bool> = OnceLock::new();

#[emacs::module(name = "live-ring")]
fn init<'a>(env: &'a Env) -> Result<Value<'a>> {
    let initialized = GTK_INITIALIZED.get_or_init(|| gtk::init().is_ok());

    if !*initialized {
        let _ = env.message("live-ring: failed to initialise gtk");
        return env.intern("t");
    }
    let _ = env.message("🐦 live-ring: gtk initialized");
    if let Some(display) = gdk::Display::default() {
        eprintln!("💦 ...............{}", display);
        if let Some(clipboard) = gtk::Clipboard::default(&display) {
            println!("🧶 ...............{}", clipboard);
            glib::timeout_add_local(Duration::from_millis(1000), {
                move || {
                    eprintln!("🦴 loop");
                    if let Some(text) = clipboard.wait_for_text() {
                        eprintln!("🌻 ...............{}", text);
                    }
                    glib::ControlFlow::Continue
                }
            });
        }
    }
    env.intern("t")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
