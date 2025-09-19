use std::time::{Duration, Instant};

use glam::Vec2;
use sdl3::event::{Event, WindowEvent};

fn main() {
    env_logger::init();

    let sdl_context = sdl3::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let window = sdl_context
        .video()
        .unwrap()
        .window("circle rendering example", 500, 500)
        .resizable()
        .build()
        .unwrap();

    let mut renderer = dust_bunny::Renderer::new(window);

    let target_frame_time = Duration::from_secs_f32(1.0 / 100.0);

    let mut render_commands = dust_bunny::RenderCommands::new();
    let mut angle = 0.0;
    let mut delta_secs: f32 = 0.0;

    'running: loop {
        let start_instant = Instant::now();

        println!("{}", 1.0 / delta_secs);

        angle += delta_secs * 2.0;
        angle %= std::f32::consts::TAU;

        let orbit_radius = 10.0;
        let count = 16;

        let mut circle_data = Vec::with_capacity(5);

        for i in 0..count {
            let offset = (i as f32 / count as f32) * std::f32::consts::TAU;
            let pos = Vec2::new(
                (angle + offset).cos() * orbit_radius,
                (angle + offset).sin() * orbit_radius,
            );

            let hue = 360.0 / count as f64;

            let colour = hsv::hsv_to_rgb(hue * i as f64, 1.0, 1.0);

            circle_data.push((pos, (colour.0, colour.1, colour.2, 255)));
        }

        for circle in circle_data {
            render_commands.draw_circle(circle.0, 1.5, circle.1);
        }

        renderer.render(&render_commands).unwrap();

        render_commands.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window {
                    win_event: WindowEvent::Resized(_, _),
                    ..
                } => renderer.resize(),
                _ => {}
            }
        }

        while (Instant::now() - start_instant) < target_frame_time {
            // I found that 200 is enough that the cpu is not being hammered,
            // and that the framerate limiter is still fairly accurate
            std::thread::sleep(Duration::from_micros(200));
        }
        delta_secs = (Instant::now() - start_instant).as_secs_f32();
    }
}
