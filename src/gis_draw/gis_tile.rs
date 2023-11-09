use crate::gis_draw::gis_image::GisImage;
use crate::gis_draw::gis_proj::{longitude_to_x, latitude_to_y, GisPoint, GisLine, GisRect};

use image::{Rgb, ImageOutputFormat};

#[allow(dead_code)]
pub struct GisXYZ {
    pub x:u64,
    pub y:u64,
    pub z:u64,
}

#[allow(dead_code)]
pub struct GisTile {
    image: GisImage,
    index: GisXYZ,
    rect: GisRect,
}

#[allow(dead_code)]
impl GisTile {
    pub fn new(index:&GisXYZ) -> GisTile{
        let min = GisPoint{
            x: (255 * index.x) as f64,
            y: (255 * index.y) as f64,
        };

        let max = GisPoint{
            x: (255 * index.x + 255) as f64,
            y: (255 * index.y + 255) as f64,
        };

        GisTile{
            image: GisImage::new(256, 256),
            index: GisXYZ { x: index.x, y: index.y, z: index.z },
            rect: GisRect {min, max},
        }
    }

    pub fn dump(&self, format: ImageOutputFormat) -> Vec<u8> {
        self.image.dump(format)
    }

    pub fn save(&self, path:&str, format: ImageOutputFormat) {
        self.image.save(path, format)
    }

    fn longitude_to_x(&self, x:f64) -> f64{
        longitude_to_x(x, self.index.z as f64)
    }

    fn latitude_to_y(&self, y:f64) -> f64 {
        latitude_to_y(y, self.index.z as f64)
    }

    pub fn draw_border(&mut self, color:Rgb<u8>) {
        self.image.draw_line(&GisPoint{x:0.0, y:0.0}, &GisPoint{x:255.0, y:0.0}, color);
        self.image.draw_line(&GisPoint{x:255.0, y:0.0}, &GisPoint{x:255.0, y:255.0}, color);
        self.image.draw_line(&GisPoint{x:255.0, y:255.0}, &GisPoint{x:0.0, y:255.0}, color);
        self.image.draw_line(&GisPoint{x:0.0, y:255.0}, &GisPoint{x:0.0, y:0.0}, color);
    }

    pub fn draw_line(&mut self, p1:&GisPoint, p2:&GisPoint, color:Rgb<u8>) {
        let p1 = GisPoint{
            x: self.longitude_to_x(p1.x) - self.rect.min.x,
            y: self.latitude_to_y(p1.y) - self.rect.min.y,
        };

        let p2 = GisPoint{
            x: self.longitude_to_x(p2.x) - self.rect.min.x,
            y: self.latitude_to_y(p2.y) - self.rect.min.y,
        };

        let line = GisLine{p1, p2};
        let rect = GisRect{
            min: GisPoint{x:0.0, y:0.0},
            max: GisPoint{x:255.0, y:255.0},
        };

        if rect.cross(&line) {
            self.image.draw_line(&p1, &p2, color);
        }
    }

    pub fn draw_polygon(&mut self, polygon: &[GisPoint], color:Rgb<u8>) {
        if polygon.len() < 2 {
            return;
        }

        self.draw_line(polygon.first().unwrap(), polygon.last().unwrap(), color);

        for i in 1..polygon.len() {
            self.draw_line(&polygon[i-1], &polygon[i], color);
        }
    }

    pub fn draw_polyline(&mut self, polyline: &[GisPoint], color:Rgb<u8>) {
        if polyline.len() < 2 {
            return;
        }

        for i in 1..polyline.len() {
            self.draw_line(&polyline[i-1], &polyline[i], color);
        }
    }

    pub fn intersection(&self, rect:&GisRect) -> bool {
        let min = GisPoint {
            x: self.longitude_to_x(rect.min.x),
            y: self.latitude_to_y(rect.max.y),
        };

        let max = GisPoint {
            x: self.longitude_to_x(rect.max.x),
            y: self.latitude_to_y(rect.min.y),
        };

        let rect = GisRect{min, max};
        self.rect.intersection(&rect)
    }

}

