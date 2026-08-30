use core::time::Duration;
use emacs::{Env, Result, Value, defun};
use gtk::prelude::*;
use gtk::{gdk, gio, glib};
use std::sync::OnceLock;

//use emacs::use_symbols;
emacs::plugin_is_GPL_compatible!();
use std::cell::RefCell;
use std::rc::Rc;

static GTK_INITIALIZED: OnceLock<bool> = OnceLock::new();

fn emit_paste_signal(text: &str) {
    if let Ok(conn) = gio::bus_get_sync(gio::BusType::Session, None::<&gio::Cancellable>) {
        let args = glib::Variant::tuple_from_iter([text.to_variant()]);
        let _ = conn.emit_signal(
            None,
            "/io/github/aganzha/LiveRing",
            "io.github.aganzha.LiveRing",
            "PasteChanged",
            Some(&args),
        );
    }
}

#[emacs::module(name = "live-ring")]
fn init<'a>(env: &'a Env) -> Result<Value<'a>> {
    let initialized = GTK_INITIALIZED.get_or_init(|| gtk::init().is_ok());
    if !*initialized {
        let _ = env.message("live-ring: failed to initialise gtk");
        return env.intern("t");
    }
    let latest_paste = Rc::new(RefCell::new("".to_string()));
    let _ = env.message("🐦 live-ring: gtk initialized");
    if let Some(display) = gdk::Display::default() {
        if let Some(clipboard) = gtk::Clipboard::default(&display) {
            glib::timeout_add_local(Duration::from_millis(1000), {
                let latest_paste = latest_paste.clone();
                move || {
                    if let Some(text) = clipboard.wait_for_text() {
                        let mut previous_paste = latest_paste.borrow_mut();
                        if *previous_paste != text {
                            eprintln!("🌻 ...............{}", text);
                            *previous_paste = text.to_string();
                            emit_paste_signal(&previous_paste.clone())
                        }
                    }
                    glib::ControlFlow::Continue
                }
            });
        }
    }
    env.intern("t")
}
