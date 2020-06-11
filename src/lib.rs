#![allow(dead_code)]

// modules
pub mod color;
pub mod prelude;

// namespacing
use color::Color;
pub use minifb::{Key, MouseButton};
use minifb::{Window, WindowOptions};

// error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    MiniFb(#[from] minifb::Error),
}

// result type
pub type Result<T> = std::result::Result<T, Error>;

// pixel buffer for internal use
struct PixelBuffer {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl PixelBuffer {
    // create a new pixel buffer
    fn new(width: usize, height: usize) -> PixelBuffer {
        PixelBuffer { width, height, data: vec![0; height * width] }
    }

    // index pixel buffer !! will panic if given x/y are out of bounds
    fn at(&self, x: usize, y: usize) -> &u32 {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.data[y * self.width + x]
    }

    // index pixel buffer mut !! will panic if given x/y are out of bounds
    fn at_mut(&mut self, x: usize, y: usize) -> &mut u32 {
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.data[y * self.width + x]
    }

    // returns the buffer as a slice
    fn as_ref(&self) -> &[u32] {
        &self.data
    }

    // returns a mutable reference to a slice
    fn as_mut_ref(&mut self) -> &mut [u32] {
        &mut self.data
    }
}

/// context data
pub struct Context {
    pixel_buffer: PixelBuffer,
    height: usize,
    width: usize,
    window: Window,
}

impl Context {
    /// create a new context
    pub fn new(width: usize, height: usize, title: String) -> Result<Context> {
        let pixel_buffer = PixelBuffer::new(height, width);
        let mut window = Window::new(&title, width, height, WindowOptions::default())?;
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        Ok(Context { pixel_buffer, window, height, width })
    }

    /// render the internal buffer to the screen
    pub fn present(&mut self) -> Result<()> {
        self.window.update_with_buffer(&self.pixel_buffer.as_ref(), self.width, self.height)?;
        Ok(())
    }

    /// clears the pixel buffer
    pub fn clear(&mut self, color: Option<color::Color>) {
        let color = color.unwrap_or(color::BLACK);
        self.pixel_buffer.as_mut_ref().iter_mut().for_each(|pixel| *pixel = color.as_u32());
    }

    /// checks if window is open
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    /// set a pixel
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        *self.pixel_buffer.at_mut(x, y) = color.as_u32();
    }

    /// get mouse position
    pub fn get_mouse_pos(&self) -> Option<(f32, f32)> {
        self.window.get_mouse_pos(minifb::MouseMode::Discard)
    }

    /// get mouse down
    pub fn get_mouse_down(&self, button: MouseButton) -> bool {
        self.window.get_mouse_down(button)
    }
}
