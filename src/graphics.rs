// Module for different graphics and gpu-interfacing libraries

// OpenGL graphics library
pub mod gl;

// Metal graphics and compute API for mac
pub mod metal;

// Vulkan graphics and compute API
pub mod vulkan;

pub trait GraphicsLibrary {
	fn init();
}