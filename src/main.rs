mod sdl;

fn main() {
    env_logger::init();
    let mut sdl = sdl::SdlWindowData::init();

    'main: loop {
        // Event Handling

        for event in sdl.events() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // window.gl_swap_window();
    }
}