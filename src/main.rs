mod graphics;
mod window;

fn main() {
    let mut sdl = window::sdl::SdlWindow::init();

    'main: loop {
        // Event Handling

        for event in sdl.events() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // Other stuff
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // window.gl_swap_window();
    }
}

// TODO: Move this somewhere, maybe to a library
fn create_whitespace_cstring_with_len(len: usize) -> std::ffi::CString {
    let mut buffer = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { std::ffi::CString::from_vec_unchecked(buffer) }
}
