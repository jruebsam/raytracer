use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
   type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3{
            x: self.x*rhs,
            y: self.y*rhs,
            z: self.z*rhs,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }
}

impl ops::Neg for Vec3 {
    type Output=Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z:  -self.z }
    }
}
    

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn add() { 
        let p1 = Vec3{x : 1., y: 2., z: 3.};
        let p2 = Vec3{x : 1., y: 2., z: 3.};
        let p = p1 + p2;
        assert_eq!(p.x, 2.);
        assert_eq!(p.y, 4.);
        assert_eq!(p.z, 6.);
    }

    #[test]
    fn subtract() { 
        let p1 = Vec3{x : 1., y: 2., z: 3.};
        let p2 = Vec3{x : 1., y: 2., z: 3.};
        let p = p1 - p2;
        assert_eq!(p.x, 0.);
        assert_eq!(p.y, 0.);
        assert_eq!(p.z, 0.);
    }

    #[test]
    fn scalar_mul() { 
        let p1 = Vec3{x : 1., y: 2., z: 3.};
        let p = p1*2.;
        assert_eq!(p.x, 2.);
        assert_eq!(p.y, 4.);
        assert_eq!(p.z, 6.);
    }
    
    #[test]
    fn scalar_product() { 
        let p1 = Vec3{x : 1., y: 2., z: 3.};
        let p2 = Vec3{x : 1., y: 2., z: 3.};
        let p = p1*p2;
        assert_eq!(p, 14.);
    }
    
    #[test]
    fn negation() { 
        let p1 = Vec3{x : 1., y: 2., z: 3.};
        let p = -p1;
        assert_eq!(p.x, -1.);
        assert_eq!(p.y, -2.);
        assert_eq!(p.z, -3.);
    }

}
