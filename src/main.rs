use std::time::{Duration, Instant};
use pixels::Pixels;
use pixels::SurfaceTexture;
use tao::dpi::LogicalSize;
use tao::event::{Event, KeyEvent, WindowEvent};
use tao::event_loop;
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::window;
use tao::window::Window;
use tao::window::WindowBuilder;

use std::sync::Arc;

fn pixel_loop<S >(
    mut state:S, 
    update_fps: usize, 
    update: fn(&mut S), 
    render:fn(&mut S, Duration)
) {
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

fn pixel_loop_tao<S: 'static>(
    mut state:S, 
    (width, height): (u32, u32),
    update_fps: usize, 
    update: fn(&mut S, u32, u32),
    render:fn(&mut S, Duration, u32, u32, &mut Pixels)
    ) {

    if update_fps == 0 {
        panic!("fps cannot be zero");
    }

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(width, height);
        let window = WindowBuilder::new()
            .with_title("Hello Pixels/Tao")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap();
        Arc::new(window)
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
        Pixels::new(width, height, surface_texture).unwrap()
    };

    let mut accum:Duration = Duration::new(0, 0);
    let mut current_time = Instant::now();
    let mut last_time= Instant::now();

    let update_dt = Duration::from_nanos((1_000_000_000f64 / update_fps as f64).round() as u64);
    
    event_loop.run(move |event, _, control_flow| {

        match event {
            // Update internal state and request a redraw
            Event::MainEventsCleared => {
                last_time = current_time;
                current_time = Instant::now();
                let mut dt = current_time - last_time;
        
                //escape hatch if the update calls takes to long
                if dt > Duration::from_millis(100) {
                    dt = Duration::from_millis(100);
                }

                while accum > update_dt {
                    update(&mut state, width, height);
                    accum -= update_dt;
                }
                render(&mut state, dt, width, height, &mut pixels);
                accum += dt; 
                if let Err(err) = pixels.render() {
                    //log_error("pixels.render", err);
                    //*control_flow = ControlFlow::Exit;
                    panic!("error in pixels draw");
                }
            }

            _ => {}
        }
    });
}


struct State {
    updates_called: usize,
    renders_called: usize,
    time_passed: Duration,
    box_position: (isize, isize),
    box_direction: (isize, isize),
    box_size: (usize, usize),
}

impl Default for State {
    fn default() -> Self {
        Self { 
            updates_called: Default::default(), 
            renders_called: Default::default(), 
            time_passed: Default::default(), 
            box_position: Default::default(), 
            box_direction: (1, 1),
            box_size: (50, 60),
        }
    }
}


fn main() {
    let width = 640;
    let height = 480;
    let state = State::default();

    pixel_loop_tao(
        state,
        (width, height),
        120,
        |s, width, height| {
            s.box_position.0 = s.box_position.0 + s.box_direction.0;
            s.box_position.1 = s.box_position.1 + s.box_direction.1;
            
            if s.box_position.0 + s.box_size.0 as isize >= width as isize || s.box_position.0 < 0 {
                s.box_direction.0 = s.box_direction.0 * -1;
                s.box_position.0 = s.box_position.0 + s.box_direction.0;
            }

            if s.box_position.1 + s.box_size.1 as isize >= height as isize || s.box_position.1 < 0 {
                s.box_direction.1 = s.box_direction.1 * -1;
                s.box_position.1 = s.box_position.1 + s.box_direction.1;
            }

            s.updates_called += 1;
            //println!("update");
        },
        |s, dt, width, height,pixels| {
            let buf = pixels.frame_mut();

            //clear background
            for y in 0..height  {
                for x in 0..width {
                    let i = ((y * width + x) * 4) as usize;
                    buf[i + 0] = 0;
                    buf[i + 1] = 0;
                    buf[i + 2] = 0;
                    buf[i + 3] = 255;
                }
            }

            for y in s.box_position.1 as usize..s.box_position.1 as usize + s.box_size.1 {
                for x in s.box_position.0 as usize..s.box_position.0 as usize + s.box_size.0 {
                    let i = ((y * width as usize + x) * 4) as usize;
                    buf[i + 0] = 255;
                    buf[i + 1] = 255;
                    buf[i + 2] = 0;
                    buf[i + 3] = 255;
                }
            }
            
            s.renders_called += 1;
            s.time_passed += dt;

            if s.time_passed > Duration::from_secs(1) {
                println!("Update FPS: {:.2}", s.updates_called as f64 / 1f64);
                println!("Render FPS: {:.2}", s.renders_called as f64 / 1f64);
                s.updates_called = 0;
                s.renders_called = 0;
                s.time_passed = Duration::default();
            }
        }
    )
}


