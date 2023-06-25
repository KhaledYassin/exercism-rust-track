use num::Num;
use std::cmp::Ordering;
pub struct Triangle<T: Num + PartialOrd + PartialEq + Copy> {
    x: T,
    y: T,
    z: T,
}

impl<T> Triangle<T>
where
    T: Num + PartialOrd + PartialEq + Copy,
{
    pub fn build(mut sides: [T; 3]) -> Option<Triangle<T>> {
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        if sides
            .iter()
            .all(|s| s.partial_cmp(&T::zero()) == Some(Ordering::Greater))
            && sides[0] + sides[1] >= sides[2]
        {
            Some(Triangle {
                x: sides[0],
                y: sides[1],
                z: sides[2],
            })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.x == self.y && self.y == self.z && self.z == self.x
    }

    pub fn is_scalene(&self) -> bool {
        self.x != self.y && self.y != self.z && self.z != self.x
    }

    pub fn is_isosceles(&self) -> bool {
        self.x == self.y || self.y == self.z || self.z == self.x
    }
}
