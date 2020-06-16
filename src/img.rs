use crate::{Color, Result};
use std::path::PathBuf;

/// an image
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Image {
    /// load an image from a file
    pub fn load(path: PathBuf) -> Result<Image> {
        let (width, height) = image::image_dimensions(&path)?;
        let (width, height) = (width as usize, height as usize);
        let data: Vec<Color> = image::open(path)?
            .into_rgb()
            .pixels()
            .map(|pixel| Color::new(pixel[0], pixel[1], pixel[2]))
            .collect();

        Ok(Image { width, height, data })
    }
}

impl crate::Renderable for Image {
    fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn data(&self) -> &Vec<Color> {
        &self.data
    }
}
