use std::time::{Duration, Instant};

fn pixel_loop<S>(mut state:S, update_fps: usize, update: fn(&mut S), render:fn(&mut S, dt: Duration)) {
    if update_fps == 0 {
        panic!("fps cannot be zero");
    }

    let mut accum:Duration = Duration::new(0, 0);
    let mut current_time = Instant::now();
    let mut last_time;

    let update_dt = Duration::from_nanos((1_000_000_000f64 / update_fps as f64).round() as u64);

    loop {
        last_time = current_time;
        current_time = Instant::now();
        let mut dt = current_time - last_time;

        //escape hatch if the update calls takes to long
        if dt > Duration::from_millis(100) {
            dt = Duration::from_millis(100);
        }

        while accum > update_dt {
            update(&mut state);
            accum -= update_dt;
        }
        
        render(&mut state, dt);
        accum += dt; 
    }
}

#[derive(Default)]
struct State {
    updates_called: usize,
    renders_called: usize,
    time_passed: Duration,

}


fn main() {
    let state = State::default();

    pixel_loop(
        state,
        120,
        |s| {
            s.updates_called += 1;
            std::thread::sleep(Duration::from_millis(4));
            //println!("update");
        },
        |s, dt| {
            s.renders_called += 1;
            s.time_passed += dt;

            if s.time_passed > Duration::from_secs(1) {
                println!("Update FPS: {:.2}", s.updates_called as f64 / 1f64);
                println!("Render FPS: {:.2}", s.renders_called as f64 / 1f64);
                s.updates_called = 0;
                s.renders_called = 0;
                s.time_passed = Duration::default();
            }

            std::thread::sleep(Duration::from_millis(4));
            //println!("render");
        }
    )
}
