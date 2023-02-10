use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std:: time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = match video_subsystem
        .window("SDL", 640, 480)
        .position_centered()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("failed to build window: {:?}", err),
    };

    let mut canvas = match window.into_canvas().build()
    {
        Ok(canvas) => canvas,
        Err(err) => panic!("failed to build canvas: {:?}", err),
    };

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = match sdl_context.event_pump()
    {
        Ok(event_pump) => event_pump,
        Err(err) => panic!("failed to build event_pump: {:?}", err),
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32) / 60);
    }

}
