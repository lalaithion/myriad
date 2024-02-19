#![feature(once_cell)]
#![feature(sort_floats)]

include!(concat!(env!("OUT_DIR"), "/myriad.rs"));

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use thousands::Separable;
use winit::dpi::PhysicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use rand::{Rng, SeedableRng};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1440;
const SCALE: f32 = 4.0;
const SHAPE: f32 = 8.0;

const TYPES: i8 = 6;
const PARTICLES: usize = 20_000;

fn main() -> Result<(), pixels::Error> {
    println!("Number of Particles: {}", PARTICLES);
    println!("Number of Pixels: {}", WIDTH * HEIGHT);
    println!("=====================================");

    let mut reds = [0 as u8; TYPES as usize];
    let mut greens = [0 as u8; TYPES as usize];
    let mut blues = [0 as u8; TYPES as usize];

    // let mut rng = rand::rngs::StdRng::from_seed([1; 32]);
    let mut rng = rand::thread_rng();
    for i in 0..TYPES {
        reds[i as usize] = rng.gen();
        greens[i as usize] = rng.gen();
        blues[i as usize] = rng.gen();
    }

    let mut raw_forces = [0.0 as f32; TYPES as usize * TYPES as usize];
    for i in 0..(TYPES as usize) {
        for j in 0..(TYPES as usize) {
            raw_forces[i * (TYPES as usize) + j] = rng.gen_range(-1.0..1.0);
        }
    }

    let ctx: &'static Context = Box::leak(Box::new(Context::new().expect("ERROR: Context::new()")));

    let mut raw_types = [0 as i8; PARTICLES];
    let mut raw_x = [0.0 as f32; PARTICLES];
    let mut raw_y = [0.0 as f32; PARTICLES];
    let mut raw_vx = [0.0 as f32; PARTICLES];
    let mut raw_vy = [0.0 as f32; PARTICLES];

    for i in 0..PARTICLES {
        raw_types[i] = rng.gen_range(0..TYPES);
        raw_x[i] = (rng.gen_range(-SHAPE..SHAPE)
            + rng.gen_range(-SHAPE..SHAPE)
            + rng.gen_range(-SHAPE..SHAPE))
            / 3.0;
        raw_y[i] = (rng.gen_range(-SHAPE..SHAPE)
            + rng.gen_range(-SHAPE..SHAPE)
            + rng.gen_range(-SHAPE..SHAPE))
            / 3.0;
        raw_vx[i] =
            (rng.gen_range(-1.0..1.0) + rng.gen_range(-1.0..1.0) + rng.gen_range(-1.0..1.0)) / 30.0;
        raw_vy[i] =
            (rng.gen_range(-1.0..1.0) + rng.gen_range(-1.0..1.0) + rng.gen_range(-1.0..1.0)) / 30.0;
    }

    raw_y.sort_floats();

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
        let size = PhysicalSize::new(WIDTH as f64, HEIGHT as f64);
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

    let mut sim_durations = Vec::with_capacity(100);
    let mut transfer_durations = Vec::with_capacity(100);
    let mut render_durations = Vec::with_capacity(100);
    let mut timing_start = std::time::Instant::now();

    let mut total_frame_times = Vec::with_capacity(100);
    let mut last_frame = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let this_frame = std::time::Instant::now();
            total_frame_times.push(this_frame - last_frame);
            last_frame = this_frame;

            let start = std::time::Instant::now();

            px.values(&mut res_x).expect("ERROR: px.get()");
            py.values(&mut res_y).expect("ERROR: py.get()");
            types.values(&mut res_t).expect("ERROR: types.get()");

            let end = std::time::Instant::now();
            transfer_durations.push(end - start);

            let start = std::time::Instant::now();

            (types, vx, vy, px, py) = ctx
                .step(&forces, &types, &vx, &vy, &px, &py, 0.005)
                .expect("ERROR: ctx.step()");

            let end = std::time::Instant::now();
            sim_durations.push(end - start);
            let start = std::time::Instant::now();

            let frame = pixels.frame_mut();
            frame.fill(0x00);

            for ((px, py), t) in res_x.iter().zip(res_y.iter()).zip(res_t.iter()) {
                let x = ((px + SCALE) / (2.0 * SCALE) * HEIGHT as f32) as i64;
                let y = ((py + SCALE) / (2.0 * SCALE) * HEIGHT as f32) as i64;

                for i in -2..=2 {
                    let dj = if i == -2 || i == 2 {
                        1
                    } else {
                        2
                    };
                    for j in -dj..=dj {
                        let x = x + i;
                        let y = y + j;
                        if x >= WIDTH as i64 || y >= HEIGHT as i64 || x < 0 || y < 0 {
                            continue;
                        }
                        if 4 * (x + y * WIDTH as i64) as usize >= frame.len() {
                            dbg!(x, y, px, py, frame.len());
                            continue;
                        }
                        let idx = 4 * (x + y * WIDTH as i64) as usize;
                        frame[idx] = reds[*t as usize];
                        frame[idx + 1] = greens[*t as usize];
                        frame[idx + 2] = blues[*t as usize];
                        frame[idx + 3] = 0xFF;
                    }
                }
            }
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
            let end = std::time::Instant::now();
            render_durations.push(end - start);

            if std::time::Instant::now() - timing_start > std::time::Duration::from_secs(30) {
                println!(
                    "Simulation: {} microseconds,\tTransferring: {} microseconds,\tRendering: {} microseconds\tTotal: {}",
                    (sim_durations
                        .iter()
                        .sum::<std::time::Duration>()
                        .as_micros() as u64
                        / sim_durations.len() as u64).separate_with_commas(),
                    (transfer_durations
                        .iter()
                        .sum::<std::time::Duration>()
                        .as_micros() as u64
                        / transfer_durations.len() as u64).separate_with_commas(),
                    (render_durations
                        .iter()
                        .sum::<std::time::Duration>()
                        .as_micros() as u64
                        / render_durations.len() as u64).separate_with_commas(),
                    (total_frame_times
                        .iter()
                        .sum::<std::time::Duration>()
                        .as_micros() as u64
                        / total_frame_times.len() as u64).separate_with_commas(),
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
