// areas_volumes.rs
pub mod areas_volumes {
    pub enum GeometricalShapes {
        Square,
        Circle,
        Rectangle,
        Triangle,
    }

    pub enum GeometricalVolumes {
        Cube,
        Sphere,
        Cone,
        Pyramid,
        Parallelepiped,
    }

    pub fn square_area(side: usize) -> usize {
        side * side
    }

    pub fn triangle_area(base: usize, height: usize) -> f64 {
        0.5 * (base as f64) * (height as f64)
    }

    pub fn circle_area(radius: usize) -> f64 {
        std::f64::consts::PI * (radius as f64).powi(2)
    }

    pub fn rectangle_area(side_a: usize, side_b: usize) -> usize {
        side_a * side_b
    }

    pub fn cube_volume(side: usize) -> usize {
        side.pow(3)
    }

    pub fn sphere_volume(radius: usize) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * (radius as f64).powi(3)
    }

    pub fn cone_volume(base_radius: usize, height: usize) -> f64 {
        (std::f64::consts::PI * (base_radius as f64).powi(2) * height as f64) / 3.0
    }

    pub fn triangular_pyramid_volume(base_area: f64, height: usize) -> f64 {
        (base_area * height as f64) / 3.0
    }

    pub fn parallelepiped_volume(side_a: usize, side_b: usize, side_c: usize) -> usize {
        side_a * side_b * side_c
    }
}

pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};


// lib.rs or main.rs or wherever your functions are defined
pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let total_area = x * y;
    let object_area = match objects {
        areas_volumes::GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        areas_volumes::GeometricalShapes::Circle => areas_volumes::circle_area(a),
        areas_volumes::GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        areas_volumes::GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
    };

    (object_area * times as f64) <= total_area as f64
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let total_volume = x * y * z;
    let object_volume = match objects {
        areas_volumes::GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        areas_volumes::GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        areas_volumes::GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        areas_volumes::GeometricalVolumes::Pyramid => {
            areas_volumes::triangular_pyramid_volume(a as f64, b)
        }
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            areas_volumes::parallelepiped_volume(a, b, c) as f64
        }
    };

    (object_volume * times as f64) <= total_volume as f64
}
