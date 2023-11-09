use std::f64::consts::PI;
use imageproc::point::Point;

pub fn longitude_to_x(v:f64, zoom:f64) -> f64 {
    256.0 / (2.0*PI) * (v.to_radians() + PI) * 2.0f64.powf(zoom) 
}

pub fn latitude_to_y(v: f64, zoom:f64) -> f64 {
    256.0 / (2.0*PI)* (PI - (PI/4.0 + v.to_radians()/2.0).tan().ln()) * 2.0f64.powf(zoom)
}

pub type GisPoint = Point<f64>;

#[allow(dead_code)]
pub struct GisLine {
    pub p1: GisPoint,
    pub p2: GisPoint,
}

#[allow(dead_code)]
impl GisLine {
    pub fn cross(&self, line:&GisLine) -> bool {
        // AB
        let x1 = self.p1.x - self.p2.x;
        let y1 = self.p1.y - self.p2.y;
        
        // AC
        let x2 = self.p1.x - line.p1.x;
        let y2 = self.p1.y - line.p1.y;
        
        // AD
        let x3 = self.p1.x - line.p2.x;
        let y3 = self.p1.y - line.p2.y;

        // CD
        let x4 = line.p1.x - line.p2.x;
        let y4 = line.p1.y - line.p2.y;

        // CB
        let x5 = line.p1.x - self.p2.x;
        let y5 = line.p1.y - self.p2.y;

        // CA
        let x6 = -x2;
        let y6 = -y2;

        (x1*y2 - x2*y1) * (x1*y3 - x3*y1) <= 0.0 && // (AB x AC) * (AB x AD) < 0
        (x4*y6 - x6*y4) * (x4*y5 - x5*y4) <= 0.0    // (CD x CA) * (CD x CB) < 0
    }
}

pub struct GisRect {
    pub min: GisPoint,
    pub max: GisPoint,
}

#[allow(dead_code)]
impl GisRect {
    pub fn contains(&self, p:&GisPoint) -> bool {
        self.min.x <= p.x && self.max.x >= p.x && self.min.y <= p.y && self.max.y >= p.y
    }

    pub fn cross(&self, line:&GisLine) -> bool {
        let diagnal1 = GisLine{
            p1: self.min,
            p2: self.max,
        };

        let diagnal2 = GisLine {
            p1: GisPoint {
                x: self.min.x,
                y: self.max.y,
            },

            p2: GisPoint {
                x: self.max.x,
                y: self.min.y,
            },
        };

        self.contains(&line.p1) || self.contains(&line.p2) || line.cross(&diagnal1) || line.cross(&diagnal2)
    }

    pub fn intersection(&self, rect:&GisRect) -> bool {
        self.max.x >= rect.min.x && self.max.y >= rect.min.y && self.min.x <= rect.max.x && self.min.y <= rect.max.y
    }
}