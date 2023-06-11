use crate::matrix::Matrix;


#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}


impl Vector {
    pub fn abs(&self) -> isize {
        return self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn rotates_into(&self, other: &Vector) -> Option<Matrix> {
        let mut a = Matrix::default();
        let w = *self;
        for r in Matrix::rotation_walk() {
            a = r * a;
            if *other == a * w {
                return Some(a);
            }
        }
        None
    }
}


impl From<(isize,isize,isize)> for Vector {
    fn from(value: (isize,isize,isize)) -> Self {
        let (x,y,z) = value;
        Vector{x,y,z}
    }
}


impl std::ops::Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}


impl std::ops::Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
