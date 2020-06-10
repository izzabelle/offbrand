// modules
pub mod color;

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
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl PixelBuffer {
    // create a new pixel buffer
    fn new(width: usize, height: usize) -> PixelBuffer {
        PixelBuffer { width, height, buffer: vec![0; width * height] }
    }
}

impl std::convert::AsRef<Vec<u32>> for PixelBuffer {
    fn as_ref(&self) -> &Vec<u32> {
        &self.buffer
    }
}

impl std::convert::AsMut<Vec<u32>> for PixelBuffer {
    fn as_mut(&mut self) -> &mut Vec<u32> {
        &mut self.buffer
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
        self.window.update_with_buffer(self.pixel_buffer.as_ref(), self.width, self.height)?;
        Ok(())
    }

    /// clears the pixel buffer
    pub fn clear(&mut self) {
        self.pixel_buffer.as_mut().iter_mut().for_each(|pixel| *pixel = color::BLACK.as_u32());
    }

    /// checks if window is open
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    /// set a pixel
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixel_buffer[(x, y)] = color.as_u32();
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
