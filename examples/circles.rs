use std::time::Instant;

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

    let mut render_commands = dust_bunny::RenderCommands::new();
    let mut last_time = Instant::now();
    let mut angle = 0.0;

    'running: loop {
        let now = Instant::now();
        let dt = (now - last_time).as_secs_f32();
        last_time = now;

        angle += dt * 2.0;
        angle %= std::f32::consts::TAU;

        let orbit_radius = 250.0;
        let count = 5;

        let mut positions = vec![];

        for i in 0..count {
            let offset = (i as f32 / count as f32) * std::f32::consts::TAU;
            positions.push(Vec2::new(
                (angle + offset).cos() * orbit_radius,
                (angle + offset).sin() * orbit_radius,
            ));
        }

        for pos in positions {
            render_commands.draw_circle(pos, 50.0);
        }

        renderer.render(&render_commands).unwrap();

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
    }
}
