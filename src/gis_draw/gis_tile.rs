use crate::gis_draw::gis_image::GisImage;
use crate::gis_draw::gis_proj::{longitude_to_x, latitude_to_y};
use imageproc::point::Point;
use image::{Rgb, ImageOutputFormat};

pub type GisPoint = Point<f64>;

pub struct GisRect {
    pub min: GisPoint,
    pub max: GisPoint,
}

#[allow(dead_code)]
impl GisRect {
    pub fn contains(&self, p:&GisPoint) -> bool {
        self.min.x <= p.x && self.max.x > p.x && self.min.y <= p.y && self.max.y > p.y
    }

    pub fn intersection(&self, rect:&GisRect) -> bool {
        self.max.x > rect.min.x && self.max.y > rect.min.y && self.min.x < rect.max.x && self.min.y < rect.max.y
    }
}

#[allow(dead_code)]
pub struct GisXYZ {
    pub x:u32,
    pub y:u32,
    pub z:u32,
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
            x: (256 * index.x) as f64,
            y: (256 * index.y) as f64,
        };

        let max = GisPoint{
            x: (256 * index.x + 256) as f64,
            y: (256 * index.y + 256) as f64,
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

    pub fn draw_line(&mut self, p1:&GisPoint, p2:&GisPoint, color:Rgb<u8>) {
        let p1 = GisPoint{
            x: self.longitude_to_x(p1.x) - self.rect.min.x,
            y: self.latitude_to_y(p1.y) - self.rect.min.y,
        };

        let p2 = GisPoint{
            x: self.longitude_to_x(p2.x) - self.rect.min.x,
            y: self.latitude_to_y(p2.y) - self.rect.min.y,
        };
        self.image.draw_line(&p1, &p2, color);
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

