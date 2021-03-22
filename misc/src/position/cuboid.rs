use crate::prelude::Vec3D;

pub struct Cuboid {
    pub start: Vec3D<i32>,
    pub end: Vec3D<i32>,
}

impl Cuboid {
    pub fn new(start: Vec3D<i32>, end: Vec3D<i32>) -> Self {
        Self {
            start: Vec3D {
                x: start.x.min(end.x),
                y: start.y.min(end.y),
                z: start.z.min(end.z),
            },
            end: Vec3D {
                x: start.x.max(end.x),
                y: start.y.max(end.y),
                z: start.z.max(end.z),
            },
        }
    }

    pub fn volume(&self) -> i32 {
        (self.end.x - self.start.x + 1)
            * (self.end.y - self.start.y + 1)
            * (self.end.z - self.start.z + 1)
    }
}

impl<'a> IntoIterator for &'a Cuboid {
    type Item = Vec3D<i32>;

    type IntoIter = CuboidIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CuboidIter::new(&self.start, &self.end)
    }
}

pub struct CuboidIter<'a> {
    start: &'a Vec3D<i32>,
    current: Vec3D<i32>,
    end: &'a Vec3D<i32>,
}

impl<'a> CuboidIter<'a> {
    fn new(start: &'a Vec3D<i32>, end: &'a Vec3D<i32>) -> Self {
        Self {
            start,
            current: start.clone(),
            end,
        }
    }
}

impl<'a> Iterator for CuboidIter<'a> {
    type Item = Vec3D<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x <= self.end.x
            && self.current.y <= self.end.y
            && self.current.z <= self.end.z
        {
            let current = self.current.clone();

            self.current.x += 1;
            if self.current.x > self.end.x {
                self.current.x = self.start.x;
                self.current.z += 1;
                if self.current.z > self.end.z {
                    self.current.z = self.start.z;
                    self.current.y += 1;
                }
            }

            Some(current)
        } else {
            None
        }
    }
}
