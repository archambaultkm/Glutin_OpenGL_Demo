use cgmath::Vector3;
use crate::cube::Cube;

pub struct World {
    pub objects : Vec<Cube>
}

impl World {
    pub fn new() -> Self {
        let cube = Cube::new(Vector3::new(0.0, 0.0, 0.0));
        let mut objects = Vec::new();
        objects.push(cube);

        World {
            objects
        }
    }
}