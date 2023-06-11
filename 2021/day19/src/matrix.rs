use crate::vector::Vector;


#[derive(Eq,PartialEq,Copy,Clone,Debug,Hash)]
pub struct Matrix {
    pub entries: (
        (isize,isize,isize),
        (isize,isize,isize),
        (isize,isize,isize),
    )
}


pub struct MatrixIterator<'a> {
    offset: usize,
    matrix: &'a Matrix,
}


impl<'a> Iterator for MatrixIterator<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.offset;
        if index >= 9 {
            None
        } else {
            self.offset += 1;
            let row = match index / 3 {
                0 => self.matrix.entries.0,
                1 => self.matrix.entries.1,
                _ => self.matrix.entries.2,
            };
            Some(match index % 3 {
                0 => row.0,
                1 => row.1,
                _ => row.2,
            })
        }
    }
}


impl<'a> Matrix {

    pub fn rotation_walk() -> RotationWalk {
        RotationWalk::default()
    }

    #[inline]
    fn roll() -> Self {
        Matrix{entries: (
            ( 1, 0, 0),
            ( 0, 0,-1),
            ( 0, 1, 0),
        )}
    }

    #[inline]
    fn turn() -> Self {
        Matrix{entries: (
            ( 0, 1, 0),
            (-1, 0, 0),
            ( 0, 0, 1),
        )}
    }

}


impl Default for Matrix {
    fn default() -> Self {
        Matrix{entries: ((1,0,0),(0,1,0),(0,0,1))}
    }
}


impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (a, b, c) = self.entries;
        f.write_fmt(format_args!(
            "[{:>2} {:>2} {:>2}]\n[{:>2} {:>2} {:>2}]\n[{:>2} {:>2} {:>2}]",
            a.0, a.1, a.2,
            b.0, b.1, b.2,
            c.0, c.1, c.2
        ))        
    }
}


impl std::ops::Mul<Vector> for Matrix {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        let (a, b, c) = self.entries;
        let Vector{x, y, z} = rhs;
        Vector{
            x: a.0 * x + a.1 * y + a.2 * z,
            y: b.0 * x + b.1 * y + b.2 * z,
            z: c.0 * x + c.1 * y + c.2 * z,
        }
    }
}


impl std::ops::Mul for Matrix {
    type Output = Self;
    fn mul(self, them: Self) -> Self::Output {
        let (a0, a1, a2) = self.entries;
        let (b0, b1, b2) = them.entries;
        Matrix{entries: ((
            a0.0 * b0.0 + a0.1 * b1.0 + a0.2 * b2.0,
            a0.0 * b0.1 + a0.1 * b1.1 + a0.2 * b2.1,
            a0.0 * b0.2 + a0.1 * b1.2 + a0.2 * b2.2,
        ), (
            a1.0 * b0.0 + a1.1 * b1.0 + a1.2 * b2.0,
            a1.0 * b0.1 + a1.1 * b1.1 + a1.2 * b2.1,
            a1.0 * b0.2 + a1.1 * b1.2 + a1.2 * b2.2,
        ), (
            a2.0 * b0.0 + a2.1 * b1.0 + a2.2 * b2.0,
            a2.0 * b0.1 + a2.1 * b1.1 + a2.2 * b2.1,
            a2.0 * b0.2 + a2.1 * b1.2 + a2.2 * b2.2,
        ))}
    }
}

#[derive(Default)]
pub struct RotationWalk {
    _swap: bool,
    _step: usize,
}


impl Iterator for RotationWalk {
    type Item = Matrix;

    fn next(&mut self) -> Option<Self::Item> {
        if self._step >= 12 {
            if self._swap {
                None
            } else {
                self._swap = true;
                self._step = 1;
                Some (
                    Matrix::roll() *
                    Matrix::roll() *
                    Matrix::turn() *
                    Matrix::roll() 
                )
            }
        } else {
            let step = self._step;
            self._step += 1;
            Some(if step % 4 == 0 {
                Matrix::roll()
            } else {
                Matrix::turn()
            })
        }
    }
}


#[test]
fn there_are_24_rotations() {
    use std::collections::HashSet;
    let mut h: HashSet<Matrix> = HashSet::new();
    let mut t = Matrix::default();
    for r in Matrix::rotation_walk() {
        t = r * t;
        h.insert(t);
    }
    assert_eq!(h.len(), 24);
}
