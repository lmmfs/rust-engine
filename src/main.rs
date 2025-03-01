use anyhow::{Ok, Result};
use tao::{event::{ElementState, Event, MouseButton, WindowEvent}, window};
use std::time::Duration;
use engine::surfaces::RenderSurface;
use engine::color::Color;

mod engine;

struct State {
    updates_called: usize,
    renders_called: usize,
    time_passed: Duration,
    box_position: (isize, isize),
    box_direction: (isize, isize),
    box_size: (usize, usize),
    button_pressed: bool,
    cursor_position: (u32, u32),
    swap_color: bool,
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
            button_pressed: false,
            cursor_position: (0, 0),
            swap_color: false,
        }
    }
}


fn main() -> Result<()> {
    let width = 640;
    let height = 480;
    let state = State::default();

    let context = engine::init_tao_window("Rust Engine", width, height)?;
    let surface = engine::init_pixels(&context, width, height)?;

    engine::run_with_tao_and_pixels(
        state, 
        context, 
        surface, 
        |s, surface| {
            s.box_position.0 = s.box_position.0 + s.box_direction.0;
            s.box_position.1 = s.box_position.1 + s.box_direction.1;
            
            if s.box_position.0 + s.box_size.0 as isize >= surface.width() as isize 
            || s.box_position.0 < 0 {
                s.box_direction.0 = s.box_direction.0 * -1;
                s.box_position.0 = s.box_position.0 + s.box_direction.0;
                s.swap_color = !s.swap_color;
            }

            if s.box_position.1 + s.box_size.1 as isize >= surface.height() as isize 
            || s.box_position.1 < 0 {
                s.box_direction.1 = s.box_direction.1 * -1;
                s.box_position.1 = s.box_position.1 + s.box_direction.1;
                s.swap_color = !s.swap_color;
            }

            s.updates_called += 1;
            
            Ok(())
        },
        |s, surface, dt| {
            let width = surface.width();
            let height = surface.height();

            surface.clear_screen(&Color::from_rgb(0, 0, 0));
            if s.swap_color {
                surface.filled_rect(
                    s.box_position.0 as u32, 
                    s.box_position.1 as u32, 
                    s.box_size.0 as u32, 
                    s.box_size.1 as u32, 
                    &Color::from_rgb(255, 0, 0)
                );
            } else {
                surface.filled_rect(
                    s.box_position.0 as u32, 
                    s.box_position.1 as u32, 
                    s.box_size.0 as u32, 
                    s.box_size.1 as u32, 
                    &Color::from_rgb(255, 255, 0)
                );
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

            surface.blit()?;

            Ok(())
        },
        |s, surface, _, event| {
            match event {
                Event::WindowEvent {
                    event: win_event, ..
                } => match win_event {
                    WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state,
                        ..
                    } => {
                        if state == &ElementState::Pressed {
                            s.button_pressed = true;
                        } else {
                            s.button_pressed = false;
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let pixel_position= surface
                            .physical_pos_to_surface_pos(position.x, position.y)
                            .unwrap_or((0, 0));
                        s.cursor_position = pixel_position;
                    }
                    _ => {}
                },
                _ => {}
            }
            Ok(())
        });
}


