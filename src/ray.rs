use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray { 
    fn at(&self, time: f64) -> Vec3 { 
        self.origin + self.direction*time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_at_time() { 
        let ray = Ray {
            origin: Vec3 { x: 1., y: 1., z: 1. },
            direction: Vec3 { x: 1.0, y: 0.0, z: 0.0},
        };
        
        let endtime = 1.;
        let endpos = Vec3 { x: 2.0, y: 1.0, z: 1.0};
        assert_eq!(ray.at(endtime), endpos);
    } 
    
}