#![allow(dead_code)]

// modules
mod color;
mod img;
pub mod prelude;

// namespacing
pub use color::Color;
pub use img::Image;
pub use minifb::{Key, MouseButton};
use minifb::{Window, WindowOptions};

/// error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    MiniFb(#[from] minifb::Error),
    #[error(transparent)]
    Image(#[from] image::ImageError),
}

/// result type
pub type Result<T> = std::result::Result<T, Error>;

/// renderable trait
pub trait Renderable {
    /// return the dimensions of the structure
    fn dimensions(&self) -> (usize, usize);
    /// return a slice of row major data to be rendered
    fn data(&self) -> &Vec<Color>;
}

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
    scale: usize,
    window: Window,
}

impl Context {
    /// create a new context
    pub fn new(
        width: usize,
        height: usize,
        title: String,
        scale: Option<usize>,
    ) -> Result<Context> {
        let scale = scale.unwrap_or(1);
        let pixel_buffer = PixelBuffer::new(width, height);
        let mut window = Window::new(&title, width * scale, height * scale, WindowOptions::default())?;
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Ok(Context { pixel_buffer, window, height, width, scale })
    }

    /// render the internal buffer to the screen
    pub fn present(&mut self) -> Result<()> {
        self.window.update_with_buffer(&self.pixel_buffer.as_ref(), self.width, self.height)?;
        Ok(())
    }

    /// clears the pixel buffer
    pub fn clear(&mut self, color: Option<color::Color>) {
        let color = color.unwrap_or(Color::BLACK);
        self.pixel_buffer.as_mut_ref().iter_mut().for_each(|pixel| *pixel = color.as_u32());
    }

    /// checks if window is open
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    /// set a pixel
    pub fn insert_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.pixel_buffer.width && y < self.pixel_buffer.height {
            *self.pixel_buffer.at_mut(x, y) = color.as_u32();
        }
    }

    /// insert a slice into the pixel buffer x/y are offsets w/h are the dimensions of the slice
    /// !! this method can panic if the given buffer dimensions don't match the buffer length
    pub fn insert_slice(&mut self, x: usize, y: usize, w: usize, h: usize, slice: &[Color]) {
        assert!(w * h == slice.len());
        let (ox, oy) = (x, y);

        for x in 0..w {
            for y in 0..h {
                self.insert_pixel(x + ox, y + oy, slice[y * w + x])
            }
        }
    }

    /// render a structure that impl's the renderable trait
    pub fn render<T>(&mut self, x: usize, y: usize, renderable: &T)
    where
        T: Renderable,
    {
        let (w, h) = renderable.dimensions();
        let slice = renderable.data();

        self.insert_slice(x, y, w, h, &slice);
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
