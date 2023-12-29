use crate::{common::*, vec3::Vec3, ray::Ray};
use itertools::iproduct;

pub struct Camera { 
    pub focal_length: f64,
    pub resolution: Shape2D<usize>,
    pub viewport: Shape2D<f64>,

    center: Vec3, 
    u: Vec3,
    v: Vec3,
    du: Vec3,
    dv: Vec3,
}
impl Camera {
    pub fn new(focal_length: f64, resolution: Shape2D<usize>, viewport: Shape2D<f64>) -> Self { 
        let u =  Vec3 { x: viewport.width, y: 0., z: 0.};
        let v =  Vec3 { x: 0., y:-viewport.height, z: 0.};
        let du = u / resolution.width as f64;
        let dv = v / resolution.height as f64;

        Self { focal_length, viewport, resolution, center: Vec3::default(), u, v, du, dv } 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_properties() { 
        let aspect_ratio  = 16./9.;
        let focal_length = 2.;

        let width = 400;
        let height = (width as f64/aspect_ratio) as usize;
        
        let resolution = Shape2D{width, height};
        let viewport = Shape2D{width: 2.0, height: 2.0*(resolution.width as f64/ resolution.height as f64)};

        let cam = Camera::new(focal_length, resolution.clone(), viewport.clone());
        assert_eq!(cam.focal_length, focal_length);
        assert_eq!(cam.viewport, viewport);
        assert_eq!(cam.resolution, resolution);
        assert_eq!(cam.center, Vec3::default());
    }
}