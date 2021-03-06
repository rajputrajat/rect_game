use log::{debug, info};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

fn config() -> Conf {
    Conf {
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main("config")]
async fn main() {
    env_logger::init();
    let ferris = load_texture("./lag.png").await.unwrap();
    let mut cam = Camera3D {
        position: vec3((screen_width() / 2.) + 50., screen_height() / 2., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };
    let mut timers = Timers::new();
    let fps = Arc::new(Mutex::new(String::from("calculating fps...")));
    let for_closure = Arc::clone(&fps);
    timers.add(
        2.,
        move || {
            info!("updating fps");
            let mut string = for_closure.lock().unwrap();
            string.replace_range(.., &format!("fps: {}", get_fps()));
        },
        true,
    );
    loop {
        clear_background(GRAY);
        cam = process_input(cam);
        set_camera(&cam);
        draw_sphere(
            vec3(screen_width() / 2., screen_height() / 2., 0.),
            10.,
            ferris,
            GOLD,
        );
        timers.update();
        set_default_camera();
        draw_text(&fps.lock().unwrap(), 10., 10., 10., WHITE);
        next_frame().await
    }
}

fn process_input(mut cam: Camera3D) -> Camera3D {
    if is_key_down(KeyCode::Left) {
        cam.position.z += 0.1;
    } else if is_key_down(KeyCode::Right) {
        cam.position.z -= 0.1;
    } else if is_key_down(KeyCode::Up) {
        cam.position.y += 0.1;
    } else if is_key_down(KeyCode::Down) {
        cam.position.y -= 0.1;
    } else if is_key_down(KeyCode::PageUp) {
        cam.position.x -= 0.1;
    } else if is_key_down(KeyCode::PageDown) {
        cam.position.x += 0.1;
    } else if is_key_down(KeyCode::Escape) {
        std::process::exit(0);
    }
    cam
}

#[derive(Clone)]
struct Timer {
    total_timeout: f32,
    remaining: f32,
    callback: Arc<Mutex<dyn FnMut()>>,
    periodic: bool,
}

impl std::fmt::Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Timer {{ {}, {}, {}}}",
            self.total_timeout, self.remaining, self.periodic
        ))
    }
}

struct Timers(Vec<Timer>);

impl Timers {
    fn new() -> Timers {
        Timers(vec![])
    }

    fn update(&mut self) {
        let elapsed_time = get_frame_time();
        let mut timer_done: Vec<usize> = Vec::new();
        for (i, t) in self.0.iter_mut().enumerate() {
            t.remaining -= elapsed_time;
            if t.remaining <= 0. {
                timer_done.push(i);
            }
        }
        for i in &timer_done {
            let mut item = self.0[*i].clone();
            let cb = Arc::clone(&item.callback);
            (cb.lock().unwrap())();
            if item.periodic {
                item.remaining = item.total_timeout;
                debug!("{:?}", item);
                self.0.push(item);
            }
            self.0.remove(*i);
        }
    }

    fn add<F: FnMut() + 'static>(&mut self, time_out: f32, callback: F, periodic: bool) {
        self.0.push(Timer {
            total_timeout: time_out,
            remaining: time_out,
            callback: Arc::new(Mutex::new(callback)),
            periodic,
        });
    }
}
