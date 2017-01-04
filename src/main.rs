extern crate gtk;
use gtk::prelude::*;
use std::cell::Cell;
use std::rc::Rc;
use std::ops::Deref;
use gtk::{Orientation, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK. Do you have libgtk 3.10+?");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    window.set_title("Iron Tomato");
    window.set_border_width(15);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        // Let the default handler destroy the window.
        Inhibit(false)
    });

    let label = gtk::Label::new_with_mnemonic(Some("25:00"));
    let run_button = gtk::Button::new_with_label("Start Tomato");
    let reset_button = gtk::Button::new_with_label("Reset");
    let main_box = gtk::Box::new(Orientation::Vertical, 30);
    main_box.add(&label);
    main_box.add(&run_button);
    main_box.add(&reset_button);
    window.add(&main_box);

    let total_seconds = Rc::new(Cell::new(1500));
    let is_timer_running = Rc::new(Cell::new(false));

    // connect_clicked() takes a Fn() closure, so interior mutability saves the day :(
    {
        let better_catch_it = is_timer_running.clone();

        run_button.connect_clicked(move |butt| {
            let timer_state = better_catch_it.deref();
            if timer_state.get() == false {
                timer_state.set(true);
                butt.set_label("Pause Tomato");
            } else {
                timer_state.set(false);
                butt.set_label("Resume Tomato");
            }
        });
    }
    {
        let seconds_remaining = total_seconds.clone();

        reset_button.connect_clicked(move |_| {
            seconds_remaining.deref().set(1500);
        });
    }

    gtk::timeout_add(1000, move || {
        let mut seconds_remaining = total_seconds.deref().get();

        // If the timer is running, decrement one second per second
        if is_timer_running.deref().get() {
            if seconds_remaining > 0 {
                seconds_remaining -= 1;
            }
        }

        // Update the total seconds remaining + clock time label
        total_seconds.deref().set(seconds_remaining);
        label.set_label(get_time_string(seconds_remaining).as_str());

        gtk::Continue(true)
    });

    window.show_all();
    gtk::main();
}

fn get_time_string(seconds: u32) -> String {
    let timer_mins = seconds / 60;
    let timer_seconds = seconds % 60;

    format!("{:02}:{:02}", timer_mins, timer_seconds)
}
