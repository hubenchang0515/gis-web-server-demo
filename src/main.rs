mod gis_net;
mod gis_draw;

use image::{Rgb, ImageOutputFormat};
use crate::gis_net::gis_http::GisServer;
use crate::gis_draw::gis_tile::{GisTile, GisXYZ, GisPoint, GisRect};
use crate::gis_net::gis_http::http;

fn main() {
    let server = GisServer::new("localhost:1995", router);
    server.start();
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

fn router(path:&str) -> http::Response<Vec<u8>> {
    if path == "/" {
        http::Response::builder()
            .header("Content-Type", "text/html")
            .status(http::StatusCode::OK)
            .body("<h1>GIS Server</h1>".as_bytes().to_vec())
            .unwrap()
    }
    else if let Some(xyz) = gis_parse_pos(path) {
        http::Response::builder()
            .header("Content-Type", "image/png")
            .status(http::StatusCode::OK)
            .body(draw_tile(xyz))
            .unwrap()
    } else {
        http::Response::builder()
            .header("Content-Type", "text/html")
            .status(http::StatusCode::NOT_FOUND)
            .body("<h1>404 Not Found</h1>".as_bytes().to_vec())
            .unwrap()
    }
}

fn draw_tile(index:GisXYZ) -> Vec<u8> {
    let mut tile = GisTile::new(&index);

    if index.z >= 4 {
        draw_city_boundary(&mut tile);
    }

    draw_country_boundary(&mut tile);

    if index.z >= 2{
        draw_province_boundary(&mut tile);
    }

    if index.z >= 6 {
        draw_train(&mut tile);
        draw_road(&mut tile);
        draw_river(&mut tile);
    }
    tile.dump(ImageOutputFormat::Png)
}

fn draw_shape(tile:&mut GisTile, files:&[&str], color:Rgb<u8>) {
    for file in files {
        let mut reader = shapefile::Reader::from_path(file).unwrap();
        for shape_record in reader.iter_shapes_and_records() {
            let (shape, _) = shape_record.unwrap();
            
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
                        tile.draw_polygon(&points, color);
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
                        tile.draw_polyline(&points, color);
                    }
                }
                _ => {
                    println!("{}", shape);
                }
            }
        }
    }
}

fn draw_country_boundary(tile:&mut GisTile) {
    let files = [
        r"shapefile\Country\CN-boundary-land.shp",
        r"shapefile\Country\CN-boundary-sea.shp",
    ];

    draw_shape(tile, &files, Rgb([255, 255, 225]));
}

fn draw_province_boundary(tile:&mut GisTile) {
    let files = [
        r"shapefile\Province\province_region.shp",
    ];

    draw_shape(tile, &files, Rgb([255, 204, 255]));
}

fn draw_city_boundary(tile:&mut GisTile) {
    let files = [
        r"shapefile\City\CN_city.shp",
    ];

    draw_shape(tile, &files, Rgb([204, 255, 204]));
}

fn draw_train(tile:&mut GisTile) {
    let files = [
        r"shapefile\Train\rai_4m.shp",
    ];

    draw_shape(tile, &files, Rgb([238, 121, 66]));
}

fn draw_road(tile:&mut GisTile) {
    let files = [
        r"shapefile\Road\roa_4m.shp",
    ];

    draw_shape(tile, &files, Rgb([230, 230, 250]));
}

fn draw_river(tile:&mut GisTile) {
    let files = [
        r"shapefile\River\hyd1_4l.shp",
        r"shapefile\River\hyd2_4l.shp",
    ];

    draw_shape(tile, &files, Rgb([0, 191, 255]));
}