use std::f64::consts::PI;

pub fn longitude_to_x(v:f64, zoom:f64) -> f64 {
    256.0 / (2.0*PI) * (v.to_radians() + PI) * 2.0f64.powf(zoom) 
}

pub fn latitude_to_y(v: f64, zoom:f64) -> f64 {
    256.0 / (2.0*PI)* (PI - (PI/4.0 + v.to_radians()/2.0).tan().ln()) * 2.0f64.powf(zoom)
}