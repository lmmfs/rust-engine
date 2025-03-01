use anyhow::{Context, Result};
use std::ops::Range;
use super::color::Color;
use pixels::{Pixels, SurfaceTexture};

pub trait RenderSurface {
    //get width of the surface
    fn width(&self) -> u32;

    //get height of the surface
    fn height(&self) -> u32;

    //clears the screen with a specific color
    fn clear_screen(&mut self, color: &Color);
    
    //blits the current frame
    fn blit(&mut self) -> Result<()>;

    //position is in surface bounds
    fn in_bounds(&self, x: i64, y: i64) -> Option<(u32, u32)>;

    //convert a position to surface pos
    fn physical_pos_to_surface_pos(&self, x: f64, y: f64) -> Option<(u32, u32)>;

    // apply a color to a range of pixels in the surface
    fn set_range(&mut self, range: Range<usize>, color: &Color);

    // draw a filled rectangle
    fn filled_rect(&mut self, sx: u32, sy: u32, width: u32, height: u32, color: &Color);
    
}

pub struct PixelsSurface {
    pixels: Pixels,
}

impl PixelsSurface {
    // constructor
    pub fn new(pixels: Pixels) -> Self {
        Self { pixels }
    }

    //get the color in a position
    pub fn get(&self, x: u32, y: u32) -> Color {
        let i = ((y * self.width() + x) * 4) as usize;
        let buf = self.pixels.frame();
        Color::from_bytes(&buf[i..i + 4])
    }

    //set a color in a position
    pub fn set(&mut self, x: u32, y: u32, color: &Color) {
        let i = ((y * self.width() + x) * 4) as usize;
        let buf = self.pixels.frame_mut();
        buf[i..i + 4].copy_from_slice(color.as_bytes());
    }
}

impl RenderSurface for PixelsSurface {
    fn width(&self) -> u32 {
        self.pixels.texture().width()
    }

    fn height(&self) -> u32 {
        self.pixels.texture().height()
    }

    fn clear_screen(&mut self, color: &Color) {
        self.set_range(0..(self.height() * self.width()) as usize, &color);
    }

    fn blit(&mut self) -> Result<()> {
        self.pixels
            .render()
            .context("letting pixels lib blit to screen")?;
        Ok(())
    }

    fn in_bounds(&self, x: i64, y: i64) -> Option<(u32, u32)> {
        if x < 0 || x >= self.width() as i64 || y < 0 || y >= self.height() as i64 {
            None
        } else {
            Some((x as u32, y as u32))
        }
    }

    fn physical_pos_to_surface_pos(&self, x: f64, y: f64) -> Option<(u32, u32)> {
        if let Ok((x, y)) = self.pixels.window_pos_to_pixel((x as f32, y as f32)) {
            Some((x as u32, y as u32))
        } else {
            None
        }
    }

    fn set_range(&mut self, range: Range<usize>, color: &Color) {
        let byte_range = range.start * 4..range.end * 4;
        let buf = self.pixels.frame_mut();
        for chunk in buf[byte_range].chunks_exact_mut(4) {
            chunk.copy_from_slice(color.as_bytes());
        }
    }

    fn filled_rect(&mut self, sx: u32, sy: u32, width: u32, height: u32, color: &Color) {
        for y in sy..sy + height {
            self.set_range(
                (y * self.width() + sx) as usize..(y * self.width() + sx + width) as usize,
                color,
            );
        }
    }
}