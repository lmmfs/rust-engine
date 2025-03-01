use anyhow::{Context, Result};
use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant};
use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::{Window, WindowBuilder};

type UpdateFn<State, Surface> = fn(&mut State, &mut Surface) -> Result<()>;
type RenderFn<State, Surface> = fn(&mut State, &mut Surface, Duration) -> Result<()>;

pub struct PixelsSurface {
    pixels: Pixels,
}

impl PixelsSurface {
    pub fn new(pixels: Pixels) -> Self {
        Self { pixels }
    }

    pub fn width(&self) -> u32 {
        self.pixels.texture().width()
    }
    pub fn height(&self) -> u32 {
        self.pixels.texture().height()
    }
    pub fn frame_mut(&mut self) -> &mut [u8] {
        self.pixels.frame_mut()
    }

    pub fn render(&mut self) -> Result<()> {
        self.pixels
            .render()
            .context("letting pixels lib blit to screen")?;
        Ok(())
    }
}


pub struct TaoContext {
    event_loop: EventLoop<()>,
    window: Window,
}

impl TaoContext {
    pub fn get_wiwdow(&self) -> &Window {
        return &self.window;
    }
}

pub fn init_tao_window(window_title: &str, width: u32, height: u32) -> Result<TaoContext> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(width, height);
        WindowBuilder::new()
            .with_title(window_title)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)?
        };
    
    Ok(TaoContext { event_loop, window })
}


pub fn init_pixels(context: &TaoContext, width: u32, height: u32) -> Result<PixelsSurface> {
    let physical_dimensions = context.get_wiwdow().inner_size();
    let surface_texture = SurfaceTexture::new(
        physical_dimensions.width,
        physical_dimensions.height,
        context.get_wiwdow(),
    );
    let pixels = Pixels::new(width, height, surface_texture).context("create pixels surface")?;
    Ok(PixelsSurface::new(pixels))
}

struct Engine<State, Surface> {
    accumulator: Duration,
    current_time: Instant,
    last_time: Instant,
    update_timestep: Duration,
    state: State,
    surface: Surface,
    update: UpdateFn<State, Surface>,
    render: RenderFn<State, Surface>,
}

impl<State, Surface> Engine<State, Surface> {
    pub fn create(
        update_fps: usize,
        state: State,
        surface: Surface,
        update: UpdateFn<State, Surface>,
        render: RenderFn<State, Surface>,
    ) -> Self {

        if update_fps == 0 {
            panic!("fps cannot set to zero");
        }

        Self {
            accumulator: Duration::new(0, 0),
            current_time: Instant::now(),
            last_time: Instant::now(),
            update_timestep: Duration::from_nanos((1_000_000_000f64 / update_fps as f64).round() as u64),
            state: state,
            surface: surface,
            update: update,
            render: render,
        }

    }

    pub fn next_loop(&mut self) -> Result<()>{
        self.last_time = self.current_time;
        self.current_time = Instant::now();
        let mut dt = self.current_time - self.last_time;

        //escape hatch if the update calls takes to long
        if dt > Duration::from_millis(100) {
            dt = Duration::from_millis(100);
        }

        while self.accumulator > self.update_timestep {
            (self.update)(&mut self.state, &mut self.surface)?;
            self.accumulator -= self.update_timestep;
        }

        (self.render)(&mut self.state, &mut self.surface, dt)?;

        self.accumulator += dt;
        Ok(())
    }

}

pub fn simple_run<State, Surface>(
    state: State,
    surface: Surface,
    update: UpdateFn<State, Surface>,
    render: RenderFn<State, Surface>,
) -> Result<()> {
    let mut engine = Engine::create(120, state, surface, update, render);

    loop {
        engine.next_loop().context("run next engine loop")?;
    }
}

pub fn run_with_tao_and_pixels<State: 'static>(
    state: State,
    context: TaoContext,
    surface: PixelsSurface,
    update: UpdateFn<State, PixelsSurface>,
    render: RenderFn<State, PixelsSurface>,
) -> ! {
    let mut engine = Engine::create(120, state, surface, update, render);

    context
        .event_loop
        .run(move |event, _, control_flow| match event {
            Event::MainEventsCleared => {
                engine
                    .next_loop()
                    .context("run next pixel loop")
                    .unwrap();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },

            _ => {}
        });
}
