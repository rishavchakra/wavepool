extern crate sdl2;

/// SDL Windows contain information about the generated window.
/// context: SDL window context
/// video_subsystem: SDL window video subsystem
/// window: SDL window created and shown
/// event_pump: SDL Event handling
/// graphics_context: Trait object for some type of graphics context
pub struct SdlWindow {
    context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    graphics_context: Box<dyn GraphicsContext>,
}

impl SdlWindow {
    /// Initializes an SDL window and its associated data and event handling
    pub fn init() -> SdlWindow {
        let init_sdl_context = sdl2::init().expect("ERROR: SDL could not be initialized");
        let init_video = init_sdl_context
            .video()
            .expect("ERROR: SDL video subsystem could not be created");

        let init_gl_attr = init_video.gl_attr();
        init_gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        // Version 4.1 is the highest allowed for mac Metal emulation, figure this out later
        init_gl_attr.set_context_version(4, 1);

        let init_window = init_video
            .window("Wavepool", 900, 600)
            .opengl() // Change as necessary
            .resizable()
            .position_centered()
            .build()
            .expect("ERROR: SDL could not create window");

        let init_gl_context = init_window
            .gl_create_context()
            .expect("ERROR: OpenGL context could not be created");

        let init_gl =
            gl::load_with(|s| init_video.gl_get_proc_address(s) as *const std::os::raw::c_void);

		// This stuff should not be in the init
        // unsafe {
        //     gl::Viewport(0, 0, 900, 600);
        //     gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        // }

        let init_event_pump = init_sdl_context
            .event_pump()
            .expect("ERROR: SDL could not create event pump");

        let gl_context = Box::new(OpenGlContext {
			context:init_gl_context,
			gl: init_gl
		});

        SdlWindow {
            context: init_sdl_context,
            video_subsystem: init_video,
            window: init_window,
            event_pump: init_event_pump,
            graphics_context: gl_context,
        }
    }

	pub fn events(&mut self) -> sdl2::event::EventPollIterator {
		self.event_pump.poll_iter()
	}
}


// Consider moving to a different file/module
trait GraphicsContext {
	fn init(&self);
}

struct OpenGlContext {
	context: sdl2::video::GLContext,
	gl: ()
}

impl GraphicsContext for OpenGlContext {
    fn init(&self) {
        todo!();
    }
}