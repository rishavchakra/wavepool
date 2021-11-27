extern crate sdl2;

use super::context::{GraphicsContext, OpenGlContext};
/// SDL Windows contain information about the generated window.
/// context: SDL window context
/// video_subsystem: SDL window video subsystem
/// window: SDL window created and shown
/// event_pump: SDL Event handling
/// graphics_context: Trait object for some type of graphics context
pub struct SdlWindowData {
    context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
}

impl SdlWindowData {
    /// Initializes an SDL window and its associated data and event handling
    pub fn init() -> SdlWindowData {
        let init_sdl_context = sdl2::init().expect("ERROR: SDL could not be initialized");
        let init_video = init_sdl_context
            .video()
            .expect("ERROR: SDL video subsystem could not be created");

        let init_window = init_video
            .window("Wavepool", 1000, 1000)
            .position_centered()
            .build()
            .expect("ERROR: SDL could not create window");

        let init_event_pump = init_sdl_context
            .event_pump()
            .expect("ERROR: SDL could not create event pump");

        SdlWindowData {
            context: init_sdl_context,
            video_subsystem: init_video,
            window: init_window,
            event_pump: init_event_pump,
        }
    }

	pub fn events(&mut self) -> sdl2::event::EventPollIterator {
		self.event_pump.poll_iter()
	}
}