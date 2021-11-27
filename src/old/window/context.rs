pub trait GraphicsContext {
	fn init(&self);
	fn clear(&self);
}

pub struct OpenGlContext {
	pub context: sdl2::video::GLContext,
	pub gl: ()
}

impl GraphicsContext for OpenGlContext {
    fn init(&self) {
        todo!()
    }

    fn clear(&self) {
        todo!()
    }
}