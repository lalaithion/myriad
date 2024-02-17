#![feature(once_cell)]
#![feature(sort_floats)]

include!(concat!(env!("OUT_DIR"), "/myriad.rs"));

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use rand::Rng;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const SCALE: f32 = 5.0;

const TYPES: i8 = 4;
const PARTICLES: usize = 12_000;

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    t: i8,
}

fn main() -> Result<(), pixels::Error> {
    println!("Number of Particles: {}", PARTICLES);
    println!("Number of Pixels: {}", WIDTH * HEIGHT);
    println!("=====================================");

    let mut reds = [0 as u8; TYPES as usize];
    let mut greens = [0 as u8; TYPES as usize];
    let mut blues = [0 as u8; TYPES as usize];

    let mut rng = rand::thread_rng();
    for i in 0..TYPES {
        reds[i as usize] = rng.gen();
        greens[i as usize] = rng.gen();
        blues[i as usize] = rng.gen();
    }

    let ctx: &'static Context = Box::leak(Box::new(Context::new().expect("ERROR: Context::new()")));

    // Initialize random world...
    let mut rng = rand::thread_rng();

    let mut raw_types = [0 as i8; PARTICLES];
    let mut raw_x = [0.0 as f32; PARTICLES];
    let mut raw_y = [0.0 as f32; PARTICLES];
    let mut raw_vx = [0.0 as f32; PARTICLES];
    let mut raw_vy = [0.0 as f32; PARTICLES];

    for i in 0..PARTICLES {
        raw_types[i] = rng.gen_range(0..TYPES);
        raw_x[i] =
            (rng.gen_range(-8.0..8.0) + rng.gen_range(-8.0..8.0) + rng.gen_range(-8.0..8.0)) / 3.0;
        raw_y[i] =
            (rng.gen_range(-8.0..8.0) + rng.gen_range(-8.0..8.0) + rng.gen_range(-8.0..8.0)) / 3.0;
        raw_vx[i] =
            (rng.gen_range(-1.0..1.0) + rng.gen_range(-1.0..1.0) + rng.gen_range(-1.0..1.0)) / 30.0;
        raw_vy[i] =
            (rng.gen_range(-1.0..1.0) + rng.gen_range(-1.0..1.0) + rng.gen_range(-1.0..1.0)) / 30.0;
    }

    raw_y.sort_floats();

    let mut raw_forces = [0.0 as f32; TYPES as usize * TYPES as usize];
    for i in 0..TYPES {
        for j in 0..TYPES {
            raw_forces[(i * TYPES + j) as usize] = rng.gen_range(-1.0..1.0);
        }
    }

    let forces = ArrayF32D2::new(&ctx, [TYPES as i64, TYPES as i64], &raw_forces)
        .expect("ERROR: forces.new()");
    let mut types =
        ArrayI8D1::new(ctx, [PARTICLES as i64], &raw_types).expect("ERROR: types.new()");
    let mut vx = ArrayF32D1::new(ctx, [PARTICLES as i64], &raw_vx).expect("ERROR: vx.new()");
    let mut vy = ArrayF32D1::new(ctx, [PARTICLES as i64], &raw_vy).expect("ERROR: vy.new()");
    let mut px = ArrayF32D1::new(ctx, [PARTICLES as i64], &raw_x).expect("ERROR: px.new()");
    let mut py = ArrayF32D1::new(ctx, [PARTICLES as i64], &raw_y).expect("ERROR: py.new()");

    let mut res_x = [0.0 as f32; PARTICLES];
    let mut res_y = [0.0 as f32; PARTICLES];
    let mut res_t = [0.0 as i8; PARTICLES];

    // The main thread is the event loop
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut world = Vec::with_capacity(PARTICLES);
    let mut sim_durations = Vec::with_capacity(100);
    let mut render_durations = Vec::with_capacity(100);
    let mut timing_start = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let start = std::time::Instant::now();

            (types, vx, vy, px, py) = ctx
                .step(&forces, &types, &vx, &vy, &px, &py, 0.01)
                .expect("ERROR: ctx.step()");

            px.values(&mut res_x).expect("ERROR: px.get()");
            py.values(&mut res_y).expect("ERROR: py.get()");
            types.values(&mut res_t).expect("ERROR: types.get()");
            world.clear();
            for ((x, y), t) in res_x.iter().zip(res_y.iter()).zip(res_t.iter()) {
                world.push(Particle {
                    x: *x,
                    y: *y,
                    t: *t,
                });
            }

            let end = std::time::Instant::now();
            sim_durations.push(end - start);

            let start = std::time::Instant::now();
            draw_pixel(pixels.frame_mut(), &world, &reds, &greens, &blues);
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
            let end = std::time::Instant::now();
            render_durations.push(end - start);

            if std::time::Instant::now() - timing_start > std::time::Duration::from_secs(5) {
                println!(
                    "Simulation: {:?} ms,\tRendering: {:?} microseconds",
                    sim_durations
                        .iter()
                        .sum::<std::time::Duration>()
                        .as_millis()
                        / sim_durations.len() as u128,
                    render_durations
                        .iter()
                        .sum::<std::time::Duration>()
                        .as_micros()
                        / render_durations.len() as u128,
                );
                timing_start = std::time::Instant::now();
                sim_durations.clear();
                render_durations.clear();
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape)
                || input.close_requested()
                || input.key_pressed(VirtualKeyCode::Q)
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn draw_pixel(frame: &mut [u8], particles: &[Particle], reds: &[u8], greens: &[u8], blues: &[u8]) {
    frame.fill(0x00);

    for p in particles {
        let x = ((p.x + SCALE) / (2.0 * SCALE) * HEIGHT as f32) as i64;
        let y = ((p.y + SCALE) / (2.0 * SCALE) * HEIGHT as f32) as i64;

        if x >= WIDTH as i64 || y >= HEIGHT as i64 || x < 0 || y < 0 {
            continue;
        }
        if 4 * (x + y * WIDTH as i64) as usize >= frame.len() {
            dbg!(x, y, p.x, p.y, frame.len());
            continue;
        }
        let idx = 4 * (x + y * WIDTH as i64) as usize;
        frame[idx] = reds[p.t as usize];
        frame[idx + 1] = greens[p.t as usize];
        frame[idx + 2] = blues[p.t as usize];
        frame[idx + 3] = 0xFF;
    }
}
