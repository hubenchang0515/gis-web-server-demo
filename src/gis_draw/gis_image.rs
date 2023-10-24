use std::fs::File;
use std::io::Cursor;
use imageproc::point::Point;
use image::{Rgb, RgbImage, ImageOutputFormat};

#[allow(dead_code)]
pub struct GisImage {
    img:  RgbImage,
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl GisImage {
    pub fn new(width:u32, height:u32) -> GisImage {
        GisImage{
            img:  RgbImage::new(width, height),
            width,
            height,
        }
    }

    pub fn save(&self, path:&str, format: ImageOutputFormat) {
        let mut fp = File::create(path).unwrap();
        self.img.write_to(&mut fp, format).unwrap();
    }

    pub fn dump(&self, format: ImageOutputFormat) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        self.img.write_to(&mut Cursor::new(&mut bytes), format).unwrap();
        bytes
    }

    pub fn draw_pixel(&mut self, x:u32, y:u32, color:Rgb<u8>) {
        self.img.put_pixel(x, y, color);
    }

    pub fn fill(&mut self, color:Rgb<u8>) {
        for y in 0u32..self.height {
            for x in 0u32..self.width {
                self.draw_pixel(x, y, color);
            }
        }
    }

    pub fn draw_line(&mut self, p1:&Point<f64>, p2:&Point<f64>, color:Rgb<u8>) {
        imageproc::drawing::draw_line_segment_mut(
            &mut self.img,
            (p1.x as f32, p1.y as f32),
            (p2.x as f32, p2.y as f32),
            color
        );
    }

    pub fn draw_polygon(&mut self, polygon: &[Point<f64>], color:Rgb<u8>) {
        if polygon.len() < 2 {
            return;
        }

        self.draw_line(polygon.first().unwrap(), polygon.last().unwrap(), color);

        for i in 1..polygon.len() {
            self.draw_line(&polygon[i-1], &polygon[i], color);
        }
    }

    // pub fn 
}