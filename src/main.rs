use cursive::view::Nameable;
use cursive::views::{Dialog, TextView};
use cursive::{Cursive, CursiveExt};

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let mut siv = Cursive::default();
    let is_running = Arc::new(Mutex::new(false));

    let root_timer = Arc::new(Mutex::new(25 * 60));

    let time_view = TextView::new(format!(
        "Time left: {}",
        format_time(*root_timer.lock().unwrap())
    ))
    .with_name("textviewtime");

    let cb_sink = siv.cb_sink().clone();

    siv.add_layer(
        Dialog::new()
            .title("Pomodoro timer at your terminal!")
            .content(time_view)
            .button("Start/Resume", {
                let timer_clone = Arc::clone(&root_timer);
                let runningclone = Arc::clone(&is_running);
                move |s| {
                    let timer = Arc::clone(&timer_clone);
                    let time = timer.lock().unwrap();
                    let mut running = runningclone.lock().unwrap();
                    *running = true;
                    s.call_on_name("textviewtime", |v: &mut TextView| {
                        v.set_content(format!("Time left: {}", format_time(*time)))
                    });
                }
            })
            .button("Pause", {
                let runningclone = Arc::clone(&is_running);
                let timer_clone = Arc::clone(&root_timer);
                move |s| {
                    let mut running = runningclone.lock().unwrap();
                    let time = *timer_clone.lock().unwrap();
                    *running = false;
                    s.call_on_name("textviewtime", |v: &mut TextView| {
                        v.set_content(format!("Time is paused, time left: {}", format_time(time)))
                    });
                }
            })
            .button("Settings", {
                let timer_clone = Arc::clone(&root_timer);
                move |s| {
                    s.add_layer(
                        Dialog::new()
                            .content(TextView::new("Choose a preset time:"))
                            .button("5 mins", {
                                let timer = Arc::clone(&timer_clone);
                                move |s| {
                                    let mut t = timer.lock().unwrap();
                                    *t = 5 * 60;
                                    s.pop_layer();
                                    s.call_on_name("textviewtime", |v: &mut TextView| {
                                        v.set_content(format!("Time left: {}", format_time(*t)))
                                    });
                                }
                            })
                            .button("10 mins", {
                                let timer = Arc::clone(&timer_clone);
                                move |s| {
                                    let mut t = timer.lock().unwrap();
                                    *t = 10 * 60;
                                    s.pop_layer();
                                    s.call_on_name("textviewtime", |v: &mut TextView| {
                                        v.set_content(format!("Time left: {}", format_time(*t)))
                                    });
                                }
                            })
                            .button("15 mins", {
                                let timer = Arc::clone(&timer_clone);
                                move |s| {
                                    let mut t = timer.lock().unwrap();
                                    *t = 5;
                                    s.pop_layer();
                                    s.call_on_name("textviewtime", |v: &mut TextView| {
                                        v.set_content(format!("Time left: {}", format_time(*t)))
                                    });
                                }
                            })
                            .button("Go back", {
                                move |s| {
                                    s.pop_layer();
                                }
                            }),
                    );
                }
            })
            .button("Quit", |s| s.quit()),
    );

    let is_running_clone = Arc::clone(&is_running);
    let root_timer_clone = Arc::clone(&root_timer);

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));

        let mut running = is_running_clone.lock().unwrap();
        let inner = Arc::clone(&root_timer_clone);
        let mut timer = root_timer_clone.lock().unwrap();
        if *running {
            *timer -= 1;
            if *timer != 0 {
                cb_sink
                    .send(Box::new(move |s: &mut Cursive| {
                        let time_left = *inner.lock().unwrap();
                        s.call_on_name("textviewtime", |v: &mut TextView| {
                            v.set_content(format!("Time left: {}", format_time(time_left)))
                        });
                    }))
                    .unwrap();
            } else {
                *timer = 25 * 60;
                cb_sink
                    .send(Box::new(move |s: &mut Cursive| {
                        s.add_layer(Dialog::info("Time is up!"));
                        s.call_on_name("textviewtime", |v: &mut TextView| {
                            v.set_content(format!("Time left: {}", format_time(25 * 60)))
                        });
                    }))
                    .unwrap();
                *running = false;
            }
        }
    });

    siv.run();
}

fn format_time(time: u32) -> String {
    let minutes = time / 60;
    let seconds = time % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
