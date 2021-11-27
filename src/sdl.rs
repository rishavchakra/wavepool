extern crate sdl2;

pub struct SdlWindowData {
    context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
}

impl SdlWindowData {
    pub fn init() -> SdlWindowData {
        let init_sdl_context = sdl2::init().expect("ERROR: SDL could not be initialized");
        let init_video = init_sdl_context
            .video()
            .expect("ERROR: SDL video subsystem could not be created");
        let init_window = init_video
            .window("Wavepool", 600, 600)
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
