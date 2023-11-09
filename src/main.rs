mod gis_net;
mod gis_draw;
mod gis_cache;

use std::thread;

use gis_net::gis_http::Router;
use image::{Rgb, ImageOutputFormat};
use crate::gis_net::gis_http::GisServer;
use crate::gis_draw::gis_tile::{GisTile, GisXYZ, GisPoint, GisRect};
use crate::gis_net::gis_http::http;
use crate::gis_cache::gis_sqlite::GisSqlite;

struct GisRouter {
    sql: GisSqlite,
    shapes: Vec<shapefile::Shape>,
}

impl GisRouter {
    fn new(file:&str) -> GisRouter{
        let mut router = GisRouter{
            sql: GisSqlite::new(file),
            shapes: Vec::new()
        };

        let shapefiles = [
            r"shapefile\Country\CN-boundary-land.shp",
            r"shapefile\Country\CN-boundary-sea.shp",
            r"shapefile\Province\province_region.shp",
            r"shapefile\City\CN_city.shp",
            r"shapefile\Train\rai_4m.shp",
            r"shapefile\Road\roa_4m.shp",
            r"shapefile\River\hyd1_4l.shp",
            r"shapefile\River\hyd2_4l.shp",
        ];

        for file in shapefiles {
            let mut reader = shapefile::Reader::from_path(file).unwrap();
            for shape_record in reader.iter_shapes_and_records() {
                let (shape, _) = shape_record.unwrap();
                router.shapes.push(shape);
            }
        }

        router
    }

    fn init(&self) {
        if !self.sql.init() {    
            for z in 0..28 {
                for x in 0..2u32.pow(z) {
                    for y in 0..2u32.pow(z) {
                        let mut tile = self.sql.get(x, y, z);
                        if tile.len() == 0 {
                            tile = self.draw_tile(&GisXYZ { x, y, z });
                            self.sql.set(x, y, z, &tile);
                        }
                    } 
                }
            }
        }
    }

    fn draw_tile(&self, index:&GisXYZ) -> Vec<u8> {
        let mut tile = GisTile::new(&index);
        for shape in &self.shapes {
            match shape {
                shapefile::Shape::Polygon(polygon) => {
                    let bbox = polygon.bbox();
                    let min = GisPoint{x:bbox.min.x, y:bbox.min.y};
                    let max = GisPoint{x:bbox.max.x, y:bbox.max.y};
                    let rect = GisRect{min, max};
                    
                    if !tile.intersection(&rect) {
                        continue;
                    }

                    for ring in polygon.rings() {
                        let mut points = Vec::<GisPoint>::with_capacity(ring.len()) ;
                        for p in ring.points() {
                            points.push(GisPoint{x:p.x,y:p.y,});
                        }
                        tile.draw_polygon(&points, Rgb([255, 255, 225]));
                    }
                },
                shapefile::Shape::Polyline(polyline) => {
                    let bbox = polyline.bbox();
                    let min = GisPoint{x:bbox.min.x, y:bbox.min.y};
                    let max = GisPoint{x:bbox.max.x, y:bbox.max.y};
                    let rect = GisRect{min, max};
                    
                    if !tile.intersection(&rect) {
                        continue;
                    }

                    for part in polyline.parts() {
                        let mut points = Vec::<GisPoint>::with_capacity(part.len()) ;
                        for p in part {
                            points.push(GisPoint{x:p.x,y:p.y,});
                        }
                        tile.draw_polyline(&points, Rgb([255, 255, 225]));
                    }
                }
                _ => {
                    println!("{}", shape);
                }
            }
        } 
        tile.dump(ImageOutputFormat::Png)
    }
}

impl Router for GisRouter {
    fn route(&self, path:&str) -> http::Response<Vec<u8>> {
        if path == "/" {
            http::Response::builder()
                .header("Content-Type", "text/html")
                .status(http::StatusCode::OK)
                .body("<h1>GIS Server</h1>".as_bytes().to_vec())
                .unwrap()
        }
        else if let Some(xyz) = gis_parse_pos(path) {
            let mut tile = self.sql.get(xyz.x, xyz.y, xyz.z);
            if tile.len() == 0 {
                tile = self.draw_tile(&xyz);
                self.sql.set(xyz.x, xyz.y, xyz.z, &tile);
            }
            http::Response::builder()
                .header("Content-Type", "image/png")
                .status(http::StatusCode::OK)
                .body(tile)
                .unwrap()
        } else {
            http::Response::builder()
                .header("Content-Type", "text/html")
                .status(http::StatusCode::NOT_FOUND)
                .body("<h1>404 Not Found</h1>".as_bytes().to_vec())
                .unwrap()
        }
    }
}

fn main() {
    let addr = "localhost:1995";
    let router = GisRouter::new("tiles.sqlite");
    router.init();
    println!("Start in {}", addr);
    GisServer::start(addr, &router);
}

fn gis_parse_pos(path:&str) -> Option<GisXYZ> {
    let exp = regex::Regex::new(r"/maps/(\d+)/(\d+)/(\d+)\.png").unwrap();
    if let Some(caps) = exp.captures(path) {
        Some(GisXYZ{x:caps[2].parse::<u32>().unwrap(), 
                y:caps[3].parse::<u32>().unwrap(), 
                z:caps[1].parse::<u32>().unwrap()})
    } else {
        None
    }
}
